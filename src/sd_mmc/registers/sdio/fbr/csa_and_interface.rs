use std::hint::unreachable_unchecked;
use bit_field::BitField;
use crate::sd_mmc::registers::registers::Register;

pub enum SdioInterfaceFunctionCode {
    /// No SDIO standard interface
    None = 0x0,
    /// UART
    Uart = 0x1,
    /// Bluetooth Type A
    BluetoothA = 0x2,
    /// Bluetooth Type B
    BluetoothB = 0x3,
    /// GPS
    Gps = 0x4,
    /// Camera
    Camera = 0x5,
    /// Phs
    Phs = 0x6,
    /// WLAN
    Wlan = 0x7,
    /// Embedded Sdio ATA
    EmbeddedSdioAta = 0x8,
    /// Check EXT interface code
    CheckExt = 0xF
}

impl From<u8> for SdioInterfaceFunctionCode {
    fn from(val: u8) -> Self {
        match val {
            0x0 => SdioInterfaceFunctionCode::None,
            0x1 => SdioInterfaceFunctionCode::Uart,
            0x2 => SdioInterfaceFunctionCode::BluetoothA,
            0x3 => SdioInterfaceFunctionCode::BluetoothB,
            0x4 => SdioInterfaceFunctionCode::Gps,
            0x5 => SdioInterfaceFunctionCode::Camera,
            0x6 => SdioInterfaceFunctionCode::Phs,
            0x7 => SdioInterfaceFunctionCode::Wlan,
            0x8 => SdioInterfaceFunctionCode::EmbeddedSdioAta,
            0xF => SdioInterfaceFunctionCode::CheckExt,
            _ => unsafe { unreachable_unchecked() }
        }
    }
}

pub struct CsaAndInterfaceCodeRegister {
    pub val: u8
}

impl CsaAndInterfaceCodeRegister {
    pub fn set_interface_code(&mut self, code: SdioInterfaceFunctionCode) {
        self.val.set_bits(0..3, code as u8);
    }

    pub fn interface_code(&self) -> SdioInterfaceFunctionCode {
        self.val.get_bits(0..3).into()
    }

    pub fn set_supports_code_storage_area(&mut self, supports: bool) {
        self.val.set_bit(6, supports);
    }

    pub fn supports_code_storage_area(&self) -> bool {
        self.val.get_bit(6)
    }

    pub fn set_enable_code_storage_area(&mut self, enable: bool) {
        self.val.set_bit(7, enable);
    }

    pub fn enable_code_storage_area(&self) -> bool {
        self.val.get_bit(7)
    }
}

impl Register<u8> for CsaAndInterfaceCodeRegister {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0x0
    }
}