use bit_field::BitField;
use crate::sd_mmc::mode_index::ModeIndex;
use core::hint::unreachable_unchecked;

#[derive(Default)]
pub struct Cmd6 {
    pub val: u32
}

pub enum Access {
    CommandSet = 0,
    SetBits = 1,
    ClearBits = 2,
    WriteByte = 3
}

impl From<u32> for Access {
    fn from(val: u32) -> Self {
        match val {
            0 => Access::CommandSet,
            1 => Access::SetBits,
            2 => Access::ClearBits,
            3 => Access::WriteByte,
            _ => Access::CommandSet
        }
    }
}

pub enum BusWidth {
    _1BIT = 0,
    _4BIT = 1,
    _8BIT = 2
}

impl From<u32> for BusWidth {
    fn from(val: u32) -> Self {
        match val {
            0 => BusWidth::_1BIT,
            1 => BusWidth::_4BIT,
            2 => BusWidth::_8BIT,
            _ => unsafe { unreachable_unchecked() }
        }
    }
}

impl Cmd6 {
    pub fn new() -> Cmd6 {
        Cmd6 { val: 0 }
    }

    pub fn set_access(&mut self, access: Access) -> &mut Self {
        self.val.set_bits(24..=25, access as u32);
        self
    }

    pub fn access(&self) -> Access {
        self.val.get_bits(24..=25).into()
    }

    pub fn set_mode_index(&mut self, mode: ModeIndex) -> &mut Self {
        self.val.set_bits(16..=23, mode as u32);
        self
    }

    pub fn mode_index(&self) -> ModeIndex {
        self.val.get_bits(16..=23).into()
    }

    pub fn set_bus_width(&mut self, bus_width: &BusWidth) -> &mut Self {
        self.val.set_bits(8..=15, bus_width as u32);
        self
    }

    pub fn bus_width(&self) -> BusWidth {
        self.val.get_bits(8..=15).into()
    }

    pub fn set_hs_timing_enable(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bits(8..=15, enabled as u32);
        self
    }

    pub fn hs_timing_enabled(&self) -> bool {
        self.val.get_bits(8..=15) > 0
    }

    pub fn set_cmd(&mut self, cmd: u8) -> &mut Self {
        self.val.set_bits(0..=2, cmd as u32);
        self
    }

    pub fn cmd(&self) -> u8 {
        self.val.get_bits(0..=2) as u8
    }
}

impl From<Cmd6> for u32 {
    #[inline(always)]
    fn from(val: Cmd6) -> Self {
        val.val
    }
}