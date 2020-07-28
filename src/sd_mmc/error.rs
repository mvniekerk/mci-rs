pub enum SdMmcError {
    InitOngoing = 1,
    NoCard = 2,
    Unusable = 3,
    Slot = 4,
    Communication = 5,
    IllegalParameter = 6,
    WriteProtected = 7
}
