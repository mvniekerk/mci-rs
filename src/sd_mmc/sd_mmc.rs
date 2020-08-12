use crate::sd_mmc::mci::Mci;
use crate::sd_mmc::card_state::CardState;
use crate::sd_mmc::card_type::CardType;
use crate::sd_mmc::card_version::CardVersion;
use crate::sd_mmc::sd::sd_bus_width::SdBusWidth;
use crate::sd_mmc::registers::csd::CsdRegister;
use atsamd_hal::hal::digital::v2::InputPin;
use crate::sd_mmc::commands::{SD_MCI_ACMD41_SD_SEND_OP_COND, SDMMC_CMD55_APP_CMD};
use crate::sd_mmc::registers::ocr::OcrRegister;
use bit_field::BitField;
use crate::sd_mmc::registers::registers::Register;

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
    /// Card access clock
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
    pub bus_width: SdBusWidth,
    /// CSD register
    pub csd: CsdRegister,
    /// High speed card
    pub high_speed: bool,
    /// Write protect pin
    pub wp: WP,
    pub detect: DETECT
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
}