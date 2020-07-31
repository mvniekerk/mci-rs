use crate::sd_mmc::registers::registers::Register;

pub struct CsdRegister {
    pub val: u128
}

impl Register<u128> for CsdRegister {
    fn value(&self) -> u128 {
        self.val
    }

    fn address() -> u8 {
        0x10u8
    }
}