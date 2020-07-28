pub enum CardType {
    Unknown         = 0x0,
    Sd              = 0x1,
    Mmc             = 0x2,
    Sdio            = 0x4,
    HighCapacity    = 0x8,
    IoMemoryCombo   = 0x1 | 0x4
}