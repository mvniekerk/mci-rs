use core::hint::unreachable_unchecked;

#[derive(PartialEq)]
pub enum SdCardVersion {
    // version 1.0 and 1.01
    Sd_1_0  = 0x10,
    // version 1.10
    Sd_1_10 = 0x1A,
    // SD version 2.00
    Sd_2_0  = 0x20,
    // SD version 3.0X
    SdMmc_3_0  = 0x30,
}

#[derive(PartialEq)]
pub enum MmcVersion {
    // MMC version 1.2
    Mmc_1_2 = 0x12,
    // MMC version 1.4
    Mmc_1_4 = 0x14,
    // MMC version 2.2
    Mmc_2_2 = 0x22,
    // MMC version 3
    SdMmc_3_0   = 0x30,
    // MMC version 4
    Mmc_4   = 0x40
}

impl Into<Option<MmcVersion>> for SdCardVersion {
    fn into(self) -> Option<MmcVersion> {
        if self == SdCardVersion::SdMmc_3_0 {
            Some(MmcVersion::SdMmc_3_0)
        } else {
            None
        }
    }
}

pub enum CardVersion {
    // Unknown card version
    Unknown,
    SdCard(SdCardVersion),
    Mmc(MmcVersion)
}

impl From<u8> for CardVersion {
    fn from(val: u8) -> Self {
        match val {
            0x00 => CardVersion::Unknown,
            0x10 => CardVersion::SdCard(SdCardVersion::Sd_1_0),
            0x12 => CardVersion::Mmc(MmcVersion::Mmc_1_2),
            0x14 => CardVersion::Mmc(MmcVersion::Mmc_1_4),
            0x1A => CardVersion::SdCard(SdCardVersion::Sd_1_10),
            0x20 => CardVersion::SdCard(SdCardVersion::Sd_2_0),
            0x22 => CardVersion::Mmc(MmcVersion::Mmc_2_2),
            0x30 => CardVersion::SdCard(SdCardVersion::SdMmc_3_0),
            0x40 => CardVersion::Mmc(MmcVersion::Mmc_4),
            _ => unsafe { unreachable_unchecked() }
        }
    }
}