use crate::registers::register_address::RegisterAddress;

pub struct CisPointerRegister {
    pub val: u8,
}

impl RegisterAddress for CisPointerRegister {
    fn address() -> u8 {
        0x09u8
    }
}
