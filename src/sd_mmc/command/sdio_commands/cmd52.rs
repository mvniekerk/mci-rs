use bit_field::BitField;

pub struct Cmd52 {
    pub val: u32
}

#[derive(PartialEq)]
pub enum Direction {
    Read = 0,
    Write = 1
}

impl From<bool> for Direction {
    fn from(val: bool) -> Self {
        match val {
            false => Direction::Read,
            true => Direction::Write
        }
    }
}

impl From<Direction> for bool {
    fn from(val: Direction) -> Self {
        val == Direction::Write
    }
}

impl Cmd52 {
    pub fn set_write_data(&mut self, val: u8) {
        self.val.set_bits(0..=7, val as u32);
    }

    pub fn write_data(&self) -> u8 {
        self.val.get_bits(0..=7) as u8
    }

    pub fn set_register_address(&mut self, val: u16) {
        self.val.set_bits(9..=25, val as u32);
    }

    pub fn register_address(&self) -> u16 {
        self.val.get_bits(9..=25) as u16
    }

    pub fn set_read_after_write(&mut self, enabled: bool) {
        self.val.set_bit(27, enabled);
    }

    pub fn read_after_write(&self) -> bool {
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