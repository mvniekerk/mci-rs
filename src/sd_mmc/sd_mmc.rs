use crate::sd_mmc::mci::Mci;
use crate::sd_mmc::card_state::CardState;
use crate::sd_mmc::card_type::CardType;
use crate::sd_mmc::card_version::{CardVersion, SdCardVersion};
use crate::sd_mmc::sd::sd_bus_width::SdBusWidth;
use crate::sd_mmc::registers::csd::{CsdRegister, SdCsdStructureVersion};
use atsamd_hal::hal::digital::v2::InputPin;
use crate::sd_mmc::commands::{SD_MCI_ACMD41_SD_SEND_OP_COND, SDMMC_CMD55_APP_CMD, SD_CMD6_SWITCH_FUNC, Command, SD_CMD8_SEND_IF_COND, SDMMC_MCI_CMD9_SEND_CSD, SDMMC_MCI_CMD13_SEND_STATUS, SD_ACMD6_SET_BUS_WIDTH, SD_ACMD51_SEND_SCR, SDMMC_CMD18_READ_MULTIPLE_BLOCK, SDMMC_CMD17_READ_SINGLE_BLOCK, SDMMC_CMD12_STOP_TRANSMISSION, SDMMC_CMD25_WRITE_MULTIPLE_BLOCK, SDMMC_CMD24_WRITE_BLOCK};
use crate::sd_mmc::registers::ocr::OcrRegister;
use bit_field::BitField;
use crate::sd_mmc::registers::registers::{Register, SdMmcRegister};
use crate::sd_mmc::registers::sd::switch_status::{SwitchStatusRegister, SD_SW_STATUS_FUN_GRP_RC_ERROR};
use crate::sd_mmc::command::sd_commands::cmd6::{Cmd6, Cmd6Mode};
use crate::sd_mmc::command::flags::CommandFlag;
use crate::sd_mmc::command::response_type::Response;
use crate::sd_mmc::command::mmc_commands::BusWidth;
use crate::sd_mmc::command::sd_commands::Cmd8::Cmd8;
use crate::sd_mmc::registers::sd::card_status::CardStatusRegister;
use crate::sd_mmc::registers::sd::scr::ScrRegister;
use crate::sd_mmc::sd::sd_physical_specification::SdPhysicalSpecification;
use crate::sd_mmc::card_version::CardVersion::SdCard;
use crate::sd_mmc::mmc::SD_MMC_BLOCK_SIZE;
use crate::sd_mmc::transfer::TransferTransaction;

// SD/MMC transfer rate unit codes (10K) list
pub const SD_MMC_TRANS_UNITS: [u32; 7] = [10, 100, 1_000, 10_000, 0, 0, 0];
// SD transfer multiplier factor codes (1/10) list
pub const SD_TRANS_MULTIPLIERS: [u32; 16] = [0, 10, 12, 13, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 70, 80];
// MMC transfer multiplier factor codes (1/10) list
pub const MMC_TRANS_MULTIPLIERS: [u32; 16] = [0, 10, 12, 13, 15, 20, 26, 30, 35, 40, 45, 52, 55, 60, 70, 80];

pub struct SdMmcCard<MCI, WP, DETECT>
    where MCI: Mci,
    WP: InputPin,       // Write protect pin
    DETECT: InputPin    // Card detect pin
{
    /// Hardware interface
    pub mci: MCI,
    /// Card access clock. Defaults to 400khz
    pub clock: u32,
    /// Card capacity in KBytes
    pub capacity: u32,
    /// Relative card address
    pub rca: u16,
    /// Card state
    pub state: CardState,
    /// Card type
    pub card_type: CardType,
    /// Card version
    pub version: CardVersion,
    /// Number of DATA lines on bus (MCI only)
    pub bus_width: BusWidth,
    /// CSD register
    pub csd: CsdRegister,
    /// High speed card
    pub high_speed: bool,
    /// This card's slot number
    pub slot: u8,
    /// Write protect pin
    pub wp: WP,
    /// Whether a pulled high pin is logic true that write protection is activated
    pub wp_high_activated: bool,
    /// Card detection pin
    pub detect: DETECT,
    /// Whether a pulled high pin is logic true that a card is detected
    pub detect_high_activated: bool
}

pub fn ocr_voltage_support() -> OcrRegister {
    let mut ocr = OcrRegister { val: 0};
    ocr.set_vdd_27_28(true)
        .set_vdd_28_29(true)
        .set_vdd_29_30(true)
        .set_vdd_30_31(true)
        .set_vdd_31_32(true)
        .set_vdd_32_33(true);
    ocr
}

