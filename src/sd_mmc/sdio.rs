use crate::sd_mmc::command::device_type::{SdioDeviceType, SdioDevice};
use crate::sd_mmc::sd_mmc::{SdMmcCard, ocr_voltage_support};
use crate::sd_mmc::mci::Mci;
use atsamd_hal::hal::digital::v2::InputPin;
use crate::sd_mmc::commands::SDIO_CMD5_SEND_OP_COND;
use crate::sd_mmc::registers::ocr::OcrRegister;
use crate::sd_mmc::registers::registers::Register;

impl SdioDevice {

}

impl <MCI, WP, DETECT> SdMmcCard<MCI, WP, DETECT>
    where MCI: Mci,
    WP: InputPin,
    DETECT: InputPin
{
    /// Try to get the SDIO card's operating condition
    pub fn sdio_send_operation_condition_command(&mut self) -> Result<(), ()> {
        if self.mci.send_command(SDIO_CMD5_SEND_OP_COND.into(), 0).is_err() {
            // No error but card type not updated
            return Ok(())
        }
        let resp = self.mci.get_response();
        let resp = OcrRegister { val: resp };
        if !resp.number_of_io_functions() {
            // No error but card type not updated
            return Ok(())
        }

        let arg = OcrRegister { val: resp.val & ocr_voltage_support().value() };
        let arg = arg.value();

        // Wait until card is ready
        // Timeout 1s = 400KHz / ((6+4)*8) cycles = 5000 retry
        // TODO use proper delay?
        for i in (0..5000).rev() {
            if i == 0 {
                return Err(()) // TODO proper error
            }
            self.mci.send_command(SDIO_CMD5_SEND_OP_COND.into(), arg)?;
            let resp = OcrRegister { val: self.mci.get_response() };
            if resp.card_powered_up_status() {
                self.card_type.set_sdio(true);
                if resp.memory_present() {
                    self.card_type.set_sd(true);
                }
                break;
            }
        }

        Ok(())
    }
}
