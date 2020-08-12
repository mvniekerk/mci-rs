use bit_field::BitField;
use crate::sd_mmc::registers::registers::Register;

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
    pub fn set_write_data(&mut self, val: u8) -> &mut Self {
        self.val.set_bits(0..8, val as u32);
        self
    }

    pub fn write_data(&self) -> u8 {
        self.val.get_bits(0..=7) as u8
    }

    pub fn set_register_address(&mut self, val: u32) -> &mut Self {
        self.val.set_bits(9..25, val);
        self
    }

    pub fn register_address(&self) -> u32 {
        self.val.get_bits(9..25)
    }

    pub fn set_read_after_write(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(27, enabled);
        self
    }

    pub fn read_after_write(&self) -> bool {
        self.val.get_bit(27)
    }

    pub fn set_function_number(&mut self, function: u8) -> &mut Self {
        self.val.set_bits(28..31, function as u32);
        self
    }

    pub fn function_number(&self) -> u8 {
        self.val.get_bits(28..=30) as u8
    }

    pub fn set_direction(&mut self, direction: Direction) -> &mut Self {
        self.val.set_bit(31, direction.into());
        self
    }

    pub fn direction(&self) -> Direction {
        self.val.get_bit(31).into()
    }
}

impl Register<u32> for Cmd52 {
    #[inline(always)]
    fn value(&self) -> u32 {
        self.val
    }

    fn address() -> u8 {
        unimplemented!()
    }
}