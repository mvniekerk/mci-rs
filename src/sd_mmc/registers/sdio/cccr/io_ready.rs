use crate::sd_mmc::registers::register::Register;
use bit_field::BitField;

pub struct IoReadyRegister {
    pub val: u8,
}

impl Register<u8> for IoReadyRegister {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0x03u8
    }
}

impl IoReadyRegister {
    pub fn set_function1_ready(&mut self, ready: bool) {
        self.val.set_bit(1, ready);
    }

    pub fn function1_ready(&mut self) -> bool {
        self.val.get_bit(1)
    }

    pub fn set_function2_ready(&mut self, ready: bool) {
        self.val.set_bit(2, ready);
    }

    pub fn function2_ready(&mut self) -> bool {
        self.val.get_bit(2)
    }

    pub fn set_function3_ready(&mut self, ready: bool) {
        self.val.set_bit(3, ready);
    }

    pub fn function3_ready(&mut self) -> bool {
        self.val.get_bit(3)
    }

    pub fn set_function4_ready(&mut self, ready: bool) {
        self.val.set_bit(4, ready);
    }

    pub fn function4_ready(&mut self) -> bool {
        self.val.get_bit(4)
    }

    pub fn set_function5_ready(&mut self, ready: bool) {
        self.val.set_bit(5, ready);
    }

    pub fn function5_ready(&mut self) -> bool {
        self.val.get_bit(5)
    }

    pub fn set_function6_ready(&mut self, ready: bool) {
        self.val.set_bit(6, ready);
    }

    pub fn function6_ready(&mut self) -> bool {
        self.val.get_bit(6)
    }

    pub fn set_function7_ready(&mut self, ready: bool) {
        self.val.set_bit(7, ready);
    }

    pub fn function7_ready(&mut self) -> bool {
        self.val.get_bit(7)
    }
}
