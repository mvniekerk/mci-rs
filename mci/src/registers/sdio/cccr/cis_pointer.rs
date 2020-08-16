use crate::registers::register::RegisterAddress;

pub struct CisPointerRegister {
    pub val: u8,
}

impl RegisterAddress for CisPointerRegister {
    fn address() -> u8 {
        0x09u8
    }
}
