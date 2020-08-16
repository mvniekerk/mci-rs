use crate::card_version::CardVersion::{Mmc, Unknown};
use crate::card_version::MmcVersion;
use crate::command_arguments::mmc::{Access, BusWidth, Cmd6};
use crate::commands::{
    MMC_CMD3_SET_RELATIVE_ADDR, MMC_CMD6_SWITCH, MMC_CMD8_SEND_EXT_CSD, MMC_MCI_CMD1_SEND_OP_COND,
    SDMMC_CMD16_SET_BLOCKLEN, SDMMC_CMD2_ALL_SEND_CID, SDMMC_CMD7_SELECT_CARD_CMD,
    SDMMC_MCI_CMD0_GO_IDLE_STATE,
};
use crate::mci::Mci;
use crate::mci_card::{ocr_voltage_support, MciCard, MMC_TRANS_MULTIPLIERS, SD_MMC_TRANS_UNITS};
use crate::mode_index::ModeIndex;
use crate::registers::ocr::{AccessMode, OcrRegister};
use crate::registers::sd::card_status::CardStatusRegister;
use bit_field::BitField;
use embedded_hal::digital::v2::InputPin;
use crate::functions::sdmmc_functions::SD_MMC_BLOCK_SIZE;

pub const EXT_CSD_CARD_TYPE_INDEX: u32 = 196;
pub const EXT_CSD_SEC_COUNT_INDEX: u32 = 212;
pub const EXT_CSD_BSIZE: u32 = 512;