impl <MCI, WP, DETECT> SdMmcCard<MCI, WP, DETECT>
    where MCI: Mci,
    WP: InputPin,
    DETECT: InputPin
{
    /// Create a new SD MMC instance
    pub fn new(mci: MCI, write_protect_pin: WP, wp_high_activated: bool, detect_pin: DETECT, detect_high_activated: bool, slot: u8) -> Self {
        SdMmcCard {
            mci,
            clock: 400_000,
            capacity: 0,
            rca: 0,
            state: CardState::NoCard,
            card_type: CardType { val: 0 },
            version: CardVersion::Unknown,
            bus_width: BusWidth::_1BIT,
            csd: Default::default(),
            high_speed: false,
            slot,
            wp: write_protect_pin,
            wp_high_activated,
            detect: detect_pin,
            detect_high_activated
        }
    }

    /// Ask to all cards to send their operations conditions (MCI only).
    /// # Arguments
    /// * `v2` Shall be true if it is a SD card V2
    pub fn sd_mci_operations_conditions(&mut self, v2: bool) -> Result<(), ()> {
        // Timeout 1s = 400KHz / ((6+6+6+6)*8) cylces = 2100 retry
        for i in (0..2100).rev() {
            if i == 0 {
                return Err(()) // TODO Proper error
            }
            // CMD55 - Indicate to the card that the next command is an
            // application specific command rather than a standard command.

            self.mci.send_command(SDMMC_CMD55_APP_CMD.into(), 0)?;
            let mut arg = ocr_voltage_support();
            arg.val.set_bit(30, v2); // SD_ACMD41_HCS ACMD41 High Capacity Support
            self.mci.send_command(SD_MCI_ACMD41_SD_SEND_OP_COND.into(), arg.value())?;
            let resp = self.mci.get_response();
            let resp = OcrRegister { val: resp };
            if resp.card_powered_up_status() {
                if resp.card_capacity_status() {
                    self.card_type.set_high_capacity(true);
                }
                break;
            }
        }
        Ok(())
    }

    pub fn sd_cmd6<RESPONSE, FLAG, MODE, DEVICE>(
        &mut self,
        command: Command<RESPONSE, FLAG, MODE, DEVICE>,
        grp1_high_speed: bool,
        grp2_no_influence: bool,
        grp3_no_influence: bool,
        grp4_no_influence: bool,
        grp5_no_influence: bool,
        grp6_no_influence: bool,
        mode: Cmd6Mode
    ) -> Result<SwitchStatusRegister, ()>
        where RESPONSE: Response,
              FLAG: CommandFlag
    {
        let mut buf = [0u8; 64];
        let mut arg = Cmd6 { val: 0 };
        arg.set_function_group_1_access_mode(grp1_high_speed)
            .set_function_group2_command_system(grp2_no_influence)
            .set_function_group3(grp3_no_influence)
            .set_function_group4(grp4_no_influence)
            .set_function_group5(grp5_no_influence)
            .set_function_group6(grp6_no_influence)
            .set_mode(mode);

        self.mci.adtc_start(command.into(), arg.value(), 64, 1, true)?;
        self.mci.read_blocks(&mut buf, 1)?;
        self.mci.wait_until_read_finished()?;

        let ret: SwitchStatusRegister = buf.into();
        Ok(ret)
    }

    /// CMD6 for SD - Switch card in high speed mode
    /// CMD6 is valid under the trans state
    /// self.high_speed is updated
    /// self.clock is updated
    ///
    /// True if set to high speed
    pub fn sd_cmd6_set_to_high_speed_mode(&mut self) -> Result<bool, ()> {
        let status = self.sd_cmd6(SD_CMD6_SWITCH_FUNC, true, false, true, true, true, true, Cmd6Mode::Switch)?;

        if status.group1_info_status() == SD_SW_STATUS_FUN_GRP_RC_ERROR {
            // Not supported, not a protocol error
            return Ok(false);
        }

        if status.group1_busy() > 0 {
            return Err(()) // TODO proper error
        }

        // CMD6 function switching period is within 8 clocks after then bit of status data
        self.mci.send_clock()?;

        self.high_speed = true;
        self.clock *= 2;

        Ok(false)
    }

    /// CMD8 for SD card - send interface condition command
    /// Send SD Memory Card interface condition, which includes host supply
    /// voltage information and asks the card whether card supports voltage.
    /// Should be performed at initialization time to detect the card type.
    ///
    pub fn sd_cmd8_is_v2(&mut self) -> Result<bool, ()> {
        let mut arg = Cmd8::default();
        arg.set_cmd8_pattern(true)
            .set_high_voltage(true);

        if self.mci.send_command(SD_CMD8_SEND_IF_COND.into(), arg.val as u32).is_err() {
            return Ok(false) // Not V2
        }
        let ret = self.mci.get_response();
        if ret == 0xFFFF_FFFF {
            // No compliance R7 value
            return Ok(false)
        }
        if ret != arg.val as u32 {
            return Err(()) // TODO special error
        }
        // Is a V2
        Ok(true)
    }

    /// CMD9: Card sends its card specific data (CSD)
    /// self.csd is updated
    pub fn sd_mmc_cmd9_mci(&mut self) -> Result<(), ()> {
        let arg = (self.rca as u32) << 16;
        self.mci.send_command(SDMMC_MCI_CMD9_SEND_CSD.into(), arg)?;
        self.csd = CsdRegister {
            val: self.mci.get_response128()
        };
        Ok(())
    }

    /// Decodes the SD CSD register
    /// updates self.clock, self.capacity
    pub fn sd_decode_csd(&mut self) -> Result<(), ()> {
        // 	Get SD memory maximum transfer speed in Hz.
        let trans_speed = self.csd.transmission_speed();
        let unit = SD_MMC_TRANS_UNITS[(trans_speed & 0x7) as usize];
        let mult = SD_TRANS_MULTIPLIERS[((trans_speed >> 3) & 0xF) as usize];
        self.clock = unit * mult * 1000;

        if self.csd.sd_csd_structure_version() as u8 >= (SdCsdStructureVersion::Ver2_0 as u8) {
            self.capacity = (self.csd.sd_2_0_card_size() + 1) * 512;
        } else {
            let block_nr = ((self.csd.card_size() as u32) + 1) * ((self.csd.card_size_multiplier() as u32) + 2);
            self.capacity = block_nr * (1 << self.csd.read_bl_length() as u32) / 1024;
        }
        Ok(())
    }

    /// CMD13: Get status register.
    /// Waits for the clear of the busy flag
    pub fn sd_mmc_cmd13_get_status_and_wait_for_ready_for_data_flag(&mut self) -> Result<CardStatusRegister, ()> {
        let mut status = CardStatusRegister::default();
        /// TODO maybe proper timeout
        for i in (0..200_000u32).rev() {
            if i == 0 {
                return Err(()); // TODO proper timeout error
            }
            self.mci.send_command(SDMMC_MCI_CMD13_SEND_STATUS.into(), (self.rca as u32) << 16)?;
            status = CardStatusRegister { val: self.mci.get_response() };
            if status.ready_for_data() {
                break;
            }
        }
        Ok(status)
    }

    /// ACMD6 = Define the data bus width to be 4 bits
    pub fn sd_acmd6_set_data_bus_width_to_4_bits(&mut self) -> Result<(), ()> {
        self.mci.send_command(SDMMC_CMD55_APP_CMD.into(), (self.rca as u32) << 16)?;
        self.mci.send_command(SD_ACMD6_SET_BUS_WIDTH.into(), 0x2);
        self.bus_width = BusWidth::_4BIT;
        Ok(())
    }

    /// Get the SD Card configuration register (ACMD51)
    pub fn sd_scr(&mut self) -> Result<ScrRegister, ()> {
        let mut buf = [0u8; 8];
        self.mci.send_command(SDMMC_CMD55_APP_CMD.into(), (self.rca as u32) << 16)?;
        self.mci.adtc_start(SD_ACMD51_SEND_SCR.into(), 0, 8, 1, true)?;
        self.mci.read_blocks(&mut buf, 1)?;
        self.mci.wait_until_read_finished()?;

        Ok(buf.into())
    }

    /// ACMD51 - Read the SD Card configuration register (SCR)
    /// SCR provides information on the SD Memory Card's special features that were configured
    /// into the given card. The SCR register is 64 bits.
    /// Updates self.version
    pub fn sd_acmd51(&mut self) -> Result<(), ()> {
        let scr = self.sd_scr()?;
        self.version = match scr.sd_specification_version() {
            SdPhysicalSpecification::Revision1_01 => SdCard(SdCardVersion::Sd_1_0),
            SdPhysicalSpecification::Revision1_10 => SdCard(SdCardVersion::Sd_1_10),
            SdPhysicalSpecification::Revision2_00 => SdCard(SdCardVersion::Sd_2_0),
            _ => SdCard(SdCardVersion::Sd_1_0)
        };
        Ok(())
    }

    pub fn sd_mmc_deselect_this_device(&mut self) -> Result<(), ()> {
        self.mci.deselect_device()
    }

    pub fn sd_select_this_device_on_mci_and_configure_mci(&mut self) -> Result<(), ()> {
        self.mci.select_device(self.slot, self.clock, &self.bus_width, self.high_speed) // TODO proper error
    }

    /// Select this instance's card slot and initialize the associated driver
    pub fn sd_mmc_select_slot(&mut self) -> Result<(), ()> {
        // Check card detection
        if self.wp.is_high().map_err(|_| ())? != self.wp_high_activated {   // TODO proper error for pin check
            if self.state == CardState::Debounce {
                // TODO Timeout stop?
            }
            self.state = CardState::NoCard;
            return Err(()); // TODO no card error
        }

        if self.state == CardState::Debounce {
            if false {
                // TODO check if timed out
                return Err(()) // TODO proper timeout
            }
            self.state = CardState::Init;
            // Set 1-bit bus width and low clock for initialization
            self.clock = 400_000;
            self.bus_width = BusWidth::_1BIT;
            self.high_speed = false;
        }
        if self.state == CardState::Unusable {
            return Err(())  // TODO proper error
        }
        self.sd_select_this_device_on_mci_and_configure_mci()?; // TODO proper error
        if self.state == CardState::Init { Ok(())} else { Ok(()) }  // TODO if it is still ongoing should return ongoing
    }

    pub fn write_protected(&self) -> Result<bool, ()> {
        let level = self.wp.is_high().map_err(|_| ())?; //TODO proper error for pin fault
        Ok(level == self.wp_high_activated)
    }

    pub fn sd_mmc_init_read_blocks(&mut self, start: u32, blocks_amount: u16) -> Result<TransferTransaction, ()> {
        self.sd_select_this_device_on_mci_and_configure_mci()?;
        // Wait for data status
        self.sd_mmc_cmd13_get_status_and_wait_for_ready_for_data_flag()?;
        let cmd: u32 = if blocks_amount > 1 { SDMMC_CMD18_READ_MULTIPLE_BLOCK.into() } else { SDMMC_CMD17_READ_SINGLE_BLOCK.into() };

        // SDSC Card (CCS=0) uses byte unit address,
        // SDHC and SDXC Cards (CCS=1) use block unit address (512 Bytes unit).
        let arg = if self.card_type.high_capacity() { start } else { start * SD_MMC_BLOCK_SIZE };
        self.mci.adtc_start(cmd, arg, SD_MMC_BLOCK_SIZE as u16, blocks_amount, true)?;
        Ok(TransferTransaction {
            amount: blocks_amount,
            remaining: blocks_amount
        })
    }

    pub fn sd_mmc_start_read_blocks(&mut self, transaction: &mut TransferTransaction, destination: &mut [u8], amount_of_blocks: u16) -> Result<(), ()> {
        if self.mci.read_blocks(destination, amount_of_blocks).is_err() {
            transaction.remaining = 0;
            return Err(()); // TODO proper read error
        }
        transaction.remaining -= amount_of_blocks;
        Ok(())
    }

    pub fn sd_mmc_wait_end_of_read_blocks(&mut self, abort: bool, transaction: &mut TransferTransaction) -> Result<(), ()> {
        !self.mci.wait_until_read_finished()?;
        if abort {
            transaction.remaining = 0;
        } else if transaction.remaining > 0 {
            return Ok(())
        }

        // All blocks are transferred then stop read operation
        if transaction.remaining == 1 {
            return Ok(())
        }

        // WORKAROUND for no compliance card (Atmel Internal ref. !MMC7 !SD19)
        // The errors on this cmmand must be ignored and one retry can be necessary in SPI mode
        // for non-complying card
        if self.mci.adtc_stop(SDMMC_CMD12_STOP_TRANSMISSION.into(), 0).is_err() {
            self.mci.adtc_stop(SDMMC_CMD12_STOP_TRANSMISSION.into(), 0);
        }
        Ok(())
    }

    pub fn sd_mmc_init_write_blocks(&mut self, start: u32, blocks_amount: u16) -> Result<TransferTransaction, ()> {
        self.sd_select_this_device_on_mci_and_configure_mci()?;
        if self.write_protected()? {
            return Err(()) // TODO proper write protection error
        }

        let cmd: u32 = if blocks_amount > 1 { SDMMC_CMD25_WRITE_MULTIPLE_BLOCK.into() } else { SDMMC_CMD24_WRITE_BLOCK.into() };

        // SDSC Card (CCS=0) uses byte unit address,
        // SDHC and SDXC Cards (CCS=1) use block unit address (512 Bytes unit).
        let arg = if self.card_type.high_capacity() { start } else { start * SD_MMC_BLOCK_SIZE };

        self.mci.adtc_start(cmd, arg, SD_MMC_BLOCK_SIZE as u16, blocks_amount, true)?; // TODO proper error

        let resp = CardStatusRegister { val: self.mci.get_response() };
        if resp.write_protect_violation() {
            return Err(()) // TODO proper error
        }

        Ok(TransferTransaction {
            remaining: blocks_amount,
            amount: blocks_amount
        })
    }
}