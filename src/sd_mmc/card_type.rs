use crate::sd_mmc::registers::registers::Register;
use bit_field::BitField;

pub struct CardType {
    pub val: u8
}

impl Register<u8> for CardType {
    fn value(&self) -> u8 {
        self.val
    }

    fn address() -> u8 {
        unimplemented!()
    }
}

impl CardType {
    pub fn set_unknown(&mut self) -> &mut Self {
        self.val = 0x0;
        self
    }

    pub fn set_sd(&mut self, sd: bool) -> &mut Self {
        self.val.set_bit(1, sd);
        self
    }

    pub fn sd(&self) -> bool {
        self.val.get_bit(1)
    }

    pub fn set_mmc(&mut self, mmc: bool) -> &mut Self {
        self.val.set_bit(2, mmc);
        self
    }

    pub fn mmc(&self) -> bool {
        self.val.get_bit(2)
    }

    pub fn set_sdio(&mut self, sdio: bool) -> &mut Self {
        self.val.set_bit(3, sdio);
        self
    }

    pub fn sdio(&self) -> bool {
        self.val.get_bit(3)
    }

    pub fn set_high_capacity(&mut self, hc: bool) -> &mut Self {
        self.val.set_bit(4, hc);
        self
    }

    pub fn high_capacity(&self) -> bool {
        self.val.get_bit(4)
    }
}