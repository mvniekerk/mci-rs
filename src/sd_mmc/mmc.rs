use crate::sd_mmc::sd_mmc::{SdMmcCard, ocr_voltage_support};
use crate::sd_mmc::mci::Mci;
use atsamd_hal::hal::digital::v2::InputPin;
use crate::sd_mmc::commands::MMC_MCI_CMD1_SEND_OP_COND;
use crate::sd_mmc::registers::ocr::{OcrRegister, AccessMode};

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
}