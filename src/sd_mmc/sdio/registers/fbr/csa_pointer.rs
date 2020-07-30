use bit_field::BitField;
use crate::sd_mmc::sdio::registers::registers::Register;

/// Address pointer to CSA, 3 bytes, LSB first
pub struct CsaPointerRegister {
    pub val: u32
}

impl CsaPointerRegister {
    pub fn set_lsb(&mut self, val: u8) {
        self.val.set_bits(16..23, val as u32);
    }

    pub fn set_mid(&mut self, val: u8) {
        self.val.set_bits(8..15, val as u32);
    }

    pub fn set_msb(&mut self, val: u8) {
        self.val.set_bits(0..7, val as u32);
    }
}

impl Register<u32> for CsaPointerRegister {
    fn value(&self) -> u32 {
        self.val
    }

    fn address() -> u8 {
        0x9
    }
}