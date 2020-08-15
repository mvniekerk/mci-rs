use crate::mci_card::MciCard;
use crate::mci::Mci;
use embedded_hal::digital::v2::InputPin;

impl<MCI, WP, DETECT> MciCard<MCI, WP, DETECT>
    where
        MCI: Mci,
        WP: InputPin,
        DETECT: InputPin,
{

}