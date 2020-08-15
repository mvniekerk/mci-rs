use crate::sd_mmc::registers::registers::Register;
use bit_field::BitField;

pub struct CardCapabilityRegister {
    pub val: u8,
}

impl Register<u8> for CardCapabilityRegister {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        0x08
    }
}

impl CardCapabilityRegister {
    pub fn set_supports_direct_commands_during_transfer(&mut self, supports: bool) {
        self.val.set_bit(0, supports);
    }

    pub fn supports_direct_commands_during_transfer(&self) -> bool {
        self.val.get_bit(0)
    }

    pub fn set_supports_multi_block(&mut self, supports: bool) {
        self.val.set_bit(1, supports);
    }

    pub fn supports_multi_block(&self) -> bool {
        self.val.get_bit(1)
    }

    pub fn set_supports_read_wait(&mut self, supports: bool) {
        self.val.set_bit(2, supports);
    }

    pub fn supports_read_wait(&self) -> bool {
        self.val.get_bit(2)
    }

    pub fn set_supports_suspend_resume(&mut self, supports: bool) {
        self.val.set_bit(3, supports);
    }

    pub fn supports_suspend_resume(&self) -> bool {
        self.val.get_bit(3)
    }

    pub fn set_supports_interrupt_between_blocks_of_data_in_4bit_sd_mode(
        &mut self,
        supports: bool,
    ) {
        self.val.set_bit(4, supports);
    }

    pub fn supports_interrupt_between_blocks_of_data_in_4bit_mode(&self) -> bool {
        self.val.get_bit(4)
    }

    pub fn set_enable_interrupt_between_blocks_of_data_in_4bit_sd_mode(&mut self, enabled: bool) {
        self.val.set_bit(5, enabled);
    }

    pub fn enable_interrupt_between_blocks_of_data_in_4bit_sd_mode(&self) -> bool {
        self.val.get_bit(5)
    }

    pub fn set_is_low_speed_card(&mut self, low_speed: bool) {
        self.val.set_bit(6, low_speed);
    }

    pub fn is_low_speed_card(&self) -> bool {
        self.val.get_bit(6)
    }

    pub fn set_low_speed_card_supports_4bit_mode(&mut self, supports: bool) {
        self.val.set_bit(7, supports);
    }

    pub fn low_speed_card_supports_4bit_mode(&self) -> bool {
        self.val.get_bit(7)
    }
}
