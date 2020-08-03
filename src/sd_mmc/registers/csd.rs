use crate::sd_mmc::registers::registers::Register;

pub struct CsdRegister {
    pub val: u128
}

impl Register<u128> for CsdRegister {
    fn value(&self) -> u128 {
        self.val
    }

    fn address() -> u8 {
        0x10u8
    }
}

impl CsdRegister {
    pub fn mask(len: u8) -> u128 {
        let mut mask = 0x0u128;
        for _ in 0..(len/8) {
            mask <<= 8;
            mask |= 0xFF;
        }
        for _ in 0..(len % 8) {
            mask <<= 1;
            mask |= 1;
        }
        mask
    }

    pub fn set_bits(&mut self, bits: u128, offset: u16, len: u8) {
        let mask = CsdRegister::mask(len);
        let bits = bits & mask;
        self.val = (self.val & !(mask << offset)) | (bits << offset);
    }

    pub fn get_bits(&self, offset: u16, len: u8) -> u128 {
        let mask = CsdRegister::mask(len);
        let val_at_offset = self.val >> offset;
        // (self.val >> offset) & mask
        val_at_offset & mask
    }

    pub fn set_csd_structure_version(&mut self, version: u8) {
        self.set_bits(version as u128, 126, 2);
    }

    pub fn csd_structure_version(&self) -> u8 {
        self.get_bits(126, 2) as u8
    }

    pub fn set_mmc_csd_spec_version(&mut self, version: u8) {
        self.set_bits(version as u128, 122, 4);
    }

    pub fn mmc_csd_spec_version(&self) -> u8 {
        self.get_bits(122, 4) as u8
    }

    pub fn set_transmission_speed(&mut self, speed: u8) {
        self.set_bits(speed as u128, 96, 8);
    }

    pub fn transmission_speed(&self) -> u8 {
        self.get_bits(96, 8) as u8
    }

    pub fn set_read_bl_length(&mut self, length: u8) {
        self.set_bits(length as u128, 80, 4);
    }

    pub fn read_bl_length(&self) -> u8 {
        self.get_bits(80, 4) as u8
    }

    pub fn set_card_size(&mut self, size: u16) {
        self.set_bits(size as u128, 62, 12);
    }

    pub fn card_size(&self) -> u16 {
        self.get_bits(62, 12) as u16
    }

    pub fn set_sd_2_0_card_size(&mut self, size: u32) {
        self.set_bits(size as u128, 48, 22);
    }

    pub fn sd_2_0_card_size(&self) -> u32 {
        self.get_bits(48, 22) as u32
    }

    pub fn set_card_size_multiplier(&mut self, multiplier: u8) {
        self.set_bits(multiplier as u128, 47, 3);
    }

    pub fn card_size_multiplier(&self) -> u8 {
        self.get_bits(47, 3) as u8
    }

}

#[cfg(test)]
mod test {
    use crate::sd_mmc::registers::csd::CsdRegister;

    #[test]
    fn mask_test() {
        let m = CsdRegister::mask(10);
        assert_eq!(m, 0x3FF);
        let m = CsdRegister::mask(128);
        assert_eq!(m, 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF);
        let m = CsdRegister::mask(64);
        assert_eq!(m, 0xFFFF_FFFF_FFFF_FFFF);
    }

    #[test]
    fn set_bits_test() {
        let mut m = CsdRegister { val: 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF };

        m.set_bits(0x0aa0_5050, 32, 32);
        assert_eq!(m.val, 0xFFFF_FFFF_FFFF_FFFF_0aa0_5050_FFFF_FFFF);

        m.val = !0;
        m.set_bits(0x5050, 16, 16);
        assert_eq!(m.val, 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_5050_FFFF)
    }

    #[test]
    fn get_bits_test() {
        let mut m = CsdRegister { val: !0 };
        assert_eq!(m.get_bits(64, 16), 0xFFFF);
        //                             128    112    96    80    64    48    32    16
        let mut m = CsdRegister { val:  0xa0a0___ffff__0000__5f5f__fafa__d1d1__d8d8__0e0e};
        assert_eq!(m.get_bits(4, 8), 0xe0);
        assert_eq!(m.get_bits(4, 16), 0x80e0);
        assert_eq!(m.get_bits(104, 24), 0xa0_a0ff);
    }
}