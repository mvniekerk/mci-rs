use crate::registers::register::RegisterAddress;
use bit_field::BitField;

/// Address pointer to function CIS (3 bytes, LSB first
pub struct CisPointerRegister {
    pub val: u32,
}

impl CisPointerRegister {
    pub fn set_lsb(&mut self, val: u8) {
        self.val.set_bits(16..=23, val as u32);
    }

    pub fn lsb(&self) -> u8 {
        self.val.get_bits(16..=23) as u8
    }

    pub fn set_mid(&mut self, val: u8) {
        self.val.set_bits(8..=15, val as u32);
    }

    pub fn mid(&self) -> u8 {
        self.val.get_bits(16..=23) as u8
    }

    pub fn set_msb(&mut self, val: u8) {
        self.val.set_bits(0..=7, val as u32);
    }

    pub fn msb(&self) -> u8 {
        self.val.get_bits(0..=7) as u8
    }
}

impl RegisterAddress for CisPointerRegister {
    fn address() -> u8 {
        0x9
    }
}
