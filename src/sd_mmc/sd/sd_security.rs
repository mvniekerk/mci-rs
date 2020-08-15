use core::hint::unreachable_unchecked;

pub enum SdSecurity {
    None = 0,
    NotUsed = 1,
    _1_01 = 2,
    _2_00 = 3,
    _3_00 = 4,
}

impl From<u8> for SdSecurity {
    fn from(val: u8) -> Self {
        match val {
            0 => SdSecurity::None,
            1 => SdSecurity::NotUsed,
            2 => SdSecurity::_1_01,
            3 => SdSecurity::_2_00,
            4 => SdSecurity::_3_00,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}
