use crate::sd_mmc::registers::registers::Register;
use bit_field::BitArray;

pub struct CsdRegister {
    pub val: [u64; 2]
}

impl Register<[u64; 2]> for CsdRegister {
    fn value(&self) -> [u64;2] {
        self.val
    }

    fn address() -> u8 {
        0x10u8
    }
}

impl CsdRegister {
    pub fn set_csd_structure_version(&mut self, version: u8) {
        self.val.set_bits(126..128, version as u64);
    }

    pub fn csd_structure_version(&self) -> u8 {
        self.val.get_bits(126..128) as u8
    }

    pub fn set_mmc_csd_spec_version(&mut self, version: u8) {
        self.val.set_bits(122..126, version as u64);
    }

    pub fn mmc_csd_spec_version(&self) -> u8 {
        self.val.get_bits(122..126) as u8
    }

    pub fn set_transmission_speed(&mut self, speed: u8) {
        self.val.set_bits(96..104, speed as u64);
    }

    pub fn transmission_speed(&self) -> u8 {
        self.val.get_bits(96..104) as u8
    }

    pub fn set_read_bl_length(&mut self, length: u8) {
        self.val.set_bits(80..84, length as u64);
    }

    pub fn read_bl_length(&self) -> u8 {
        self.val.get_bits(80..84) as u8
    }

    pub fn set_card_size(&mut self, size: u16) {
        self.val.set_bits(62..74, size as u64);
    }

    pub fn card_size(&self) -> u16 {
        self.val.get_bits(62..74) as u16
    }

    pub fn set_sd_2_0_card_size(&mut self, size: u32) {
        self.val.set_bits(48..70, size as u64);
    }

    pub fn sd_2_0_card_size(&self) -> u32 {
        self.val.get_bits(48..70) as u32
    }

    pub fn set_card_size_multiplier(&mut self, multiplier: u8) {
        self.val.set_bits(47..50, multiplier as u64);
    }

    pub fn card_size_multiplier(&self) -> u8 {
        self.val.get_bits(47..50) as u8
    }

}