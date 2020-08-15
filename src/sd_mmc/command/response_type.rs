// Have response MCI only
pub trait ResponsePresent {}
// 8 bit response = SPI only
pub trait Response8 {}
// 32 bit response = SPI only,
pub trait Response32 {}
// 136 bit response = MCI only
pub trait Response136 {}
// Card may send busy
pub trait ResponseBusy {}
// Expect valid crc = MCI only
pub trait ResponseCrc {}

pub trait Response {
    fn val(&self) -> u32;
}

pub struct NoResponse;
impl Response for NoResponse {
    #[inline(always)]
    fn val(&self) -> u32 {
        ResponseType::NoResponse as u32
    }
}

#[derive(Default)]
pub struct CmdR1R6;
impl ResponsePresent for CmdR1R6 {}
impl ResponseCrc for CmdR1R6 {}
impl Response for CmdR1R6 {
    #[inline(always)]
    fn val(&self) -> u32 {
        ResponseType::CmdR1R6 as u32
    }
}

#[derive(Default)]
pub struct CmdR1B;
impl ResponsePresent for CmdR1B{}
impl ResponseBusy for CmdR1B{}
impl ResponseCrc for CmdR1B {}
impl Response for CmdR1B {
    #[inline(always)]
    fn val(&self) -> u32 {
        ResponseType::CmdR1B as u32
    }
}

#[derive(Default)]
pub struct CmdR2;
impl ResponsePresent for CmdR2 {}
impl Response8 for CmdR2 {}
impl Response136 for CmdR2 {}
impl ResponseCrc for CmdR2 {}
impl Response for CmdR2 {
    #[inline(always)]
    fn val(&self) -> u32 {
        ResponseType::CmdR2 as u32
    }
}

#[derive(Default)]
pub struct CmdR3R4;
impl ResponsePresent for CmdR3R4 {}
impl Response32 for CmdR3R4 {}
impl Response for CmdR3R4 {
    #[inline(always)]
    fn val(&self) -> u32 {
        ResponseType::CmdR3R4 as u32
    }
}

#[derive(Default)]
pub struct CmdR5;
impl ResponsePresent for CmdR5 {}
impl Response8 for CmdR5 {}
impl ResponseCrc for CmdR5 {}
impl Response for CmdR5 {
    #[inline(always)]
    fn val(&self) -> u32 {
        ResponseType::CmdR5 as u32
    }
}

#[derive(Default)]
pub struct CmdR7;
impl ResponsePresent for CmdR7 {}
impl Response32 for CmdR7 {}
impl ResponseCrc for CmdR7 {}
impl Response for CmdR7 {
    #[inline(always)]
    fn val(&self) -> u32 {
        ResponseType::CmdR7 as u32
    }
}

pub enum ResponseFlags {
    ResponsePresent     = 1 << 8,
    Response8           = 1 << 9,
    Response32          = 1 << 10,
    Response136         = 1 << 11,
    ResponseCrc         = 1 << 12,
    ResponseBusy        = 1 << 13,
}

pub enum ResponseType {
    NoResponse = 0,
    CmdR1R6     = ResponseFlags::ResponsePresent as isize | ResponseFlags::ResponseCrc as isize,
    CmdR1B      = ResponseFlags::ResponsePresent as isize | ResponseFlags::ResponseCrc as isize | ResponseFlags::ResponseBusy as isize,
    CmdR2       = ResponseFlags::ResponsePresent as isize | ResponseFlags::Response8 as isize | ResponseFlags::Response136 as isize | ResponseFlags::ResponseCrc as isize,
    CmdR3R4     = ResponseFlags::ResponsePresent as isize | ResponseFlags::Response32 as isize,
    CmdR5       = ResponseFlags::ResponsePresent as isize | ResponseFlags::Response8 as isize | ResponseFlags::ResponseCrc as isize,
    CmdR7       = ResponseFlags::ResponsePresent as isize | ResponseFlags::Response32 as isize | ResponseFlags::ResponseCrc as isize
}