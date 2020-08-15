use crate::sd_mmc::registers::register::Register;

/// Read/Write fifo to CSA
pub struct FifoToCsaReadWriteRegister {
    pub val: u8,
}

impl Register<u8> for FifoToCsaReadWriteRegister {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0xF
    }
}
