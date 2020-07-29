pub trait SpiModeType {}
pub trait MciModeType {}
pub trait DontCareModeType {}

pub struct SpiMode {}
impl SpiModeType for SpiMode {}
impl DontCareModeType for SpiMode {}

pub struct MciMode {}
impl MciModeType for MciMode {}
impl DontCareModeType for MciMode {}

pub struct DontCareMode {}
impl MciModeType for DontCareMode {}
impl SpiModeType for DontCareMode {}
impl DontCareModeType for DontCareMode {}