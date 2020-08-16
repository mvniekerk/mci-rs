use crate::registers::register::RegisterAddress;
use bit_field::BitField;

/// Block size, LSB first, 2 bytes
pub struct BlockSize {
    pub val: u16,
}

impl BlockSize {
    pub fn set_lsb(&mut self, val: u8) {
        self.val.set_bits(8..16, val as u16);
    }

    pub fn lsb(&self) -> u8 {
        self.val.get_bits(8..16) as u8
    }

    pub fn set_msb(&mut self, val: u8) {
        self.val.set_bits(0..8, val as u16);
    }

    pub fn msb(&mut self) -> u8 {
        self.val.get_bits(0..8) as u8
    }
}

impl RegisterAddress for BlockSize {
    fn address() -> u8 {
        0x10u8
    }
}
