use crate::registers::register::Register;

pub struct CisPointerRegister {
    pub val: u8,
}

impl Register<u8> for CisPointerRegister {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0x09u8
    }
}
