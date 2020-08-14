use crate::sd_mmc::command::mmc_commands::BusWidth;

// TODO keep + get current selected slot
pub trait Mci {
    /// Initialize MCI low level driver.
    fn init(&mut self) -> Result<(), ()>;

    fn send_command(&mut self, cmd: u32, arg: u32) -> Result<(), ()>;

    /// Deinitialize MCI low level driver.
    fn deinit(&mut self) -> Result<(), ()>;

    /// Select a device and initialize it
    fn select_device(&mut self, slot: u8, clock: u32, bus_width: &BusWidth, high_speed: bool) -> Result<(), ()>;

    /// Deselect device
    fn deselect_device(&mut self) -> Result<(), ()>;

    /// Get the maximum bus width for a device
    fn get_bus_width(&mut self, slot: u8) -> Result<BusWidth, ()>;

    /// Whether the device is high speed capable
    fn is_high_speed_capable(&mut self) -> Result<bool, ()>;

    /// Send 74 clock cycles on the line. Required after card plug and install
    fn send_clock(&mut self) -> Result<(), ()>;

    /// Get 32 bits response of last command
    fn get_response(&mut self) -> u32;

    /// Get 128 bits response of last command
    fn get_response128(&mut self) -> [u32; 4];

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
    fn adtc_start(&mut self, command: u32, argument: u32, block_size: u16, block_amount: u16, access_in_blocks: bool) -> Result<(), ()>;

    /// ADTC command stop
    /// Send a command to stop an ADTC
    /// # Arguments
    /// * `command`: 32bit command
    /// * `argument`: Argument of the command
    fn adtc_stop(&self, command: u32, argument: u32) -> Result<(), ()>;

    /// Read a word on the wire
    fn read_word(&mut self) -> Result<(u32, u8), ()>;

    /// Write a word on the wire
    fn write_word(&mut self, val: u32) -> Result<bool, ()>;

    /// Start a read block transfer on the line
    /// # Arguments
    ///  * `destination` Buffer to write to
    ///  * `number_of_blocks` Number of blocks to read
    fn read_blocks(&mut self, destination: &mut [u8], number_of_blocks: usize) -> Result<bool, ()>;

    /// Start a write block transfer on the line
    /// # Arguments
    ///  * `data` - Data to write on the line
    ///  * `number_of_blocks` - Number of blocks to write
    fn write_blocks(&mut self, data: &[u8], number_of_blocks: usize) -> Result<bool, ()>;

    /// Wait until the end of reading the blocks
    fn wait_until_read_finished(&self) -> Result<(), ()>;

    /// Wait until the end of writing blocks
    fn wait_until_write_finished(&self) -> Result<(), ()>;
}