use crate::sd_mmc::sdio::registers::registers::RegisterU8;

pub struct ExtendedInterfaceCodeRegister {
    pub val: u8
}

impl RegisterU8 for ExtendedInterfaceCodeRegister {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0x1
    }
}