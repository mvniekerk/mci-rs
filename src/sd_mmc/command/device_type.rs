pub trait SdDeviceType {}
pub trait MmcDeviceType {}
pub trait SdioDeviceType {}

pub struct SdDevice {}
impl SdDeviceType for SdDevice {}

#[cfg(feature = "mmc")]
pub struct MmcDevice {}
#[cfg(feature = "mmc")]
impl MmcDeviceType for MmcDevice {}

pub struct SdMmcDevice {}
impl SdDeviceType for SdMmcDevice {}
#[cfg(feature = "mmc")]
impl MmcDeviceType for SdMmcDevice {}

#[cfg(feature = "sdio")]
pub struct SdioDevice {}

#[cfg(feature = "sdio")]
impl SdioDeviceType for SdioDevice {}

#[cfg(feature = "sdio")]
impl SdDeviceType for SdioDevice {}

#[cfg(feature = "sdio")]
impl MmcDeviceType for SdioDevice {}