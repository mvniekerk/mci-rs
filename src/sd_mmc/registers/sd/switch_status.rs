use crate::sd_mmc::registers::registers::Register;
use bit_field::{BitField, BitArray};

pub struct SwitchStatusRegister {
    pub val: [u64; 8]
}

impl Register<[u64; 8]> for SwitchStatusRegister {
    fn value(&self) -> [u64;8] {
        self.val
    }

    fn address() -> u8 {
        0x0
    }
}

impl SwitchStatusRegister {
    pub fn set_max_current_consumption(&mut self, max: u16) {
        self.val.set_bits(496..512, max as u64);
    }

    pub fn max_current_consumption(&self) -> u16 {
        self.val.get_bits(496..512) as u16
    }

    pub fn set_grp6_info_status(&mut self, val: u16) {
        self.val.set_bits(490..496, val as u64);
    }
}


