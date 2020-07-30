use crate::sd_mmc::sdio::registers::registers::Register;

pub struct BlockSize {
    pub val: u8
}

impl Register<u8> for BlockSize {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0x10u8
    }
}