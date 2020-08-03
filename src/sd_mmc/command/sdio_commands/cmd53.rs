use bit_field::BitField;

pub struct Cmd53 {
    pub val: u32
}

#[derive(PartialEq)]
pub enum OpCode {
    Fixed = 0,
    Line = 1,
}

impl From<OpCode> for bool {
    fn from(val: OpCode) -> Self {
        val == OpCode::Line
    }
}

impl From<bool> for OpCode {
    fn from(val: bool) -> Self {
        if val {
            OpCode::Line
        } else {
            OpCode::Fixed
        }
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Read = 0,
    Write = 1
}

impl From<Direction> for bool {
    fn from(val: Direction) -> Self {
        val == Direction::Write
    }
}

impl From<bool> for Direction {
    fn from(val: bool) -> Self {
        if val {
            Direction::Write
        } else {
            Direction::Read
        }
    }
}

impl Cmd53 {
    pub fn set_block_or_bytes_count(&mut self, amount: u16) {
        self.val.set_bits(0..=8, amount as u32);
    }

    pub fn block_or_bytes_count(&self) -> u16 {
        self.val.get_bits(0..=8) as u16
    }

    pub fn set_address(&mut self, address: u16) {
        self.val.set_bits(9..=25, address as u32);
    }

    pub fn address(&self) -> u16 {
        self.val.get_bits(9..=25) as u16
    }

    pub fn set_op_code(&mut self, op_code: OpCode) {
        self.val.set_bit(26, op_code.into());
    }

    pub fn op_code(&self) -> OpCode {
        self.val.get_bit(26).into()
    }

    pub fn set_block_mode(&mut self, enabled: bool) {
        self.val.set_bit(27, enabled);
    }

    pub fn block_mode(&self) -> bool {
        self.val.get_bit(27)
    }

    pub fn set_function_number(&mut self, function: u8) {
        self.val.set_bits(28..=30, function as u32);
    }

    pub fn function_number(&self) -> u8 {
        self.val.get_bits(28..=30) as u8
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.val.set_bit(31, direction.into());
    }

    pub fn direction(&self) -> Direction {
        self.val.get_bit(31).into()
    }
}