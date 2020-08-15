use crate::sd_mmc::registers::registers::Register;
use crate::sd_mmc::sd::sd_physical_specification::SdPhysicalSpecification;
use bit_field::BitField;

pub struct SdPhysicalSpecificationRegister {
    pub val: u8,
}

impl Register<u8> for SdPhysicalSpecificationRegister {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0x01u8
    }
}

impl SdPhysicalSpecificationRegister {
    pub fn set_specification(&mut self, val: SdPhysicalSpecification) {
        self.val.set_bits(0..8, val as u8);
    }

    pub fn specification(&self) -> SdPhysicalSpecification {
        self.val.get_bits(0..8).into()
    }
}
