use crate::registers::register::Register;
use bit_field::BitField;

pub struct BusSuspendRegister {
    pub val: u8,
}

impl Register for BusSuspendRegister {
    fn address() -> u8 {
        0x0C
    }
}

impl BusSuspendRegister {
    pub fn set_transfer_on_datx_line(&mut self, transferring: bool) {
        self.val.set_bit(0, transferring);
    }

    pub fn transfer_on_datx_line(&self) -> bool {
        self.val.get_bit(0)
    }

    pub fn set_request_release_status(&mut self, release: bool) {
        self.val.set_bit(1, release);
    }

    pub fn request_release_status(&self) -> bool {
        self.val.get_bit(0)
    }
}
