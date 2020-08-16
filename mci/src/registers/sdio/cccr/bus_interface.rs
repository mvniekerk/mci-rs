use crate::registers::register::RegisterAddress;
use bit_field::BitField;
use core::hint::unreachable_unchecked;

pub enum BusWidth {
    /// 1-bit data bus
    _1bit = 0,
    /// 4-bit data bus
    _4bit = 2,
}

impl From<u8> for BusWidth {
    fn from(val: u8) -> Self {
        match val {
            0 => BusWidth::_1bit,
            2 => BusWidth::_4bit,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

pub struct BusInterfaceControlRegister {
    pub val: u8,
}

impl RegisterAddress for BusInterfaceControlRegister {
    fn address() -> u8 {
        0x07u8
    }
}

impl BusInterfaceControlRegister {
    pub fn set_bus_width(&mut self, width: BusWidth) {
        self.val.set_bits(0..1, width as u8);
    }

    pub fn bus_width(&mut self) -> BusWidth {
        self.val.get_bits(0..1).into()
    }

    pub fn set_enable_continuous_spi_interrupt(&mut self, enabled: bool) {
        self.val.set_bit(5, enabled);
    }

    pub fn enable_continuous_spi_interrupt(&self) -> bool {
        self.val.get_bit(5)
    }

    pub fn set_supports_continuous_spi_interrupt(&mut self, enabled: bool) {
        self.val.set_bit(6, enabled);
    }

    pub fn supports_continuous_spi_interrupt(&self) -> bool {
        self.val.get_bit(6)
    }

    pub fn set_cd_dat3_pull_up(&mut self, pulled_up: bool) {
        self.val.set_bit(7, pulled_up);
    }

    pub fn cd_dat3_pull_up(&self) -> bool {
        self.val.get_bit(7)
    }
}
