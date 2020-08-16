use crate::registers::register::RegisterAddress;

pub struct ExtendedInterfaceCodeRegister {
    pub val: u8,
}

impl RegisterAddress for ExtendedInterfaceCodeRegister {
    fn address() -> u8 {
        0x1
    }
}
