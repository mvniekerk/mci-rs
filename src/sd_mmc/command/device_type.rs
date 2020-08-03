pub trait SdDeviceType {}
pub trait MmcDeviceType {}
pub trait SdioDeviceType {}

pub struct SdDevice {}
impl SdDeviceType for SdDevice {}

pub struct MmcDevice {}
impl MmcDeviceType for MmcDevice {}

pub struct SdMmcDevice {}
impl SdDeviceType for SdMmcDevice {}
impl MmcDeviceType for SdMmcDevice {}

pub struct SdioDevice {}
impl SdioDeviceType for SdioDevice {}
impl SdDeviceType for SdioDevice {}
impl MmcDeviceType for SdioDevice {}