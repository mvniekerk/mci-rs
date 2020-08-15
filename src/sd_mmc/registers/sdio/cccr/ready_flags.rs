use crate::sd_mmc::registers::register::Register;
use bit_field::BitField;

pub struct ReadyFlagsRegister {
    pub val: u8,
}

impl Register<u8> for ReadyFlagsRegister {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0x0Fu8
    }
}

impl ReadyFlagsRegister {
    pub fn set_ready_flag_for_memory(&mut self, ready: bool) {
        self.val.set_bit(0, ready);
    }

    pub fn ready_flag_for_memory(&self) -> bool {
        self.val.get_bit(0)
    }

    pub fn set_ready_flag_for_function1(&mut self, ready: bool) {
        self.val.set_bit(1, ready);
    }

    pub fn ready_flag_for_function1(&self) -> bool {
        self.val.get_bit(1)
    }

    pub fn set_ready_flag_for_function2(&mut self, ready: bool) {
        self.val.set_bit(2, ready);
    }

    pub fn ready_flag_for_function2(&self) -> bool {
        self.val.get_bit(2)
    }

    pub fn set_ready_flag_for_function3(&mut self, ready: bool) {
        self.val.set_bit(3, ready);
    }

    pub fn ready_flag_for_function3(&self) -> bool {
        self.val.get_bit(3)
    }

    pub fn set_ready_flag_for_function4(&mut self, ready: bool) {
        self.val.set_bit(4, ready);
    }

    pub fn ready_flag_for_function4(&self) -> bool {
        self.val.get_bit(4)
    }

    pub fn set_ready_flag_for_function5(&mut self, ready: bool) {
        self.val.set_bit(5, ready);
    }

    pub fn ready_flag_for_function5(&self) -> bool {
        self.val.get_bit(5)
    }

    pub fn set_ready_flag_for_function6(&mut self, ready: bool) {
        self.val.set_bit(6, ready);
    }

    pub fn ready_flag_for_function6(&self) -> bool {
        self.val.get_bit(6)
    }

    pub fn set_ready_flag_for_function7(&mut self, ready: bool) {
        self.val.set_bit(7, ready);
    }

    pub fn ready_flag_for_function7(&self) -> bool {
        self.val.get_bit(7)
    }
}
