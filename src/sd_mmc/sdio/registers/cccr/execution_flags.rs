use bit_field::BitField;

pub struct ExecutionFlagsRegister {
    pub val: u8
}

impl ExecutionFlagsRegister {
    pub fn set_executing_status_of_memory(&mut self, executing: bool) {
        self.val.set_bit(0, executing);
    }

    pub fn executing_status_of_memory(&self) -> bool {
        self.val.get_bit(0)
    }

    pub fn set_executing_status_of_function1(&mut self, executing: bool) {
        self.val.set_bit(1, executing);
    }

    pub fn executing_status_of_function1(&self) -> bool {
        self.val.get_bit(1)
    }

    pub fn set_executing_status_of_function2(&mut self, executing: bool) {
        self.val.set_bit(2, executing);
    }

    pub fn executing_status_of_function2(&self) -> bool {
        self.val.get_bit(2)
    }

    pub fn set_executing_status_of_function3(&mut self, executing: bool) {
        self.val.set_bit(3, executing);
    }

    pub fn executing_status_of_function3(&self) -> bool {
        self.val.get_bit(3)
    }

    pub fn set_executing_status_of_function4(&mut self, executing: bool) {
        self.val.set_bit(4, executing);
    }

    pub fn executing_status_of_function4(&self) -> bool {
        self.val.get_bit(4)
    }

    pub fn set_executing_status_of_function5(&mut self, executing: bool) {
        self.val.set_bit(5, executing);
    }

    pub fn executing_status_of_function5(&self) -> bool {
        self.val.get_bit(5)
    }

    pub fn set_executing_status_of_function6(&mut self, executing: bool) {
        self.val.set_bit(6, executing);
    }

    pub fn executing_status_of_function6(&self) -> bool {
        self.val.get_bit(6)
    }

    pub fn set_executing_status_of_function7(&mut self, executing: bool) {
        self.val.set_bit(7, executing);
    }

    pub fn executing_status_of_function7(&self) -> bool {
        self.val.get_bit(7)
    }
}