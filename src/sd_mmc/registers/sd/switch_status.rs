use crate::sd_mmc::registers::registers::Register;
use bit_field::{BitField, BitArray};

pub struct SwitchStatusRegister {
    pub val: [u16; 32]
}

impl Register<[u16; 32]> for SwitchStatusRegister {
    fn value(&self) -> [u16;32] {
        self.val
    }

    fn address() -> u8 {
        0x0
    }
}

impl From<[u8; 64]> for SwitchStatusRegister {
    fn from(val: [u8; 64]) -> Self {
        let mut v = [0u16; 32];
        for i in 0..64 {
            v[i / 2] = (val[i] as u16) << ((i % 2) * 8);
        }
        SwitchStatusRegister {
            val: v
        }
    }
}

impl SwitchStatusRegister {
    pub fn set_max_current_consumption(&mut self, max: u16) {
        self.val.set_bits(496..512, max);
    }

    pub fn max_current_consumption(&self) -> u16 {
        self.val.get_bits(496..512) as u16
    }

    pub fn set_group6_info_status(&mut self, val: u16) {
        self.val.set_bits(480..496, val);
    }

    pub fn group6_info_status(&self) -> u16 {
        self.val.get_bits(480..496) as u16
    }

    pub fn set_group5_info_status(&mut self, val: u16) {
        self.val.set_bits(464..480, val);
    }

    pub fn group5_info_status(&self) -> u16 {
        self.val.get_bits(464..480) as u16
    }

    pub fn set_group4_info_status(&mut self, val: u16) {
        self.val.set_bits(448..464, val);
    }

    pub fn group4_info_status(&self) -> u16 {
        self.val.get_bits(448..464) as u16
    }

    pub fn set_group3_info_status(&mut self, val: u16) {
        self.val.set_bits(432..448, val);
    }

    pub fn group3_info_status(&self) -> u16 {
        self.val.get_bits(432..448) as u16
    }

    pub fn set_group1_info_status(&mut self, val: u16) {
        self.val.set_bits(416..432, val);
    }

    pub fn group1_info_status(&self) -> u16 {
        self.val.get_bits(416..432) as u16
    }

    pub fn set_group6_rc(&mut self, val: u8) {
        self.val.set_bits(396..400, val as u16);
    }

    pub fn get_group6_rc(&self) -> u8 {
        self.val.get_bits(396..400) as u8
    }

    pub fn set_group5_rc(&mut self, val: u8) {
        self.val.set_bits(392..396, val as u16);
    }

    pub fn group5_rc(&self) -> u8 {
        self.val.get_bits(392..396) as u8
    }

    pub fn set_group4_rc(&mut self, val: u8) {
        self.val.set_bits(388..392, val as u16);
    }

    pub fn group4_rc(&self) -> u8 {
        self.val.get_bits(388..392) as u8
    }

    pub fn set_group3_rc(&mut self, val: u8) {
        self.val.set_bits(384..388, val as u16);
    }

    pub fn group3_rc(&self) -> u8 {
        self.val.get_bits(384..388) as u8
    }

    pub fn set_group2_rc(&mut self, val: u8) {
        self.val.set_bits(380..384, val as u16);
    }

    pub fn group2_rc(&self) -> u8 {
        self.val.get_bits(380..384) as u8
    }

    pub fn set_group1_rc(&mut self, val: u8) {
        self.val.set_bits(376..380, val as u16);
    }

    pub fn group1_rc(&self) -> u8 {
        self.val.get_bits(376..380) as u8
    }

    pub fn set_structure_version(&mut self, val: u8) {
        self.val.set_bits(368..376, val as u16);
    }

    pub fn structure_version(&self) -> u8 {
        self.val.get_bits(368..376) as u8
    }

    pub fn set_group6_busy(&mut self, val: u16) {
        self.val.set_bits(352..368, val);
    }

    pub fn group6_busy(&self) -> u16 {
        self.val.get_bits(352..368) as u16
    }

    pub fn set_group5_busy(&mut self, val: u16) {
        self.val.set_bits(336..352, val);
    }

    pub fn group5_busy(&self) -> u16 {
        self.val.get_bits(336..352) as u16
    }

    pub fn set_group4_busy(&mut self, val: u16) {
        self.val.set_bits(320..336, val);
    }

    pub fn group4_busy(&self) -> u16 {
        self.val.get_bits(320..336) as u16
    }

    pub fn set_group3_busy(&mut self, val: u16) {
        self.val.set_bits(304..320, val);
    }

    pub fn group3_busy(&self) -> u16 {
        self.val.get_bits(304..320) as u16
    }

    pub fn set_group2_busy(&mut self, val: u16) {
        self.val.set_bits(288..304, val);
    }

    pub fn group2_busy(&self) -> u16 {
        self.val.get_bits(288..304) as u16
    }

    pub fn set_group1_busy(&mut self, val: u16) {
        self.val.set_bits(272..288, val);
    }

    pub fn group1_busy(&self) -> u16 {
        self.val.get_bits(272..288) as u16
    }
}


