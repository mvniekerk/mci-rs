pub trait CommandFlag {
    fn val(&self) -> u32;
}

#[derive(Default)]
pub struct NoFlag;
impl CommandFlag for NoFlag {
    fn val(&self) -> u32 {
        CommandFlags::NoFlag as u32
    }
}

#[derive(Default)]
pub struct OpenDrain;
impl CommandFlag for OpenDrain {
    fn val(&self) -> u32 {
        CommandFlags::OpenDrain as u32
    }
}

#[derive(Default)]
pub struct Write;
impl CommandFlag for Write {
    fn val(&self) -> u32 {
        CommandFlags::Write as u32
    }
}

#[derive(Default)]
pub struct SdioByte;
impl CommandFlag for SdioByte {
    fn val(&self) -> u32 {
        CommandFlags::SdioByte as u32
    }
}

#[derive(Default)]
pub struct SdioBlock;
impl CommandFlag for SdioBlock {
    fn val(&self) -> u32 {
        CommandFlags::SdioBlock as u32
    }
}

#[derive(Default)]
pub struct Stream;
impl CommandFlag for Stream {
    fn val(&self) -> u32 {
        CommandFlags::Stream as u32
    }
}

#[derive(Default)]
pub struct SingleBlock;
impl CommandFlag for SingleBlock {
    fn val(&self) -> u32 {
        CommandFlags::SingleBlock as u32
    }
}

#[derive(Default)]
pub struct MultiBlock;
impl CommandFlag for MultiBlock {
    fn val(&self) -> u32 {
        CommandFlags::MultiBlock as u32
    }
}

#[derive(Default)]
pub struct WriteSingleBlock;
impl CommandFlag for WriteSingleBlock {
    fn val(&self) -> u32 {
        CommandFlags::WriteSingleBlock as u32
    }
}

#[derive(Default)]
pub struct WriteMultiBlock;
impl CommandFlag for WriteMultiBlock {
    fn val(&self) -> u32 {
        CommandFlags::WriteMultiBlock as u32
    }
}

#[derive(Default)]
pub struct SdioWriteByte;
impl CommandFlag for SdioWriteByte {
    fn val(&self) -> u32 {
        CommandFlags::SdioWriteByte as u32
    }
}

#[derive(Default)]
pub struct SdioWriteBlock;
impl CommandFlag for SdioWriteBlock {
    fn val(&self) -> u32 {
        CommandFlags::SdioWriteBlock as u32
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

