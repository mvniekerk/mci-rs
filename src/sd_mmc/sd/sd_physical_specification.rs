use std::hint::unreachable_unchecked;

pub enum SdPhysicalSpecification {
    /// SD 1.01 (March 2000)
    Revision1_01 = 0,
    /// SD 1.10 (October 2004)
    Revision1_10 = 1,
    /// SD 2.00 (May 2006)
    Revision2_00 = 2,
    /// SD 3.00
    Revision3_00 = 3
}

impl From<u8> for SdPhysicalSpecification {
    fn from(val: u8) -> Self {
        match val {
            0 => SdPhysicalSpecification::Revision1_01,
            1 => SdPhysicalSpecification::Revision1_10,
            2 => SdPhysicalSpecification::Revision2_00,
            3 => SdPhysicalSpecification::Revision3_00,
            _ => unsafe { unreachable_unchecked() }
        }
    }
}
