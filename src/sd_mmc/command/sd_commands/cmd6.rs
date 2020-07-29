//
// //! \name SD CMD6 argument structure
// //! @{
// //! CMD6 arg[ 3: 0] function group 1, access mode
// #define SD_CMD6_GRP1_HIGH_SPEED (0x1lu << 0)
// #define SD_CMD6_GRP1_DEFAULT (0x0lu << 0)
// //! CMD6 arg[ 7: 4] function group 2, command system
// #define SD_CMD6_GRP2_NO_INFLUENCE (0xFlu << 4)
// #define SD_CMD6_GRP2_DEFAULT (0x0lu << 4)
// //! CMD6 arg[11: 8] function group 3, 0xF or 0x0
// #define SD_CMD6_GRP3_NO_INFLUENCE (0xFlu << 8)
// #define SD_CMD6_GRP3_DEFAULT (0x0lu << 8)
// //! CMD6 arg[15:12] function group 4, 0xF or 0x0
// #define SD_CMD6_GRP4_NO_INFLUENCE (0xFlu << 12)
// #define SD_CMD6_GRP4_DEFAULT (0x0lu << 12)
// //! CMD6 arg[19:16] function group 5, 0xF or 0x0
// #define SD_CMD6_GRP5_NO_INFLUENCE (0xFlu << 16)
// #define SD_CMD6_GRP5_DEFAULT (0x0lu << 16)
// //! CMD6 arg[23:20] function group 6, 0xF or 0x0
// #define SD_CMD6_GRP6_NO_INFLUENCE (0xFlu << 20)
// #define SD_CMD6_GRP6_DEFAULT (0x0lu << 20)
// //! CMD6 arg[30:24] reserved 0
// //! CMD6 arg[31   ] Mode, 0: Check, 1: Switch
// #define SD_CMD6_MODE_CHECK (0lu << 31)
// #define SD_CMD6_MODE_SWITCH (1lu << 31)

use bit_field::BitField;

pub struct Cmd6 {
    pub val: u32
}

#[derive(PartialEq)]
pub enum Cmd6Mode {
    Check = 0,
    Switch = 1
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

impl Cmd6 {
    pub fn set_function_group_1_access_mode(&mut self, high_speed: bool) {
        self.val.set_bits(0..3, high_speed as u32);
    }

    pub fn function_group_1_access_mode(&self) -> bool {
        self.val.get_bits(0..3) > 0
    }

    pub fn set_function_group2_command_system(&mut self, default_influence: bool) {
        self.val.set_bits(4..7, if default_influence { 0xF } else { 0x0 });
    }

    pub fn function_group2_command_system(&self) -> bool {
        self.val.get_bits(4..7) == 0xF
    }

    pub fn set_function_group3(&mut self, default_influence: bool) {
        self.val.set_bits(8..11, if default_influence { 0xF } else { 0x0 });
    }

    pub fn function_group3(&self) -> bool {
        self.val.get_bits(8..1) == 0xF
    }

    pub fn set_function_group4(&mut self, default_influence: bool) {
        self.val.set_bits(12..15, if default_influence { 0xF } else { 0x0 });
    }

    pub fn function_group4(&self) -> bool {
        self.val.get_bits(12..15) == 0xF
    }

    pub fn set_function_group5(&mut self, default_influence: bool) {
        self.val.set_bits(16..19, if default_influence { 0xF } else { 0x0 });
    }

    pub fn function_group5(&self) -> bool {
        self.val.get_bits(16..19) == 0xF
    }

    pub fn set_function_group6(&mut self, default_influence: bool) {
        self.val.set_bits(20..23, if default_influence { 0xF } else { 0x0 });
    }

    pub fn function_group6(&self) -> bool {
        self.val.get_bits(20..23) == 0xF
    }

    pub fn set_mode(&mut self, mode: Cmd6Mode) {
        self.val.set_bit(31, mode.into());
    }

    pub fn mode(&self) -> Cmd6Mode {
        self.val.get_bit(31).into()
    }
}

