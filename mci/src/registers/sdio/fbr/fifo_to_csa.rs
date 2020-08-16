use crate::registers::register_address::RegisterAddress;

/// Read/Write fifo to CSA
pub struct FifoToCsaReadWriteRegister {
    pub val: u8,
}

impl RegisterAddress for FifoToCsaReadWriteRegister {
    fn address() -> u8 {
        0xF
    }
}
