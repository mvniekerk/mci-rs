use crate::registers::register::Register;
use bit_field::BitField;

pub mod flags;
pub mod mmc_commands;
pub mod response_type;
pub mod sd_commands;
pub mod sdio_commands;

pub struct MciCommand {
    pub val: u32,
}

impl Register<u32> for MciCommand {
    fn value(&self) -> u32 {
        self.val
    }

    fn address() -> u8 {
        0x0
    }
}

impl From<u32> for MciCommand {
    fn from(val: u32) -> Self {
        MciCommand { val }
    }
}

impl MciCommand {
    pub fn have_response(&self) -> bool {
        self.val.get_bit(8)
    }

    pub fn have_136bit_response(&self) -> bool {
        self.val.get_bit(11)
    }

    pub fn expect_valid_crc(&self) -> bool {
        self.val.get_bit(12)
    }

    pub fn card_may_send_busy(&self) -> bool {
        self.val.get_bit(13)
    }

    pub fn open_drain_broadcast_command(&self) -> bool {
        self.val.get_bit(14)
    }

    pub fn data_write_command(&self) -> bool {
        self.val.get_bit(15)
    }

    pub fn sdio_multi_byte_transfer(&self) -> bool {
        self.val.get_bit(16)
    }

    pub fn sdio_block_mode_transfer(&self) -> bool {
        self.val.get_bit(17)
    }

    pub fn stream_mode_data_transfer(&self) -> bool {
        self.val.get_bit(18)
    }

    pub fn single_block_data_transfer(&self) -> bool {
        self.val.get_bit(19)
    }

    pub fn multi_block_data_transfer(&self) -> bool {
        self.val.get_bit(20)
    }
}
