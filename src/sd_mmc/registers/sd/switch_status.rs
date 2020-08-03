use crate::sd_mmc::registers::registers::Register;
use bit_field::BitField;

pub struct SwitchStatusRegister {
    pub val: [u64; 8]
}

impl Register<[u64; 8]> for SwitchStatusRegister {
    fn value(&self) -> [u64;8] {
        self.val
    }

    fn address() -> u8 {
        0x0
    }
}

impl SwitchStatusRegister {
    pub fn set_bits(&mut self, val: u16, offset: u16, len: usize) {
        let index = offset / 64;
        let p_offset = (offset - (index * 64)) as usize;
        let index = index as usize;
        let mut current = self.val[index];
        if p_offset < 64 - len {
            let val = val as u64;
            let range = p_offset..(p_offset + len);
            current.set_bits(range, val);
            self.val[index] = current;
        } else {
            let shift = 64 - p_offset;
            let over = val >> shift;
            let mut mask = !0u16;
            mask <<= shift;
            let val = val & !mask;
            current.set_bits(p_offset..64, val as u64);
            self.val[index] = current;
            current = self.val[index +1];
            current.set_bits(0..(len - shift), over as u64);
            self.val[index + 1] = current;
        }
    }

    /// Largest get is 16 bits, hence u16 as return
    pub fn get_bits(&self, offset: u16, len: usize) -> u16 {
        let index = offset / 64;
        let p_offset = (offset - (index * 64)) as usize;
        let index = index as usize;
        let mut current = self.val[index];
        if p_offset < 64 - len {
            let range = p_offset..(p_offset + len);
            current.get_bits(range) as u16
        }  else {
            let shift = 64 - p_offset;
            let mut val = current.get_bits(p_offset..64) as u16;
            current = self.val[index];
            val | (current.get_bits(0..shift) as u16)
        }
    }


}

#[cfg(test)]
mod tests {
    use crate::sd_mmc::registers::sd::switch_status::SwitchStatusRegister;

    #[test]
    fn test_setting_bits() {
        let mut reg = SwitchStatusRegister { val: [0u64; 8] };

        reg.set_bits(0b10101, 128, 5);
        assert_eq!(reg.val[0], 0);
        assert_eq!(reg.val[1], 0);
        assert_eq!(reg.val[2], 0b10101);

        reg.val = [0u64; 8];

        reg.set_bits(0b11111, 126, 5);
        assert_eq!(reg.val[0], 0);
        assert_eq!(reg.val[1], 0xC000_0000_0000_0000);
        assert_eq!(reg.val[2], 0b111);
    }

    #[test]
    fn test_getting_bits() {
        let mut reg = SwitchStatusRegister { val: [0u64; 8] };

        reg.set_bits(0xa0a0, 80, 16);
        reg.set_bits(0x5555, 96, 16);

        let bits = reg.get_bits(88, 16);
        assert_eq!(bits, 0x55a0);
        let bits = reg.get_bits(92, 8);
        assert_eq!(bits, 0x5a);
        let bits = reg.get_bits(94, 4);
        assert_eq!(bits, 0x06);
    }

}