use bit_field::BitField;
use crate::sd_mmc::registers::registers::Register;

pub struct IoEnableRegister {
    pub val: u8
}

impl Register<u8> for IoEnableRegister {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0x02u8
    }
}

impl IoEnableRegister {
    pub fn set_function1_enabled(&mut self, enabled: bool) {
        self.val.set_bit(1, enabled);
    }

    pub fn function1_enabled(&mut self) -> bool {
        self.val.get_bit(1)
    }

    pub fn set_function2_enabled(&mut self, enabled: bool) {
        self.val.set_bit(2, enabled);
    }

    pub fn function2_enabled(&mut self) -> bool {
        self.val.get_bit(2)
    }

    pub fn set_function3_enabled(&mut self, enabled: bool) {
        self.val.set_bit(3, enabled);
    }

    pub fn function3_enabled(&mut self) -> bool {
        self.val.get_bit(3)
    }

    pub fn set_function4_enabled(&mut self, enabled: bool) {
        self.val.set_bit(4, enabled);
    }

    pub fn function4_enabled(&mut self) -> bool {
        self.val.get_bit(4)
    }

    pub fn set_function5_enabled(&mut self, enabled: bool) {
        self.val.set_bit(5, enabled);
    }

    pub fn function5_enabled(&mut self) -> bool {
        self.val.get_bit(5)
    }

    pub fn set_function6_enabled(&mut self, enabled: bool) {
        self.val.set_bit(6, enabled);
    }

    pub fn function6_enabled(&mut self) -> bool {
        self.val.get_bit(6)
    }

    pub fn set_function7_enabled(&mut self, enabled: bool) {
        self.val.set_bit(7, enabled);
    }

    pub fn function7_enabled(&mut self) -> bool {
        self.val.get_bit(7)
    }
}
