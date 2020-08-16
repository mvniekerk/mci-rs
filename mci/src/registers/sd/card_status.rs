use bit_field::BitField;
use core::hint::unreachable_unchecked;

pub enum CardStatusState {
    Idle = 0,
    Ready = 1,
    Identity = 2,
    Standby = 3,
    Transmitting = 4,
    Data = 5,
    Receiving = 6,
    Programming = 7,
    Disabled = 8,
}

impl From<u32> for CardStatusState {
    fn from(val: u32) -> Self {
        match val {
            0 => CardStatusState::Idle,
            1 => CardStatusState::Ready,
            2 => CardStatusState::Identity,
            3 => CardStatusState::Standby,
            4 => CardStatusState::Transmitting,
            6 => CardStatusState::Data,
            7 => CardStatusState::Programming,
            8 => CardStatusState::Disabled,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

#[derive(Default)]
pub struct CardStatusRegister {
    pub val: u32,
}

impl CardStatusRegister {
    pub fn set_app_command(&mut self, set: bool) {
        self.val.set_bit(5, set);
    }

    pub fn app_command(&self) -> bool {
        self.val.get_bit(5)
    }

    pub fn set_switch_error(&mut self, set: bool) {
        self.val.set_bit(7, set);
    }

    pub fn switch_error(&self) -> bool {
        self.val.get_bit(7)
    }

    pub fn set_ready_for_data(&mut self, ready: bool) {
        self.val.set_bit(8, ready);
    }

    pub fn ready_for_data(&self) -> bool {
        self.val.get_bit(8)
    }

    pub fn set_state(&mut self, state: CardStatusState) {
        self.val.set_bits(9..13, state as u32);
    }

    pub fn state(&self) -> CardStatusState {
        self.val.get_bits(9..13).into()
    }

    pub fn set_erase_reset(&mut self, set: bool) {
        self.val.set_bit(13, set);
    }

    pub fn erase_reset(&self) -> bool {
        self.val.get_bit(13)
    }

    pub fn set_write_protect_erase_skip(&mut self, set: bool) {
        self.val.set_bit(15, set);
    }

    pub fn write_protect_erase_skip(&self) -> bool {
        self.val.get_bit(15)
    }

    pub fn set_cidcsd_overwrite(&mut self, overwrite: bool) {
        self.val.set_bit(16, overwrite);
    }

    pub fn cidcsd_overwrite(&self) -> bool {
        self.val.get_bit(16)
    }

    pub fn set_overrun(&mut self, overrun: bool) {
        self.val.set_bit(17, overrun);
    }

    pub fn overrun(&self) -> bool {
        self.val.get_bit(17)
    }

    pub fn set_underrun(&mut self, underrun: bool) {
        self.val.set_bit(18, underrun);
    }

    pub fn underrun(&self) -> bool {
        self.val.get_bit(18)
    }

    pub fn set_status_error(&mut self, error: bool) {
        self.val.set_bit(19, error);
    }

    pub fn status_error(&self) -> bool {
        self.val.get_bit(19)
    }

    pub fn set_cc_error(&mut self, error: bool) {
        self.val.set_bit(20, error);
    }

    pub fn cc_error(&self) -> bool {
        self.val.get_bit(20)
    }

    pub fn set_card_ecc_failed(&mut self, failed: bool) {
        self.val.set_bit(21, failed);
    }

    pub fn card_ecc_failed(&self) -> bool {
        self.val.get_bit(21)
    }

    pub fn set_illegal_command(&mut self, illegal: bool) {
        self.val.set_bit(22, illegal);
    }

    pub fn illegal_command(&self) -> bool {
        self.val.get_bit(22)
    }

    pub fn set_communication_crc_error(&mut self, error: bool) {
        self.val.set_bit(23, error);
    }

    pub fn communication_crc_error(&self) -> bool {
        self.val.get_bit(23)
    }

    pub fn set_unlock_failed(&mut self, failed: bool) {
        self.val.set_bit(24, failed);
    }

    pub fn unlock_failed(&self) -> bool {
        self.val.get_bit(24)
    }

    pub fn set_card_is_locked(&mut self, locked: bool) {
        self.val.set_bit(25, locked);
    }

    pub fn card_is_locked(&self) -> bool {
        self.val.get_bit(25)
    }

    pub fn set_write_protect_violation(&mut self, violated: bool) {
        self.val.set_bit(26, violated);
    }

    pub fn write_protect_violation(&self) -> bool {
        self.val.get_bit(26)
    }

    pub fn set_erase_parameter(&mut self, erase: bool) {
        self.val.set_bit(27, erase);
    }

    pub fn erase_parameter(&self) -> bool {
        self.val.get_bit(27)
    }

    pub fn set_erase_sequence_error(&mut self, error: bool) {
        self.val.set_bit(28, error);
    }

    pub fn erase_sequence_error(&self) -> bool {
        self.val.get_bit(28)
    }

    pub fn set_block_length_error(&mut self, error: bool) {
        self.val.set_bit(29, error);
    }

    pub fn block_length_error(&self) -> bool {
        self.val.get_bit(29)
    }

    pub fn set_address_misalign_error(&mut self, error: bool) {
        self.val.set_bit(30, error);
    }

    pub fn address_misalign_error(&self) -> bool {
        self.val.get_bit(30)
    }

    pub fn set_address_out_of_range_error(&mut self, error: bool) {
        self.val.set_bit(31, error);
    }

    pub fn address_out_of_range_error(&self) -> bool {
        self.val.get_bit(31)
    }

    pub fn has_error(&self) -> bool {
        self.address_out_of_range_error()
            | self.address_misalign_error()
            | self.block_length_error()
            | self.write_protect_violation()
            | self.illegal_command()
            | self.cc_error()
            | self.status_error()
    }
}
