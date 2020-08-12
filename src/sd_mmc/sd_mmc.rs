use crate::sd_mmc::mci::Mci;
use crate::sd_mmc::card_state::CardState;
use crate::sd_mmc::card_type::CardType;
use crate::sd_mmc::card_version::CardVersion;
use crate::sd_mmc::sd::sd_bus_width::SdBusWidth;
use crate::sd_mmc::registers::csd::CsdRegister;
use atsamd_hal::hal::digital::v2::InputPin;

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