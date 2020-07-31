use std::hint::unreachable_unchecked;

pub enum SdBusWidth {
    /// 1-bit data bus
    _1bit = 0,
    /// 4-bit data bus
    _4bit = 4
}

impl From<u8> for SdBusWidth {
    fn from(val: u8) -> Self {
        match val {
            1 => SdBusWidth::_1bit,
            4 => SdBusWidth::_4bit,
            _ => unsafe { unreachable_unchecked() }
        }
    }
}