use core::hint::unreachable_unchecked;

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub enum SdCardVersion {
    // version 1.0 and 1.01
    Sd1d0 = 0x10,
    // version 1.10
    Sd1d10 = 0x1A,
    // SD version 2.00
    Sd2d0 = 0x20,
    // SD version 3.0X
    SdMmc3d0 = 0x30,
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub enum MmcVersion {
    // MMC version 1.2
    Mmc1d2 = 0x12,
    // MMC version 1.4
    Mmc1d4 = 0x14,
    // MMC version 2.2
    Mmc2d2 = 0x22,
    // MMC version 3
    SdMmc3d0 = 0x30,
    // MMC version 4
    Mmc4d0 = 0x40
}

impl Into<Option<MmcVersion>> for SdCardVersion {
    fn into(self) -> Option<MmcVersion> {
        if self == SdCardVersion::SdMmc3d0 {
            Some(MmcVersion::SdMmc3d0)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub enum CardVersion {
    // Unknown card version
    Unknown,
    SdCard(SdCardVersion),
    Mmc(MmcVersion)
}

impl From<CardVersion> for usize {
    fn from(val: CardVersion) -> Self {
        match val {
            CardVersion::Unknown => 0,
            CardVersion::SdCard(sdcard) => sdcard as usize,
            CardVersion::Mmc(mmc) => mmc as usize
        }
    }
}

impl From<u8> for CardVersion {
    fn from(val: u8) -> Self {
        match val {
            0x00 => CardVersion::Unknown,
            0x10 => CardVersion::SdCard(SdCardVersion::Sd1d0),
            0x12 => CardVersion::Mmc(MmcVersion::Mmc1d2),
            0x14 => CardVersion::Mmc(MmcVersion::Mmc1d4),
            0x1A => CardVersion::SdCard(SdCardVersion::Sd1d10),
            0x20 => CardVersion::SdCard(SdCardVersion::Sd2d0),
            0x22 => CardVersion::Mmc(MmcVersion::Mmc2d2),
            0x30 => CardVersion::SdCard(SdCardVersion::SdMmc3d0),
            0x40 => CardVersion::Mmc(MmcVersion::Mmc4d0),
            _ => unsafe { unreachable_unchecked() }
        }
    }
}