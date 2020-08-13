use crate::sd_mmc::sd_mmc::{SdMmcCard, ocr_voltage_support};
use crate::sd_mmc::mci::Mci;
use atsamd_hal::hal::digital::v2::InputPin;
use crate::sd_mmc::commands::{MMC_MCI_CMD1_SEND_OP_COND, MMC_CMD6_SWITCH};
use crate::sd_mmc::registers::ocr::{OcrRegister, AccessMode};
use crate::sd_mmc::command::mmc_commands::{BusWidth, Cmd6, Access};
use crate::sd_mmc::mode_index::ModeIndex;
use crate::sd_mmc::registers::sd::card_status::CardStatusRegister;

impl <MCI, WP, DETECT> SdMmcCard<MCI, WP, DETECT>
    where MCI: Mci,
          WP: InputPin,       // Write protect pin
          DETECT: InputPin    // Card detect pin
{
    /// Sends operation condition command and read OCR (MCI only)
    pub fn mmc_mci_send_operation_condition(&mut self) -> Result<(), ()> {
        let mut ocr = ocr_voltage_support();
        ocr.set_access_mode(AccessMode::Sector);
        // Timeout is 1s = 400KHz / ((6+6)*8) cycles = 4200 retries. TODO maybe a delay check?
        for i in (0..4200).rev() {
            if i == 0 {
                return Err(()) // TODO proper error
            }
            self.mci.send_command(MMC_MCI_CMD1_SEND_OP_COND.into(), ocr.val)?;
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
    pub fn mmc_cmd6_set_bus_width(&mut self, bus_width: BusWidth) -> Result<bool, ()> {
        let mut arg = Cmd6::default();
        arg.set_access(Access::SetBits)
            .set_bus_width(&bus_width)
            .set_mode_index(ModeIndex::BusWidth);
        self.mci.send_command(MMC_CMD6_SWITCH.into(), arg.val)?;
        let ret = CardStatusRegister { val: self.mci.get_response() };
        if ret.switch_error() {
            // Not supported, not a protocol error
            return Ok(false)
        }
        self.bus_width = bus_width;
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
        self.mci.send_command(MMC_CMD6_SWITCH.into())?;
        let ret = CardStatusRegister { val: self.mci.get_response() };
        if ret.switch_error() {
            // Not supported, not a protocol error
            return Ok(false);
        }
        self.high_speed = true;
        self.clock = 52_000_000u32;
        Ok(true)
    }
}