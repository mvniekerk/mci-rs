use crate::registers::register::Register;
use crate::sd::sd_bus_width::SdBusWidth;
use crate::sd::sd_physical_specification::SdPhysicalSpecification;
use crate::sd::sd_security::SdSecurity;
use bit_field::BitField;
use core::hint::unreachable_unchecked;

pub struct ScrRegister {
    pub val: u64,
}

impl From<[u8; 8]> for ScrRegister {
    fn from(val: [u8; 8]) -> Self {
        ScrRegister {
            val: (val[0] as u64)
                + ((val[1] as u64) << 8)
                + ((val[2] as u64) << 16)
                + ((val[3] as u64) << 24)
                + ((val[4] as u64) << 32)
                + ((val[5] as u64) << 40)
                + ((val[6] as u64) << 48)
                + ((val[7] as u64) << 56),
        }
    }
}

pub enum ScrRegisterStructureVersion {
    Version1_0 = 0,
}

impl From<u64> for ScrRegisterStructureVersion {
    fn from(val: u64) -> Self {
        match val {
            0 => ScrRegisterStructureVersion::Version1_0,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

impl ScrRegister {
    pub fn set_structure_version(&mut self, version: ScrRegisterStructureVersion) {
        self.val.set_bits(60..=63, version as u64);
    }

    pub fn structure_version(&self) -> ScrRegisterStructureVersion {
        self.val.get_bits(60..=63).into()
    }

    pub fn set_sd_specification_version(&mut self, version: SdPhysicalSpecification) {
        self.val.set_bits(56..=59, version as u64);
    }

    pub fn sd_specification_version(&self) -> SdPhysicalSpecification {
        (self.val.get_bits(56..=59) as u8).into()
    }

    pub fn set_data_status_after_erase(&mut self, enabled: bool) {
        self.val.set_bit(55, enabled);
    }

    pub fn data_status_after_erase(&self) -> bool {
        self.val.get_bit(55)
    }

    pub fn set_sd_security_version(&mut self, version: SdSecurity) {
        self.val.set_bits(52..=54, version as u64);
    }

    pub fn sd_security_version(&self) -> SdSecurity {
        (self.val.get_bits(52..=54) as u8).into()
    }

    pub fn set_sd_bus_width(&mut self, bus_width: SdBusWidth) {
        self.val.set_bits(48..=51, bus_width as u64);
    }

    pub fn sd_bus_width(&self) -> SdBusWidth {
        (self.val.get_bits(48..=51) as u8).into()
    }

    pub fn set_is_spec3(&mut self, spec3: bool) {
        self.val.set_bit(47, spec3);
    }

    pub fn spec3(&self) -> bool {
        self.val.get_bit(47)
    }

    pub fn set_sd_extended_security(&mut self, val: u8) {
        self.val.set_bits(43..=46, val as u64);
    }

    pub fn sd_extended_security(&self) -> u8 {
        self.val.get_bits(43..=46) as u8
    }

    pub fn set_sd_command_support(&mut self, val: u8) {
        self.val.set_bits(32..=33, val as u64);
    }

    pub fn sd_command_support(&self) -> u8 {
        self.val.get_bits(32..=33) as u8
    }
}
