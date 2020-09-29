use crate::command_arguments::mmc::BusWidth;
use embedded_error::mci::MciError;

// TODO keep + get current selected slot
pub trait Mci {
    /// Initialize MCI low level driver.
    fn init(&self) -> Result<(), MciError>;

    fn send_command(&self, cmd: u32, arg: u32) -> Result<(), MciError>;

    /// Deinitialize MCI low level driver.
    fn deinit(&self) -> Result<(), MciError>;

    /// Select a device and initialize it
    fn select_device(
        &self,
        slot: u8,
        clock: u32,
        bus_width: &BusWidth,
        high_speed: bool,
    ) -> Result<(), MciError>;

    /// Deselect device
    fn deselect_device(&self, slot: u8) -> Result<(), MciError>;

    /// Get the maximum bus width for a device
    fn get_bus_width(&self, slot: u8) -> Result<BusWidth, MciError>;

    /// Whether the device is high speed capable
    fn is_high_speed_capable(&self) -> Result<bool, MciError>;

    /// Send 74 clock cycles on the line. Required after card plug and install
    fn send_clock(&self) -> Result<(), MciError>;

    /// Get 32 bits response of last command
    fn get_response(&self) -> Result<u32, MciError>;

    /// Get 128 bits response of last command
    fn get_response128(&self) -> Result<[u32; 4], MciError>;

    /// ADTC command start
    /// An ADTC (Addressed Data Transfer Commands) is used for R/W access
    ///
    /// # Arguments
    /// * `command`: 32bit command
    /// * `argument`: Argument of the command
    /// * `block_size`: 16bit block size
    /// * `block_amount`: Amount of blocks to transfer
    /// * `access_in_blocks`: If true - read_blocks/write_blocks must be used after this command
    ///                 Otherwise read_word/write_word must be used
    fn adtc_start(
        &self,
        command: u32,
        argument: u32,
        block_size: u16,
        block_amount: u16,
        access_in_blocks: bool,
    ) -> Result<(), MciError>;

    /// ADTC command stop
    /// Send a command to stop an ADTC
    /// # Arguments
    /// * `command`: 32bit command
    /// * `argument`: Argument of the command
    fn adtc_stop(&self, command: u32, argument: u32) -> Result<(), MciError>;

    /// Read a word on the wire
    fn read_word(&self) -> Result<(u32, u8), MciError>;

    /// Write a word on the wire
    fn write_word(&self, val: u32) -> Result<bool, MciError>;

    /// Start a read block transfer on the line
    /// # Arguments
    ///  * `destination` Buffer to write to
    ///  * `number_of_blocks` Number of blocks to read
    fn read_blocks(&self, destination: &mut [u8], number_of_blocks: u16) -> Result<bool, MciError>;

    /// Start a write block transfer on the line
    /// # Arguments
    ///  * `data` - Data to write on the line
    ///  * `number_of_blocks` - Number of blocks to write
    fn write_blocks(&self, data: &[u8], number_of_blocks: u16) -> Result<bool, MciError>;

    /// Wait until the end of reading the blocks
    fn wait_until_read_finished(&self) -> Result<(), MciError>;

    /// Wait until the end of writing blocks
    fn wait_until_write_finished(&self) -> Result<(), MciError>;
}
