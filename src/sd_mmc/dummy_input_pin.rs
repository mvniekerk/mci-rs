use atsamd_hal::hal::digital::v2::InputPin;

/// A dummy input pin that.
/// Can be used when a write protect or detection pin is not available for the peripheral
pub struct DummyInputPin {
    pub high: bool
}

impl DummyInputPin {
    pub fn set_high(&mut self, high: bool) -> &mut Self {
        self.high = high;
        self
    }
}

impl InputPin for DummyInputPin {
    type Error = ();

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.high)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.high)
    }
}
