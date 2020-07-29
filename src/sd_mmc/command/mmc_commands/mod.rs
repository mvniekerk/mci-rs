use bit_field::BitField;
use crate::sd_mmc::mode::ModeIndex;

pub struct Cmd6 {
    pub val: u32
}

pub enum Access {
    CommandSet = 0,
    SetBits = 1,
    ClearBits = 2,
    WriteByte = 3
}

pub enum BusWidth {
    _1BIT = 0,
    _4BIT = 1,
    _8BIT = 2
}

impl Cmd6 {
    pub fn new() -> Cmd6 {
        Cmd6 { val: 0 }
    }

    pub fn set_access(&mut self, access: Access) {
        self.val.set_bits(24..25, access as u32);
    }

    pub fn set_mode_index(&mut self, mode: ModeIndex) {
        self.val.set_bits(16..23, mode as u32);
    }

    pub fn set_bus_width(&mut self, bus_width: BusWidth) {
        self.val.set_bits(8..15, bus_width as u32);
    }

    pub fn set_hs_timing_enable(&mut self, enabled: bool) {
        self.val.set_bits(8..15, enabled as u32);
    }

    pub fn set_cmd(&mut self, cmd: u8) {
        self.val.set_bits(0..2, cmd as u32);
    }
}

impl From<Cmd6> for u32 {
    #[inline(always)]
    fn from(val: Cmd6) -> Self {
        val.val
    }
}