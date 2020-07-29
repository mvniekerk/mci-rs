use bit_field::BitField;

pub enum SdioR5StateValue {
    Disabled = 0,
    CmdDatLinesFree = 1,
    Transfer = 2,
    Reserved = 3
}

impl From<SdioR5StateValue> for u32 {
    fn from(val: SdioR5StateValue) -> Self {
        val as u32
    }
}

impl From<u32> for SdioR5StateValue {
    fn from(val: u32) -> Self {
        match val {
            0 => SdioR5StateValue::Disabled,
            1 => SdioR5StateValue::CmdDatLinesFree,
            2 => SdioR5StateValue::Transfer,
            _ => SdioR5StateValue::Reserved
        }
    }
}

pub struct SdioR5State {
    pub val: u16
}

impl From<SdioR5State> for u16 {
    fn from(val: SdioR5State) -> Self {
        val.val
    }
}

impl SdioR5State {
    pub fn set_crc_error(&mut self, error: bool) {
        self.val.set_bit(15, error);
    }

    pub fn crc_error(&self) -> bool {
        self.val.get_bit(15)
    }

    pub fn set_illegal_command(&mut self, illegal: bool) {
        self.val.set_bit(14, illegal);
    }

    pub fn illegal_command(&self) -> bool {
        self.val.get_bit(14)
    }

    pub fn set_state(&mut self, val: SdioR5StateValue) {
        self.val.set_bits(12..13, val.into());
    }

    pub fn state(&self) -> SdioR5StateValue {
        self.val.get_bits(12..13).into()
    }

    pub fn set_general_error(&mut self, error: bool) {
        self.val.set_bit(11, error);
    }

    pub fn general_error(&self) -> bool {
        self.val.get_bit(11)
    }

    pub fn set_invalid_function_number(&mut self, error: bool) {
        self.val.set_bit(9, error);
    }

    pub fn invalid_function_number(&self) -> bool {
        self.val.get_bit(9)
    }

    pub fn set_argument_out_of_range(&mut self, error: bool) {
        self.val.set_bit(8, error);
    }

    pub fn argument_out_of_range(&self) -> bool {
        self.val.get_bit(8)
    }
}

pub struct SdioR6State {
    pub val: u16
}

impl SdioR6State {
    pub fn set_crc_error(&mut self, error: bool) {
        self.val.set_bit(15, error);
    }

    pub fn crc_error(&self) -> bool {
        self.val.get_bit(15)
    }

    pub fn set_illegal_command(&mut self, error: bool) {
        self.val.set_bit(14, error);
    }

    pub fn illegal_command(&self) -> bool {
        self.val.get_bit(14)
    }

    pub fn set_r6_error(&mut self, error: bool) {
        self.val.set_bit(13, error);
    }

    pub fn r6_error(&self) -> bool {
        self.val.get_bit(13)
    }
}