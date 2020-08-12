use crate::sd_mmc::registers::registers::Register;
use bit_field::BitField;
use core::hint::unreachable_unchecked;

pub struct OcrRegister {
    pub val: u32
}

impl Register<u32> for OcrRegister {
    fn value(&self) -> u32 {
        self.val
    }

    fn address() -> u8 {
        0x0
    }
}

#[derive(PartialEq)]
pub enum AccessMode {
    /// Byte access mode
    Byte = 0,
    /// Sector access mode
    Sector = 2
}

impl From<u32> for AccessMode {
    fn from(val: u32) -> Self {
        match val {
            0 => AccessMode::Byte,
            2 => AccessMode::Sector,
            _ => unsafe { unreachable_unchecked() }
        }
    }
}

impl OcrRegister {
    pub fn set_vdd_170_195(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(7, enabled);
        self
    }
    
    pub fn vdd_170_195(&self) -> bool {
        self.val.get_bit(7)
    }

    pub fn set_vdd_20_21(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(8, enabled);
        self
    }
    
    pub fn vdd_20_21(&self) -> bool {
        self.val.get_bit(8)
    }

    pub fn set_vdd_21_22(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(9, enabled);
        self
    }
    
    pub fn vdd_21_22(&self) -> bool {
        self.val.get_bit(9)
    }

    pub fn set_vdd_22_23(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(10, enabled);
        self
    }
    
    pub fn vdd_22_23(&self) -> bool {
        self.val.get_bit(10)
    }

    pub fn set_vdd_23_24(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(11, enabled);
        self
    }
    
    pub fn vdd_23_24(&self) -> bool {
        self.val.get_bit(11)
    }

    pub fn set_vdd_24_25(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(12, enabled);
        self
    }
    
    pub fn vdd_24_25(&self) -> bool {
        self.val.get_bit(12)
    }

    pub fn set_vdd_25_26(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(13, enabled);
        self
    }
    
    pub fn vdd_25_26(&self) -> bool {
        self.val.get_bit(13)
    }

    pub fn set_vdd_26_27(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(14, enabled);
        self
    }
    
    pub fn vdd_26_27(&self) -> bool {
        self.val.get_bit(14)
    }

    pub fn set_vdd_27_28(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(15, enabled);
        self
    }
    
    pub fn vdd_27_28(&self) -> bool {
        self.val.get_bit(15)
    }

    pub fn set_vdd_28_29(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(16, enabled);
        self
    }
    
    pub fn vdd_28_29(&self) -> bool {
        self.val.get_bit(16)
    }

    pub fn set_vdd_29_30(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(17, enabled);
        self
    }
    
    pub fn vdd_29_30(&self) -> bool {
        self.val.get_bit(17)
    }

    pub fn set_vdd_30_31(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(18, enabled);
        self
    }
    
    pub fn vdd_30_31(&self) -> bool {
        self.val.get_bit(18)
    }

    pub fn set_vdd_31_32(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(19, enabled);
        self
    }
    
    pub fn vdd_31_32(&self) -> bool {
        self.val.get_bit(19)
    }

    pub fn set_vdd_32_33(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(20, enabled);
        self
    }
    
    pub fn vdd_32_33(&self) -> bool {
        self.val.get_bit(20)
    }

    pub fn set_vdd_33_34(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(21, enabled);
        self
    }
    
    pub fn vdd_33_34(&self) -> bool {
        self.val.get_bit(21)
    }

    pub fn set_vdd_34_35(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(22, enabled);
        self
    }
    
    pub fn vdd_34_35(&self) -> bool {
        self.val.get_bit(22)
    }

    pub fn set_vdd_35_36(&mut self, enabled: bool) -> &mut Self {
        self.val.set_bit(23, enabled);
        self
    }
    
    pub fn vdd_35_36(&self) -> bool {
        self.val.get_bit(23)
    }

    /// Set if switching to 1.8V accepted
    pub fn set_switching_to_1_8v_accepted(&mut self, accepted: bool) -> &mut Self {
        self.val.set_bit(24, accepted);
        self
    }

    /// Switching to 1.8V accepted
    pub fn switching_to_1_8v_accepted(&self) -> bool {
        self.val.get_bit(24)
    }

    /// Set if memory is present
    pub fn set_memory_present(&mut self, present: bool) -> &mut Self {
        self.val.set_bit(27, present);
        self
    }

    /// Memory is present
    pub fn memory_present(&self) -> bool {
        self.val.get_bit(27)
    }

    /// Set if number of I/O functions is available
    pub fn set_number_of_io_functions(&mut self, available: bool) -> &mut Self {
        self.val.set_bit(28, available);
        self
    }

    /// If number of I/O functions is available
    pub fn number_of_io_functions(&self) -> bool {
        self.val.get_bit(28)
    }

    /// Set access mode - MMC card
    pub fn set_access_mode(&mut self, mode: AccessMode) -> &mut Self {
        self.val.set_bits(29..=30, mode as u32);
        self
    }

    /// Access mode - MMC card
    pub fn access_mode(&self) -> AccessMode {
        self.val.get_bits(29..=30).into()
    }

    /// Set card capacity status bit - SD card
    pub fn set_card_capacity_status(&mut self, available: bool) -> &mut Self {
        self.val.set_bit(30, available);
        self
    }

    /// Card capacity status bit - SD card
    pub fn card_capacity_status(&self) -> bool {
        self.val.get_bit(30)
    }

    /// Set card powered up status bit
    pub fn set_card_powered_up_status(&mut self, powered_up: bool) -> &mut Self {
        self.val.set_bit(31, powered_up);
        self
    }

    /// Card powered up status bit
    pub fn card_powered_up_status(&self) -> bool {
        self.val.get_bit(31)
    }
}