use crate::registers::register_address::RegisterAddress;
use bit_field::BitArray;

#[derive(Default)]
pub struct CsdRegister {
    pub val: [u32; 4],
}

impl RegisterAddress for CsdRegister {
    fn address() -> u8 {
        0x10u8
    }
}

impl From<[u8; 32]> for CsdRegister {
    fn from(val: [u8; 32]) -> Self {
        let mut v = [0u32; 4];
        for i in 0..32 {
            v[i / 4] |= (val[i] as u32) << ((i % 4) * 8);
        }
        CsdRegister { val: v }
    }
}

#[cfg(feature = "mmc")]
pub enum MmcCsdStructureVersion {
    Unknown = -1,
    Ver1d0 = 0,
    Ver1d1 = 1,
    Ver1d2 = 2,
}

#[cfg(feature = "mmc")]
impl From<u8> for MmcCsdStructureVersion {
    fn from(val: u8) -> Self {
        match val {
            0 => MmcCsdStructureVersion::Ver1d0,
            1 => MmcCsdStructureVersion::Ver1d1,
            2 => MmcCsdStructureVersion::Ver1d2,
            _ => MmcCsdStructureVersion::Unknown,
        }
    }
}

pub enum SdCsdStructureVersion {
    Unknown = -1,
    Ver1d0 = 0,
    Ver2d0 = 1,
}

impl From<u8> for SdCsdStructureVersion {
    fn from(val: u8) -> Self {
        match val {
            0 => SdCsdStructureVersion::Ver1d0,
            1 => SdCsdStructureVersion::Ver2d0,
            _ => SdCsdStructureVersion::Unknown,
        }
    }
}

impl CsdRegister {
    pub fn set_csd_structure_version(&mut self, version: u8) {
        self.val.set_bits(126..128, version as u32);
    }

    pub fn csd_structure_version(&self) -> u8 {
        self.val.get_bits(126..128) as u8
    }

    pub fn sd_csd_structure_version(&self) -> SdCsdStructureVersion {
        self.csd_structure_version().into()
    }

    #[cfg(feature = "mmc")]
    pub fn mmc_csd_structure_version(&self) -> MmcCsdStructureVersion {
        self.csd_structure_version().into()
    }

    #[cfg(feature = "mmc")]
    pub fn set_mmc_csd_spec_version(&mut self, version: u8) {
        self.val.set_bits(122..126, version as u32);
    }

    #[cfg(feature = "mmc")]
    pub fn mmc_csd_spec_version(&self) -> u8 {
        self.val.get_bits(122..126) as u8
    }

    pub fn set_transmission_speed(&mut self, speed: u8) {
        self.val.set_bits(96..104, speed as u32);
    }

    pub fn transmission_speed(&self) -> u8 {
        self.val.get_bits(96..104) as u8
    }

    pub fn set_read_bl_length(&mut self, length: u8) {
        self.val.set_bits(80..84, length as u32);
    }

    pub fn read_bl_length(&self) -> u8 {
        self.val.get_bits(80..84) as u8
    }

    pub fn set_card_size(&mut self, size: u16) {
        self.val.set_bits(62..74, size as u32);
    }

    pub fn card_size(&self) -> u16 {
        self.val.get_bits(62..74) as u16
    }

    pub fn set_sd_2_0_card_size(&mut self, size: u32) {
        self.val.set_bits(48..70, size as u32);
    }

    pub fn sd_2_0_card_size(&self) -> u32 {
        self.val.get_bits(48..70) as u32
    }

    pub fn set_card_size_multiplier(&mut self, multiplier: u8) {
        self.val.set_bits(47..50, multiplier as u32);
    }

    pub fn card_size_multiplier(&self) -> u8 {
        self.val.get_bits(47..50) as u8
    }
}
