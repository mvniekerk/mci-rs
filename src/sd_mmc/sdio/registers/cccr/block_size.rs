use crate::sd_mmc::sdio::registers::registers::RegisterU8;

pub struct BlockSize {
    pub val: u8
}

impl RegisterU8 for BlockSize {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0x10u8
    }
}