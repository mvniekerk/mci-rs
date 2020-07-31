use crate::sd_mmc::registers::registers::Register;
use bit_field::BitField;
use std::hint::unreachable_unchecked;
use crate::sd_mmc::sd::sd_physical_specification::SdPhysicalSpecification;
use crate::sd_mmc::sd::sd_security::SdSecurity;
use crate::sd_mmc::sd::sd_bus_width::SdBusWidth;

pub struct ScrRegister {
    pub val: u64
}

impl Register<u64> for ScrRegister {
    fn value(&self) -> u64 {
        self.val
    }

    fn address() -> u8 {
        0x0
    }
}

pub enum ScrRegisterStructureVersion {
    Version1_0 = 0
}

impl From<u64> for ScrRegisterStructureVersion {
    fn from(val: u64) -> Self {
        match val {
            0 => ScrRegisterStructureVersion::Version1_0,
            _ => unsafe { unreachable_unchecked() }
        }
    }
}

impl ScrRegister {
    pub fn set_structure_version(&mut self, version: ScrRegisterStructureVersion) {
        self.val.set_bits(60..63, version as u64);
    }

    pub fn structure_version(&self) -> ScrRegisterStructureVersion {
        self.val.get_bits(60..63).into()
    }

    pub fn set_sd_specification_version(&mut self, version: SdPhysicalSpecification) {
        self.val.set_bits(56..59, version as u64);
    }

    pub fn sd_specification_version(&mut self) -> SdPhysicalSpecification {
        (self.val.get_bits(56..59) as u8).into()
    }

    pub fn set_data_status_after_erase(&mut self, enabled: bool) {
        self.val.set_bit(55, enabled);
    }

    pub fn data_status_after_erase(&self) -> bool {
        self.val.get_bit(55)
    }

    pub fn set_sd_security_version(&mut self, version: SdSecurity) {
        self.val.set_bits(52..54, version as u64);
    }

    pub fn sd_security_version(&self) -> SdSecurity {
        (self.val.get_bits(52..54) as u8).into()
    }

    pub fn set_sd_bus_width(&mut self, bus_width: SdBusWidth) {
        self.val.set_bits(48..51, bus_width as u64);
    }

    pub fn sd_bus_width(&self) -> SdBusWidth {
        (self.val.get_bits(48..51) as u8).into()
    }

    pub fn set_is_spec3(&mut self, spec3: bool) {
        self.val.set_bit(47, spec3);
    }

    pub fn spec3(&self) -> bool {
        self.val.get_bit(47)
    }

    pub fn set_sd_extended_security(&mut self, val: u8) {
        self.val.set_bits(43..46, val as u64);
    }

    pub fn sd_extended_security(&self) -> u8 {
        self.val.get_bits(43..46) as u8
    }

    pub fn set_sd_command_support(&mut self, val: u8) {
        self.val.set_bits(32..33, val as u64);
    }

    pub fn sd_command_support(&self) -> u8 {
        self.val.get_bits(32..33) as u8
    }
}
