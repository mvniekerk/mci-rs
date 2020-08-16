use crate::registers::register::Register;

pub struct ExtendedInterfaceCodeRegister {
    pub val: u8,
}

impl Register for ExtendedInterfaceCodeRegister {
    fn address() -> u8 {
        0x1
    }
}