impl<MCI, WP, DETECT> MciCard<MCI, WP, DETECT>
where
    MCI: Mci,
    WP: InputPin,     // Write protect pin
    DETECT: InputPin, // Card detect pin
{
    /// Sends operation condition command and read OCR (MCI only)
    pub fn mmc_mci_send_operation_condition(&mut self) -> Result<(), ()> {
        let mut ocr = ocr_voltage_support();
        ocr.set_access_mode(AccessMode::Sector);
        // Timeout is 1s = 400KHz / ((6+6)*8) cycles = 4200 retries. TODO maybe a delay check?
        for i in (0..4200).rev() {
            if i == 0 {
                return Err(()); // TODO proper error
            }
            self.mci
                .send_command(MMC_MCI_CMD1_SEND_OP_COND.into(), ocr.val)?;
            let response = self.mci.get_response();
            let response = OcrRegister { val: response };
            if response.card_powered_up_status() {
                if response.access_mode() == AccessMode::Sector {
                    self.card_type.set_high_capacity(true);
                }
                break;
            }
        }
        Ok(())
    }

    /// CMD6 for MMC - Switches the bus width mode
    pub fn mmc_cmd6_set_bus_width(&mut self, bus_width: &BusWidth) -> Result<bool, ()> {
        let mut arg = Cmd6::default();
        arg.set_access(Access::SetBits)
            .set_bus_width(&bus_width)
            .set_mode_index(ModeIndex::BusWidth);
        self.mci.send_command(MMC_CMD6_SWITCH.into(), arg.val)?;
        let ret = CardStatusRegister {
            val: self.mci.get_response(),
        };
        if ret.switch_error() {
            // Not supported, not a protocol error
            return Ok(false);
        }
        self.bus_width = bus_width.clone();
        Ok(true)
    }

    /// CMD6 for MMC - Switches in high speed mode
    /// self.high_speed is updated
    /// self.clock is updated
    pub fn mmc_cmd6_set_high_speed(&mut self) -> Result<bool, ()> {
        let mut arg = Cmd6::default();
        arg.set_access(Access::WriteByte)
            .set_mode_index(ModeIndex::HsTimingIndex)
            .set_hs_timing_enable(true);
        self.mci.send_command(MMC_CMD6_SWITCH.into(), arg.val)?;
        let ret = CardStatusRegister {
            val: self.mci.get_response(),
        };
        if ret.switch_error() {
            // Not supported, not a protocol error
            return Ok(false);
        }
        self.high_speed = true;
        self.clock = 52_000_000u32;
        Ok(true)
    }

    /// CMD8 - The card sends its EXT_CSD as a block of data
    /// Returns whether high speed can be handled by this
    /// self.capacity is updated
    pub fn mmc_cmd8_high_speed_capable_and_update_capacity(&mut self) -> Result<bool, ()> {
        self.mci
            .adtc_start(MMC_CMD8_SEND_EXT_CSD.into(), 0, 512, 1, false)?;

        let mut index = 0u32;
        let mut read = (0u32, 0u8);
        // Read in bytes (4 at a time) and not to a buffer to "fast forward" to the card type
        while index < ((EXT_CSD_CARD_TYPE_INDEX + 4) / 4) {
            read = self.mci.read_word()?;
            index += 1;
        }
        let high_speed_capable =
            (read.0 >> ((EXT_CSD_CARD_TYPE_INDEX % 4) * 8)).get_bits(0..2) == 0x2; // 52MHz = 0x2, 26MHz = 0x1

        if self.csd.card_size() == 0xFFF {
            // For high capacity SD/MMC card, memory capacity = sec_count * 512 bytes
            while index < (EXT_CSD_SEC_COUNT_INDEX + 4) / 4 {
                read = self.mci.read_word()?;
                index += 1;
            }
            self.capacity = read.0
        }
        // Forward to the end
        while index < EXT_CSD_BSIZE / 4 {
            self.mci.read_word()?;
            index += 1;
        }
        Ok(high_speed_capable)
    }

    /// Decode CSD for MMC
    /// Updates self.version, self.clock, self.capacity
    pub fn mmc_decode_csd(&mut self) -> Result<(), ()> {
        self.version = match self.csd.mmc_csd_spec_version() {
            0 => Mmc(MmcVersion::Mmc1d2),
            1 => Mmc(MmcVersion::Mmc1d4),
            2 => Mmc(MmcVersion::Mmc2d2),
            3 => Mmc(MmcVersion::SdMmc3d0),
            4 => Mmc(MmcVersion::Mmc4d0),
            _ => Unknown,
        };

        // 	Get MMC memory max transfer speed in Hz
        let trans_speed = self.csd.transmission_speed();
        let unit = SD_MMC_TRANS_UNITS[(trans_speed & 0x7) as usize];
        let mult = MMC_TRANS_MULTIPLIERS[((trans_speed >> 3) & 0xF) as usize];
        self.clock = unit * mult * 1000;

        // 	 Get card capacity.
        // 	 ----------------------------------------------------
        // 	 For normal SD/MMC card:
        // 	 memory capacity = BLOCKNR * BLOCK_LEN
        // 	 Where
        // 	 BLOCKNR = (C_SIZE+1) * MULT
        // 	 MULT = 2 ^ (C_SIZE_MULT+2)       (C_SIZE_MULT < 8)
        // 	 BLOCK_LEN = 2 ^ READ_BL_LEN      (READ_BL_LEN < 12)
        // 	 ----------------------------------------------------
        // 	 For high capacity SD/MMC card:
        // 	 memory capacity = SEC_COUNT * 512 byte

        if self.csd.card_size() != 0xFFF {
            let block_nr = ((self.csd.card_size() as u32) + 1)
                * ((self.csd.card_size_multiplier() as u32) + 2);
            self.capacity = block_nr * (1 << self.csd.read_bl_length() as u32) / 1024;
        }
        Ok(())
    }

    /// Initialize the MMC card in MCI mode
    /// This function runs the initialization procedure and the identification process, then it
    /// sets the SD/MMC card in transfer state.
    /// At last, it will enable maximum bus width and transfer speed.
    pub fn sd_mmc_mci_install_mmc(&mut self) -> Result<(), ()> {
        // CMD0 - Reset all cards to idle state.
        self.mci
            .send_command(SDMMC_MCI_CMD0_GO_IDLE_STATE.into(), 0)?;
        self.mmc_mci_send_operation_condition()?;

        // Put the card in Identify Mode
        // Note: The CID is not used
        self.mci.send_command(SDMMC_CMD2_ALL_SEND_CID.into(), 0)?; // TODO Proper error

        //Assign relative address to the card
        self.rca = 1;
        self.mci
            .send_command(MMC_CMD3_SET_RELATIVE_ADDR.into(), (self.rca as u32) << 16)?;

        // Get the card specific data
        self.sd_mmc_cmd9_mci()?;
        self.mmc_decode_csd()?;

        // Select the card and put it into Transfer mode
        self.mci
            .send_command(SDMMC_CMD7_SELECT_CARD_CMD.into(), (self.rca as u32) << 16)?;

        let version: usize = self.version.into();
        if version >= MmcVersion::Mmc4d0 as usize {
            // For MMC 4.0 Higher version
            // Get EXT_CSD
            let authorize_high_speed = self.mmc_cmd8_high_speed_capable_and_update_capacity()?;
            if BusWidth::_4BIT <= self.mci.get_bus_width(self.slot)? {
                // Enable more bus width
                let bus_width = self.bus_width;
                self.mmc_cmd6_set_bus_width(&bus_width)?; // TODO proper error
                self.sd_mmc_select_this_device_on_mci_and_configure_mci()?; // TODO proper error
            }
            if self.mci.is_high_speed_capable()? && authorize_high_speed {
                // TODO proper error
                self.mmc_cmd6_set_high_speed()?; // TODO proper error
                self.sd_mmc_select_this_device_on_mci_and_configure_mci()?; // TODO proper error
            }
        } else {
            self.sd_mmc_select_this_device_on_mci_and_configure_mci()?; // TODO proper error
        }
        for _ in 0..10 {
            // Retry is a workaround for no compliance card (Atmel Internal ref. MMC19)
            // These cards seem not ready immediately after the end of busy of mmc_cmd6_set_high_speed
            if self
                .mci
                .send_command(SDMMC_CMD16_SET_BLOCKLEN.into(), SD_MMC_BLOCK_SIZE)
                .is_ok()
            {
                return Ok(());
            }
        }
        Err(()) // TODO proper timeout error
    }
}
