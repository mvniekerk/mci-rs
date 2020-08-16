use crate::registers::register_address::RegisterAddress;
use bit_field::BitField;
use core::hint::unreachable_unchecked;

pub enum CccrRevision {
    /// CCCR/FBR revision 1.00
    Revision1_00 = 0,
    /// CCCR/FBR revision 1.10
    Revision1_10 = 1,
    /// CCCR/FBR revision 2.00
    Revision2_00 = 2,
    /// CCCR/FBR revision 3.00
    Revision3_00 = 3,
}

impl From<u8> for CccrRevision {
    fn from(val: u8) -> Self {
        match val {
            0 => CccrRevision::Revision1_00,
            1 => CccrRevision::Revision1_10,
            2 => CccrRevision::Revision2_00,
            3 => CccrRevision::Revision3_00,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

pub enum SdioSpecification {
    /// SDIO specification version 1.00
    Specification1_00 = 0,
    /// SDIO specification version 1.10
    Specification1_10 = 1,
    /// SDIO specification version 1.20 (unreleased)
    Specification1_20 = 2,
    /// SDIO specification version 2.00
    Specification2_00 = 3,
    /// SDIO specification version 3.00
    Specification3_00 = 4,
}

impl From<u8> for SdioSpecification {
    fn from(val: u8) -> Self {
        match val {
            0 => SdioSpecification::Specification1_00,
            1 => SdioSpecification::Specification1_10,
            2 => SdioSpecification::Specification1_20,
            3 => SdioSpecification::Specification2_00,
            4 => SdioSpecification::Specification3_00,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

pub struct CccrSdioRevisionRegister {
    pub val: u8,
}

impl RegisterAddress for CccrSdioRevisionRegister {
    fn address() -> u8 {
        0x00u8
    }
}

impl CccrSdioRevisionRegister {
    pub fn set_cccr_revision(&mut self, revision: CccrRevision) {
        self.val.set_bits(0..3, revision as u8);
    }

    pub fn cccr_revision(&self) -> CccrRevision {
        self.val.get_bits(0..3).into()
    }

    pub fn set_sdio_specification_version(&mut self, version: SdioSpecification) {
        self.val.set_bits(4..7, version as u8);
    }

    pub fn sdio_specification_verison(&self) -> SdioSpecification {
        self.val.get_bits(4..7).into()
    }
}
