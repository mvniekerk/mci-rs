use bit_field::BitField;

pub struct IoAbortRegister {
    pub val: u8
}

impl IoAbortRegister {
    pub fn set_function1_abort(&mut self, abort: bool) {
        self.val.set_bit(1, abort);
    }

    pub fn function1_abort(&mut self) -> bool {
        self.val.get_bit(1)
    }

    pub fn set_function2_abort(&mut self, abort: bool) {
        self.val.set_bit(2, abort);
    }

    pub fn function2_abort(&mut self) -> bool {
        self.val.get_bit(2)
    }

    pub fn set_function3_abort(&mut self, abort: bool) {
        self.val.set_bit(3, abort);
    }

    pub fn function3_abort(&mut self) -> bool {
        self.val.get_bit(3)
    }

    pub fn set_function4_abort(&mut self, abort: bool) {
        self.val.set_bit(4, abort);
    }

    pub fn function4_abort(&mut self) -> bool {
        self.val.get_bit(4)
    }

    pub fn set_function5_abort(&mut self, abort: bool) {
        self.val.set_bit(5, abort);
    }

    pub fn function5_abort(&mut self) -> bool {
        self.val.get_bit(5)
    }

    pub fn set_function6_abort(&mut self, abort: bool) {
        self.val.set_bit(6, abort);
    }

    pub fn function6_abort(&mut self) -> bool {
        self.val.get_bit(6)
    }

    pub fn set_function7_abort(&mut self, abort: bool) {
        self.val.set_bit(7, abort);
    }

    pub fn function7_abort(&mut self) -> bool {
        self.val.get_bit(7)
    }
}
