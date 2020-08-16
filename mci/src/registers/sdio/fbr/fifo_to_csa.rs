use crate::registers::register::Register;

/// Read/Write fifo to CSA
pub struct FifoToCsaReadWriteRegister {
    pub val: u8,
}

impl Register for FifoToCsaReadWriteRegister {
    fn address() -> u8 {
        0xF
    }
}
