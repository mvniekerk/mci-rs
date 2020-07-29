pub trait CommandFlag {
    fn val() -> isize;
}

pub struct NoFlag {}
impl CommandFlag for NoFlag {
    fn val() -> isize {
        CommandFlags::NoFlag as isize
    }
}

pub struct OpenDrain {}
impl CommandFlag for OpenDrain {
    fn val() -> isize {
        CommandFlags::OpenDrain as isize
    }
}

pub struct Write {}
impl CommandFlag for Write {
    fn val() -> isize {
        CommandFlags::Write as isize
    }
}

pub struct SdioByte {}
impl CommandFlag for SdioByte {
    fn val() -> isize {
        CommandFlags::SdioByte as isize
    }
}

pub struct SdioBlock {}
impl CommandFlag for SdioBlock {
    fn val() -> isize {
        CommandFlags::SdioBlock as isize
    }
}

pub struct Stream {}
impl CommandFlag for Stream {
    fn val() -> isize {
        CommandFlags::Stream as isize
    }
}

pub struct SingleBlock {}
impl CommandFlag for SingleBlock {
    fn val() -> isize {
        CommandFlags::SingleBlock as isize
    }
}

pub struct MultiBlock {}
impl CommandFlag for MultiBlock {
    fn val() -> isize {
        CommandFlags::MultiBlock as isize
    }
}

pub struct WriteSingleBlock {}
impl CommandFlag for WriteSingleBlock {
    fn val() -> isize {
        CommandFlags::WriteSingleBlock as isize
    }
}

pub struct WriteMultiBlock {}
impl CommandFlag for WriteMultiBlock {
    fn val() -> isize {
        CommandFlags::WriteMultiBlock as isize
    }
}

pub struct SdioWriteByte {}
impl CommandFlag for SdioWriteByte {
    fn val() -> isize {
        CommandFlags::SdioWriteByte as isize
    }
}

pub struct SdioWriteBlock {}
impl CommandFlag for SdioWriteBlock {
    fn val() -> isize {
        CommandFlags::SdioWriteBlock as isize
    }
}

pub enum CommandFlags {
    NoFlag              = 0,
    // Open drain for a broadcast command = bc,
    // or to enter in inactive state = MCI only,
    OpenDrain    = 1 << 14,
    // To signal a data write operation
    Write        = 1 << 15,
    // To signal a SDIO tranfer in multi byte mode
    SdioByte     = 1 << 16,
    // To signal a SDIO tranfer in block mode
    SdioBlock    = 1 << 17,
    // To signal a data transfer in stream mode
    Stream       = 1 << 18,
    // To signal a data transfer in single block mode
    SingleBlock  = 1 << 19,
    // To signal a data transfer in multi block mode
    MultiBlock   = 1 << 20,
    WriteSingleBlock = (1 << 19) | (1 << 15),
    WriteMultiBlock = (1 << 20) | (1 << 15),
    SdioWriteByte = (1 << 16) | (1 << 15),
    SdioWriteBlock = (1 << 17) | (1 << 15)
}

