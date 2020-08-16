use crate::registers::register::Register;

pub struct CisPointerRegister {
    pub val: u8,
}

impl Register for CisPointerRegister {
    fn address() -> u8 {
        0x09u8
    }
}
