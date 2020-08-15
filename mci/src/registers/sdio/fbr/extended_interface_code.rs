use crate::registers::register::Register;

pub struct ExtendedInterfaceCodeRegister {
    pub val: u8,
}

impl Register<u8> for ExtendedInterfaceCodeRegister {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0x1
    }
}
