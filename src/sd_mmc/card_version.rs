
pub enum SdCardVersion {
    // version 1.0 and 1.01
    Sd_1_0  = 0x10,
    // version 1.10
    Sd_1_10 = 0x1A,
    // SD version 2.00
    Sd_2_0  = 0x20,
    // SD version 3.0X
    Sd_3_0  = 0x30,
}

pub enum MmcVersion {
    // MMC version 1.2
    Mmc_1_2 = 0x12,
    // MMC version 1.4
    Mmc_1_4 = 0x14,
    // MMC version 2.2
    Mmc_2_2 = 0x22,
    // MMC version 3
    Mmc_3   = 0x30,
    // MMC version 4
    Mmc_4   = 0x40
}

pub enum CardVersion {
    // Unknown card version
    Unknown,
    SdCard(SdCardVersion),
    Mmc(MmcVersion)
}