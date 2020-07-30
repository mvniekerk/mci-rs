use crate::sd_mmc::sdio::registers::registers::Register;

pub struct CisPointerRegister {
    pub val: u8
}

impl Register for CisPointerRegister {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0x09u8
    }
}