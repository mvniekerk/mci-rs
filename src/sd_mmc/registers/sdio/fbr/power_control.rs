use crate::sd_mmc::registers::register::Register;
use bit_field::BitField;

pub struct PowerControlRegister {
    pub val: u8,
}

impl PowerControlRegister {
    pub fn set_function_support_power_control(&mut self, supports: bool) {
        self.val.set_bit(0, supports);
    }

    pub fn function_supports_power_control(&mut self) -> bool {
        self.val.get_bit(0)
    }

    pub fn set_high_current_mode(&mut self, high_current_mode: bool) {
        self.val.set_bit(1, high_current_mode);
    }

    /// True for high current mode, false for low current mode
    pub fn high_current_mode(&self) -> bool {
        self.val.get_bit(1)
    }
}

impl Register<u8> for PowerControlRegister {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0x2
    }
}
