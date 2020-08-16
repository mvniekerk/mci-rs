use crate::card_state::CardState;
use crate::card_type::CardType;
use crate::card_version::CardVersion;
use crate::command_arguments::mmc::BusWidth;
use crate::mci::Mci;
use crate::registers::csd::CsdRegister;
use crate::registers::ocr::OcrRegister;
use embedded_error::mci::MciError;
use embedded_hal::digital::v2::InputPin;

// SD/MMC transfer rate unit codes (10K) list
pub const SD_MMC_TRANS_UNITS: [u32; 7] = [10, 100, 1_000, 10_000, 0, 0, 0];
// SD transfer multiplier factor codes (1/10) list
pub const SD_TRANS_MULTIPLIERS: [u32; 16] = [
    0, 10, 12, 13, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 70, 80,
];
// MMC transfer multiplier factor codes (1/10) list
pub const MMC_TRANS_MULTIPLIERS: [u32; 16] = [
    0, 10, 12, 13, 15, 20, 26, 30, 35, 40, 45, 52, 55, 60, 70, 80,
];

pub struct MciCard<MCI, WP, DETECT>
where
    MCI: Mci,
    WP: InputPin,     // Write protect pin
    DETECT: InputPin, // Card detect pin
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
    pub detect_high_activated: bool,
}

pub fn ocr_voltage_support() -> OcrRegister {
    let mut ocr = OcrRegister { val: 0 };
    ocr.set_vdd_27_28(true)
        .set_vdd_28_29(true)
        .set_vdd_29_30(true)
        .set_vdd_30_31(true)
        .set_vdd_31_32(true)
        .set_vdd_32_33(true);
    ocr
}

impl<MCI, WP, DETECT> MciCard<MCI, WP, DETECT>
where
    MCI: Mci,
    WP: InputPin,
    DETECT: InputPin,
{
    /// Create a new SD MMC instance
    pub fn new(
        mci: MCI,
        write_protect_pin: WP,
        wp_high_activated: bool,
        detect_pin: DETECT,
        detect_high_activated: bool,
        slot: u8,
    ) -> Self {
        MciCard {
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
            detect_high_activated,
        }
    }

    pub fn write_protected(&self) -> Result<bool, MciError> {
        let level = self.wp.is_high().map_err(|_| MciError::PinLevelReadError)?; //TODO proper error for pin fault
        Ok(level == self.wp_high_activated)
    }
}
