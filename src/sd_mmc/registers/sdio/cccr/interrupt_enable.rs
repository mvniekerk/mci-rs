use bit_field::BitField;
use crate::sd_mmc::registers::registers::Register;
use crate::sd_mmc::sdio::registers::cccr::io_enable::IoEnableRegister;

pub struct InterruptEnableRegister {
    pub val: u8
}

impl Register<u8> for InterruptEnableRegister {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0x04u8
    }
}

impl InterruptEnableRegister {
    pub fn set_interrupts_enabled(&mut self, enabled: bool) {
        self.val.set_bit(0, enabled);
    }

    pub fn interrupts_enabled(&self) -> bool {
        self.val.get_bit(0)
    }

    pub fn set_function1_interrupt_enabled(&mut self, enabled: bool) {
        self.val.set_bit(1, enabled);
    }

    pub fn function1_interrupt_enabled(&mut self) -> bool {
        self.val.get_bit(1)
    }

    pub fn set_function2_interrupt_enabled(&mut self, enabled: bool) {
        self.val.set_bit(2, enabled);
    }

    pub fn function2_interrupt_enabled(&mut self) -> bool {
        self.val.get_bit(2)
    }

    pub fn set_function3_interrupt_enabled(&mut self, enabled: bool) {
        self.val.set_bit(3, enabled);
    }

    pub fn function3_interrupt_enabled(&mut self) -> bool {
        self.val.get_bit(3)
    }

    pub fn set_function4_interrupt_enabled(&mut self, enabled: bool) {
        self.val.set_bit(4, enabled);
    }

    pub fn function4_interrupt_enabled(&mut self) -> bool {
        self.val.get_bit(4)
    }

    pub fn set_function5_interrupt_enabled(&mut self, enabled: bool) {
        self.val.set_bit(5, enabled);
    }

    pub fn function5_interrupt_enabled(&mut self) -> bool {
        self.val.get_bit(5)
    }

    pub fn set_function6_interrupt_enabled(&mut self, enabled: bool) {
        self.val.set_bit(6, enabled);
    }

    pub fn function6_interrupt_enabled(&mut self) -> bool {
        self.val.get_bit(6)
    }

    pub fn set_function7_interrupt_enabled(&mut self, enabled: bool) {
        self.val.set_bit(7, enabled);
    }

    pub fn function7_interrupt_enabled(&mut self) -> bool {
        self.val.get_bit(7)
    }
}
