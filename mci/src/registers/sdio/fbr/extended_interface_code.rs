use crate::registers::register_address::RegisterAddress;

pub struct ExtendedInterfaceCodeRegister {
    pub val: u8,
}

impl RegisterAddress for ExtendedInterfaceCodeRegister {
    fn address() -> u8 {
        0x1
    }
}
