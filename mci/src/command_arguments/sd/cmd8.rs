use bit_field::BitField;

#[derive(Default)]
pub struct Cmd8 {
    pub val: u16,
}

impl Cmd8 {
    pub fn set_cmd8_pattern(&mut self, pattern: bool) -> &mut Self {
        self.val.set_bits(0..8, if pattern { 0xAA } else { 0x0 });
        self
    }

    pub fn cmd8_pattern(&self) -> bool {
        self.val.get_bits(0..8) == 0xAA
    }

    pub fn set_high_voltage(&mut self, high_voltage: bool) -> &mut Self {
        self.val
            .set_bits(8..12, if high_voltage { 0x1 } else { 0x0 });
        self
    }

    pub fn high_voltage(&self) -> bool {
        self.val.get_bits(8..12) == 0x1
    }
}
