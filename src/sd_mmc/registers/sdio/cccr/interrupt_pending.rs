use bit_field::BitField;
use crate::sd_mmc::registers::registers::Register;

pub struct InterruptPendingRegister {
    pub val: u8
}

impl Register<u8> for InterruptPendingRegister {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0x05
    }
}

impl InterruptPendingRegister {
    pub fn set_function1_interrupt_pending(&mut self, pending: bool) {
        self.val.set_bit(1, pending);
    }

    pub fn function1_interrupt_pending(&mut self) -> bool {
        self.val.get_bit(1)
    }

    pub fn set_function2_interrupt_pending(&mut self, pending: bool) {
        self.val.set_bit(2, pending);
    }

    pub fn function2_interrupt_pending(&mut self) -> bool {
        self.val.get_bit(2)
    }

    pub fn set_function3_interrupt_pending(&mut self, pending: bool) {
        self.val.set_bit(3, pending);
    }

    pub fn function3_interrupt_pending(&mut self) -> bool {
        self.val.get_bit(3)
    }

    pub fn set_function4_interrupt_pending(&mut self, pending: bool) {
        self.val.set_bit(4, pending);
    }

    pub fn function4_interrupt_pending(&mut self) -> bool {
        self.val.get_bit(4)
    }

    pub fn set_function5_interrupt_pending(&mut self, pending: bool) {
        self.val.set_bit(5, pending);
    }

    pub fn function5_interrupt_pending(&mut self) -> bool {
        self.val.get_bit(5)
    }

    pub fn set_function6_interrupt_pending(&mut self, pending: bool) {
        self.val.set_bit(6, pending);
    }

    pub fn function6_interrupt_pending(&mut self) -> bool {
        self.val.get_bit(6)
    }

    pub fn set_function7_interrupt_pending(&mut self, pending: bool) {
        self.val.set_bit(7, pending);
    }

    pub fn function7_interrupt_pending(&mut self) -> bool {
        self.val.get_bit(7)
    }
}
