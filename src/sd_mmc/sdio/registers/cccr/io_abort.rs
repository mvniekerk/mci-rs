use bit_field::BitField;
use crate::sd_mmc::sdio::registers::registers::Register;

pub struct IoAbortRegister {
    pub val: u8
}

impl Register for IoAbortRegister {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0x06u8
    }
}


impl IoAbortRegister {
    pub fn set_function1_abort(&mut self, abort: bool) {
        self.val.set_bits(0..2, if abort { 1 } else { 0 });
    }

    pub fn function1_abort(&mut self) -> bool {
        self.val.get_bits(0..2) == 1
    }

    pub fn set_function2_abort(&mut self, abort: bool) {
        self.val.set_bits(0..2, if abort { 2 } else { 0 });
    }

    pub fn function2_abort(&mut self) -> bool {
        self.val.get_bits(0..2) == 2
    }

    pub fn set_function3_abort(&mut self, abort: bool) {
        self.val.set_bits(0..2, if abort { 3 } else { 0 });
    }

    pub fn function3_abort(&mut self) -> bool {
        self.val.get_bits(0..2) == 3
    }

    pub fn set_function4_abort(&mut self, abort: bool) {
        self.val.set_bits(0..2, if abort { 4 } else { 0 });
    }

    pub fn function4_abort(&mut self) -> bool {
        self.val.get_bits(0..2) == 4
    }

    pub fn set_function5_abort(&mut self, abort: bool) {
        self.val.set_bits(0..2, if abort { 5 } else { 0 });
    }

    pub fn function5_abort(&mut self) -> bool {
        self.val.get_bits(0..2) == 5
    }

    pub fn set_function6_abort(&mut self, abort: bool) {
        self.val.set_bits(0..2, if abort { 6 } else { 0 });
    }

    pub fn function6_abort(&mut self) -> bool {
        self.val.get_bits(0..2) == 6
    }

    pub fn set_function7_abort(&mut self, abort: bool) {
        self.val.set_bits(0..2, if abort { 7 } else { 0 });
    }

    pub fn function7_abort(&mut self) -> bool {
        self.val.get_bits(0..2) == 7
    }

    pub fn set_card_reset(&mut self, reset: bool) {
        self.val.set_bit(3, reset);
    }

    pub fn card_reset(&self) -> bool {
        self.val.get_bit(3)
    }

    pub fn set_abort_select_in_order(&mut self, abort: bool) {
        self.val.set_bits(0..2, if abort { 7 } else { 0 });
    }

    pub fn abort_select_in_order(&self) -> bool {
        self.val.get_bits(0..2) == 7
    }
}
