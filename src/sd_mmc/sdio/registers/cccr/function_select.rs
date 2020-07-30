use std::hint::unreachable_unchecked;
use bit_field::BitField;

pub enum FunctionSelection {
    FunctionCia0 = 0,
    Function1 = 1,
    Function2 = 2,
    Function3 = 3,
    Function4 = 4,
    Function5 = 5,
    Function6 = 6,
    Function7 = 7,
    MemoryInCard = 8
}

impl From<u8> for FunctionSelection {
    fn from(val: u8) -> Self {
        match val {
            0 => FunctionSelection::FunctionCia0,
            1 => FunctionSelection::Function1,
            2 => FunctionSelection::Function2,
            3 => FunctionSelection::Function3,
            4 => FunctionSelection::Function4,
            5 => FunctionSelection::Function5,
            6 => FunctionSelection::Function6,
            7 => FunctionSelection::Function7,
            8 => FunctionSelection::MemoryInCard,
            _ => unsafe { unreachable_unchecked() }
        }
    }
}

struct FunctionSelectionRegister {
    pub val: u8
}

impl FunctionSelectionRegister {
    pub fn set_resume_data_flag(&mut self, resume: bool) {
        self.val.set_bit(7, resume);
    }

    pub fn resume_data_flag(&self) -> bool {
        self.val.get_bit(7)
    }

    pub fn set_function_selection(&mut self, selection: FunctionSelection) {
        self.val.set_bits(0..3, selection as u8);
    }

    pub fn function_selection(&self) -> FunctionSelection {
        self.val.get_bits(0..3).into()
    }
}
