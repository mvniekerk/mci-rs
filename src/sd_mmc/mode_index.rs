use core::hint::unreachable_unchecked;

pub enum ModeIndex {
    EraseGroupDef = 0xAF,
    BootBusWidth = 0xB1,
    BootConfig = 0xB3,
    ErasedMemCont = 0xB5,
    BusWidth = 0xB7,
    HsTimingIndex = 0xB9,
    PowerClass = 0xBB,
    SetRev = 0xBD,
    Set = 0xBF,
}

impl From<u32> for ModeIndex {
    fn from(val: u32) -> Self {
        match val {
            0xAF => ModeIndex::EraseGroupDef,
            0xB1 => ModeIndex::BootBusWidth,
            0xB3 => ModeIndex::BootConfig,
            0xB5 => ModeIndex::ErasedMemCont,
            0xB7 => ModeIndex::BusWidth,
            0xB9 => ModeIndex::HsTimingIndex,
            0xBB => ModeIndex::PowerClass,
            0xBD => ModeIndex::SetRev,
            0xBF => ModeIndex::Set,
            _ => unsafe { unreachable_unchecked() }
        }
    }
}