use crate::registers::register::Register;
use bit_field::BitField;

pub struct Cmd6 {
    pub val: u32,
}

#[derive(PartialEq)]
pub enum Cmd6Mode {
    Check = 0,
    Switch = 1,
}

impl From<Cmd6Mode> for bool {
    fn from(val: Cmd6Mode) -> Self {
        (val as isize) == 1
    }
}

impl From<bool> for Cmd6Mode {
    fn from(val: bool) -> Self {
        if val {
            Cmd6Mode::Switch
        } else {
            Cmd6Mode::Check
        }
    }
}

impl Register<u32> for Cmd6 {
    fn value(&self) -> u32 {
        self.val
    }

    fn address() -> u8 {
        6
    }
}

impl Cmd6 {
    pub fn set_function_group_1_access_mode(&mut self, high_speed: bool) -> &mut Self {
        self.val.set_bits(0..=3, high_speed as u32);
        self
    }

    pub fn function_group_1_access_mode_high_speed(&self) -> bool {
        self.val.get_bits(0..=3) > 0
    }

    pub fn set_function_group2_command_system(&mut self, no_influence: bool) -> &mut Self {
        self.val
            .set_bits(4..=7, if no_influence { 0xF } else { 0x0 });
        self
    }

    pub fn function_group2_command_system(&self) -> bool {
        self.val.get_bits(4..=7) == 0xF
    }

    pub fn set_function_group3(&mut self, no_influence: bool) -> &mut Self {
        self.val
            .set_bits(8..=11, if no_influence { 0xF } else { 0x0 });
        self
    }

    pub fn function_group3(&self) -> bool {
        self.val.get_bits(8..=11) == 0xF
    }

    pub fn set_function_group4(&mut self, no_influence: bool) -> &mut Self {
        self.val
            .set_bits(12..=15, if no_influence { 0xF } else { 0x0 });
        self
    }

    pub fn function_group4(&self) -> bool {
        self.val.get_bits(12..=15) == 0xF
    }

    pub fn set_function_group5(&mut self, no_influence: bool) -> &mut Self {
        self.val
            .set_bits(16..=19, if no_influence { 0xF } else { 0x0 });
        self
    }

    pub fn function_group5(&self) -> bool {
        self.val.get_bits(16..=19) == 0xF
    }

    pub fn set_function_group6(&mut self, no_influence: bool) -> &mut Self {
        self.val
            .set_bits(20..=23, if no_influence { 0xF } else { 0x0 });
        self
    }

    pub fn function_group6(&self) -> bool {
        self.val.get_bits(20..=23) == 0xF
    }

    pub fn set_mode(&mut self, mode: Cmd6Mode) -> &mut Self {
        self.val.set_bit(31, mode.into());
        self
    }

    pub fn mode(&self) -> Cmd6Mode {
        self.val.get_bit(31).into()
    }
}
