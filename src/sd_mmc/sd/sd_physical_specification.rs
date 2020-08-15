pub enum SdPhysicalSpecification {
    /// SD 1.01 (March 2000)
    Revision1d01 = 0,
    /// SD 1.10 (October 2004)
    Revision1d10 = 1,
    /// SD 2.00 (May 2006)
    Revision2d00 = 2,
    /// SD 3.00
    Revision3d00 = 3
}

impl From<u8> for SdPhysicalSpecification {
    fn from(val: u8) -> Self {
        match val {
            0 => SdPhysicalSpecification::Revision1d01,
            1 => SdPhysicalSpecification::Revision1d10,
            2 => SdPhysicalSpecification::Revision2d00,
            3 => SdPhysicalSpecification::Revision3d00,
            _ => unreachable!()
        }
    }
}
