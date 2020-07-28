use std::ops::{BitOr, Add};

pub enum Flags {
    NoFlag              = 0,
    // Have response MCI only
    ResponsePresent     = 1 << 8,
    // 8 bit response = SPI only,
    Response8           = 1 << 9,
    // 32 bit response = SPI only,
    Response32          = 1 << 10,
    // 136 bit response = MCI only,
    Response136         = 1 << 11,
    // Expect valid crc = MCI only,
    ResponseCrc         = 1 << 12,
    // Card may send busy
    ResponseBusy        = 1 << 13,
    // Open drain for a braodcast command = bc,
    // or to enter in inactive state = MCI only,
    CommandOpenDrain    = 1 << 14,
    // To signal a data write operation
    CommandWrite        = 1 << 15,
    // To signal a SDIO tranfer in multi byte mode
    CommandSdioByte     = 1 << 16,
    // To signal a SDIO tranfer in block mode
    CommandSdioBlock    = 1 << 17,
    // To signal a data transfer in stream mode
    CommandStream       = 1 << 18,
    // To signal a data transfer in single block mode
    CommandSingleBlock  = 1 << 19,
    // To signal a data transfer in multi block mode
    CommandMultiBlock   = 1 << 20,
    CommandWriteSingleBlock = (1 << 19) | (1 << 15),
    CommandWriteMultiBlock = (1 << 20) | (1 << 15),
    CommandSdioWriteByte = (1 << 16) | (1 << 15),
    CommandSdioWriteBlock = (1 << 17) | (1 << 15)
}

pub enum ResponseType {
    NoResponse = 0,
    CmdR1R6     = (Flags::ResponsePresent as isize | Flags::ResponseCrc as isize),
    CmdR1B      = (Flags::ResponsePresent as isize | Flags::ResponseCrc as isize | Flags::ResponseBusy as isize),
    CmdR2       = (Flags::ResponsePresent as isize | Flags::Response8 as isize | Flags::Response136 as isize | Flags::ResponseCrc as isize),
    CmdR3R4     = (Flags::ResponsePresent as isize | Flags::Response32 as isize),
    CmdR5       = (Flags::ResponsePresent as isize | Flags::Response8 as isize | Flags::ResponseCrc as isize),
    CmdR7       = (Flags::ResponsePresent as isize | Flags::Response32 as isize | Flags::ResponseCrc as isize)
}

pub struct Command {
    number: usize,
    response: ResponseType,
    flag: Flags
}

// Cmd0(bc): Reset all cards to idle state
pub const SDMMC_SPI_CMD0_GO_IDLE_STATE: Command = Command { number: 0, response: ResponseType::CmdR1R6, flag: Flags::NoFlag };
pub const SDMMC_MCI_CMD0_GO_IDLE_STATE: Command = Command { number: 0, response: ResponseType::NoResponse, flag: Flags::CommandOpenDrain };

// MMC Cmd1(bcr, R3): Ask the card to send its Operating Conditions
pub const MMC_SPI_CMD1_SEND_OP_COND: Command = Command { number: 1, response: ResponseType::CmdR1R6, flag: Flags::NoFlag };
pub const MMC_MCI_CMD1_SEND_OP_COND: Command = Command { number: 1, response: ResponseType::CmdR3R4, flag: Flags::CommandOpenDrain };

// Cmd2(bcr, R2): Ask the card to send its CID number (stuff but arg 0 used)
pub const SDMMC_CMD2_ALL_SEND_CID: Command = Command { number: 2, response: ResponseType::CmdR2, flag: Flags::CommandOpenDrain };

// SD Cmd3(bcr, R6): Ask the card to publish a new relative address (RCA)
pub const SD_CMD3_SEND_RELATIVE_ADDR: Command = Command { number: 3, response: ResponseType::CmdR1R6, flag: Flags::CommandOpenDrain };

// MMC Cmd3(ac, R1): Assigns relative address to the card
pub const MMC_CMD3_SET_RELATIVE_ADDR: Command = Command { number: 3, response: ResponseType::CmdR1R6, flag: Flags::NoFlag };

// Cmd4(bc): Program the DSR of all cards (MCI only)
pub const SDMMC_CMD4_SET_DSR: Command = Command  { number: 4, response: ResponseType::NoResponse, flag: Flags::NoFlag };

// MMC Cmd5(ac, R1b): Toggle the card between Sleep state and Standby state.
pub const MMC_CMD5_SLEEP_AWAKE: Command = Command { number: 5, response: ResponseType::CmdR1B, flag: Flags::NoFlag};

// Cmd7(ac, R1/R1b): Select/Deselect card
// For SD: R1b only from the selected card.
// For MMC: R1 while selecting from Stand-By State to Transfer State;
//           R1b while selecting from Disconnected State to Programming State.
//
pub const SDMMC_CMD7_SELECT_CARD_CMD: Command = Command { number: 7, response: ResponseType::CmdR1B, flag: Flags::NoFlag};
pub const SDMMC_CMD7_DESELECT_CARD_CMD: Command = Command { number: 7, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

// MMC Cmd8(adtc, R1): Send EXT_CSD register as a block of data
pub const MMC_CMD8_SEND_EXT_CSD: Command = Command { number: 7, response: ResponseType::CmdR1R6, flag: Flags::CommandSingleBlock};
// SD Cmd8(bcr, R7) : Send SD Memory Card interface condition
pub const SD_CMD8_SEND_IF_COND: Command = Command { number: 8, response: ResponseType::CmdR7, flag: Flags::CommandOpenDrain};

// Cmd9 SPI (R1): Addressed card sends its card-specific data (CSD)
pub const SDMMC_SPI_CMD9_SEND_CSD: Command = Command { number: 8, response: ResponseType::CmdR1R6, flag: Flags::CommandSingleBlock};
// Cmd9 MCI (ac, R2): Addressed card sends its card-specific data (CSD)
pub const SDMMC_MCI_CMD9_SEND_CSD: Command = Command { number: 9, response: ResponseType::CmdR2, flag: Flags::NoFlag};

// Cmd10(ac, R2): Addressed card sends its card identification (CID)
pub const SDMMC_CMD10_SEND_CID: Command = Command { number: 10, response: ResponseType::CmdR2, flag: Flags::NoFlag};

//
//  MMC Cmd11(adtc, R1): Read data stream from the card, starting at the given
//  address, until a STOP_TRANSMISSION follows.
//
pub const MMC_CMD11_READ_DAT_UNTIL_STOP: Command = Command { number: 11, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};
// SD Cmd11 MCI (ac, R1): Voltage switching
pub const SD_CMD11_READ_DAT_UNTIL_STOP: Command = Command { number: 11, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

// Cmd12(ac, R1b): Force the card to stop transmission
pub const SDMMC_CMD12_STOP_TRANSMISSION: Command = Command { number: 12, response: ResponseType::CmdR1B, flag: Flags::NoFlag};

// Cmd13(R2): Addressed card sends its status register.
pub const SDMMC_SPI_CMD13_SEND_STATUS: Command = Command { number: 13, response: ResponseType::CmdR2, flag: Flags::NoFlag};
// Cmd13(ac, R1): Addressed card sends its status register.
pub const SDMMC_MCI_CMD13_SEND_STATUS: Command = Command { number: 13, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

// MMC Cmd14(adtc, R1): Read the reversed bus testing data pattern from a card.
pub const MMC_CMD14_BUSTEST_R: Command = Command { number: 14, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

// Cmd15(ac): Send an addressed card into the Inactive State.
// Note: It is a ac cmd, but it must be send like bc cmd to open drain
pub const SDMMC_CMD15_GO_INACTIVE_STATE: Command = Command {number: 15, response: ResponseType::NoResponse, flag: Flags::CommandOpenDrain};

// MMC Cmd19(adtc, R1): Send the bus test data pattern
pub const MMC_CMD19_BUSTEST_W: Command = Command { number: 19, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

// Cmd58(R3): Reads the OCR register of a card
pub const SDMMC_SPI_CMD58_READ_OCR: Command = Command { number: 19, response: ResponseType::CmdR3R4, flag: Flags::NoFlag};

// Cmd59(R1): Turns the CRC option on or off
pub const SDMMC_SPI_CMD59_CRC_ON_OFF: Command = Command { number: 59, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};


//
//  --- Block-oriented read commands (class 2) --
//

// Cmd16(ac, R1): Set the block length (in bytes)
pub const SDMMC_CMD16_SET_BLOCKLEN: Command = Command { number: 16, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

// Cmd17(adtc, R1): Read single block
pub const SDMMC_CMD17_READ_SINGLE_BLOCK: Command = Command { number: 17, response: ResponseType::CmdR1R6, flag: Flags::CommandSingleBlock};
// Cmd18(adtc, R1): Read multiple block
pub const SDMMC_CMD18_READ_MULTIPLE_BLOCK: Command = Command { number: 18, response: ResponseType::CmdR1R6, flag: Flags::CommandMultiBlock};

//
//  --- Sequential write commands (class 3) ---
//

//  MMC Cmd20(adtc, R1): Write a data stream from the host, starting at the
//  given address, until a STOP_TRANSMISSION follows.
pub const MMC_CMD20_WRITE_DAT_UNTIL_STOP: Command = Command { number: 20, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

//
//  --- Block-oriented write commands (class 4) ---
//

// MMC Cmd23(ac, R1): Set block count
pub const MMC_CMD23_SET_BLOCK_COUNT: Command = Command { number: 23, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

// Cmd24(adtc, R1): Write block
pub const SDMMC_CMD24_WRITE_BLOCK: Command = Command { number: 24, response: ResponseType::CmdR1R6, flag: Flags::CommandWriteSingleBlock };

// Cmd25(adtc, R1): Write multiple block
pub const SDMMC_CMD25_WRITE_MULTIPLE_BLOCK: Command = Command { number: 25, response: ResponseType::CmdR1R6, flag: Flags::CommandWriteMultiBlock};

// MMC Cmd26(adtc, R1): Programming of the card identification register.
pub const MMC_CMD26_PROGRAM_CID: Command = Command { number: 26, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

// Cmd27(adtc, R1): Programming of the programmable bits of the CSD.
pub const SDMMC_CMD27_PROGRAM_CSD: Command = Command { number: 27, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

//
//  --- Erase commands  (class 5) ---
//
// SD Cmd32(ac, R1):
pub const SD_CMD32_ERASE_WR_BLK_START: Command = Command { number: 32, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

// SD Cmd33(ac, R1):
pub const SD_CMD33_ERASE_WR_BLK_END: Command = Command { number: 33, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

// MMC Cmd35(ac, R1):
pub const MMC_CMD35_ERASE_GROUP_START: Command = Command { number: 35, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

// MMC Cmd36(ac, R1):
pub const MMC_CMD36_ERASE_GROUP_END: Command = Command { number: 36, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

// Cmd38(ac, R1B):
pub const SDMMC_CMD38_ERASE: Command = Command { number: 38, response: ResponseType::CmdR1B, flag: Flags::NoFlag};

//
//  --- Block Oriented Write Protection Commands (class 6) ---
//

// Cmd28(ac, R1b): Set write protection
pub const SDMMC_CMD28_SET_WRITE_PROT: Command = Command { number: 28, response: ResponseType::CmdR1B, flag: Flags::NoFlag};

// Cmd29(ac, R1b): Clr write protection
pub const SDMMC_CMD29_CLR_WRITE_PROT: Command = Command { number: 29, response: ResponseType::CmdR1B, flag: Flags::NoFlag};

// Cmd30(adtc, R1b): Send write protection
pub const SDMMC_CMD30_SEND_WRITE_PROT: Command = Command { number: 30, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

//
//  --- Lock Card (class 7) ---
//

// Cmd42(adtc, R1): Used to set/reset the password or lock/unlock the card.
pub const SDMMC_CMD42_LOCK_UNLOCK: Command = Command { number: 42, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

//
//  --- Application-specific commands (class 8) ---
//

//  Cmd55(ac, R1): Indicate to the card that the next command is an application
//  specific command rather than a standard command.
pub const SDMMC_CMD55_APP_CMD: Command = Command { number: 55, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

//  Cmd 56(adtc, R1): Used either to transfer a data block to the card or to get
//  a data block from the card for general purpose/application specific commands.
pub const SDMMC_CMD56_GEN_CMD: Command = Command { number: 56, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

//  MMC Cmd6(ac, R1b) : Switche the mode of operation of the selected card
//  or modifies the EXT_CSD registers.
pub const MMC_CMD6_SWITCH: Command = Command { number: 6, response: ResponseType::CmdR1B, flag: Flags::NoFlag};

//  SD Cmd6(adtc, R1) : Check switchable function (mode 0)
//  and switch card function (mode 1).
pub const SD_CMD6_SWITCH_FUNC: Command = Command { number: 6, response: ResponseType::CmdR1R6, flag: Flags::CommandSingleBlock};

// ACMD6(ac, R1): Define the data bus width
pub const SD_ACMD6_SET_BUS_WIDTH: Command = Command { number: 6, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

// ACMD13(adtc, R1): Send the SD Status
pub const SD_ACMD13_SD_STATUS: Command = Command { number: 13, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

//  ACMD22(adtc, R1): Send the number of the written (with-out errors) write blocks.
pub const SD_ACMD22_SEND_NUM_WR_BLOCKS: Command = Command { number: 22, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

//  ACMD23(ac, R1): Set the number of write blocks to be pre-erased before writing
pub const SD_ACMD23_SET_WR_BLK_ERASE_COUNT: Command = Command { number: 23, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

//  ACMD41(bcr, R3): Send host capacity support information (HCS) and asks the accessed card to send
//  its operating condition register (OCR) content in the response
pub const SD_MCI_ACMD41_SD_SEND_OP_COND: Command = Command { number: 41, response: ResponseType::CmdR3R4, flag: Flags::CommandOpenDrain};

//  ACMD41(R1): Send host capacity support information (HCS) and activates the card's initialization
//  process
pub const SD_SPI_ACMD41_SD_SEND_OP_COND: Command = Command { number: 41, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

//  ACMD42(ac, R1): Connect[1]/Disconnect[0] the 50 KOhm pull-up resistor on CD/DAT3 (pin 1) of the card.
pub const SD_ACMD42_SET_CLR_CARD_DETECT: Command = Command { number: 42, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

// ACMD51(adtc, R1): Read the SD Configuration Register (SCR)
pub const SD_ACMD51_SEND_SCR: Command = Command { number: 51, response: ResponseType::CmdR1R6, flag: Flags::NoFlag};

//
//  --- I/O mode commands (class 9) ---
//

// MMC Cmd39(ac, R4): Used to write and read 8 bit (register) data fields.
pub const MMC_CMD39_FAST_IO: Command = Command { number: 39, response: ResponseType::CmdR3R4, flag: Flags::NoFlag};

// MMC Cmd40(bcr, R5): Set the system into interrupt mode
pub const MMC_CMD40_GO_IRQ_STATE: Command = Command { number: 40, response: ResponseType::CmdR5, flag: Flags::CommandOpenDrain};

// SDIO Cmd5(R4): Send operation condition
pub const SDIO_CMD5_SEND_OP_COND: Command = Command { number: 5, response: ResponseType::CmdR3R4, flag: Flags::CommandOpenDrain};

// SDIO CMD52(R5): Direct IO read/write
pub const SDIO_CMD52_IO_RW_DIRECT: Command = Command { number: 52, response: ResponseType::CmdR5, flag: Flags::NoFlag};

// /** SDIO CMD53(R5): Extended IO read/write */
pub const SDIO_CMD53_IO_R_BYTE_EXTENDED: Command = Command { number: 53, response: ResponseType::CmdR5, flag: Flags::CommandSdioByte};
pub const SDIO_CMD53_IO_W_BYTE_EXTENDED: Command = Command { number: 53, response: ResponseType::CmdR5, flag: Flags::CommandSdioWriteByte};
pub const SDIO_CMD53_IO_R_BLOCK_EXTENDED: Command = Command { number: 53, response: ResponseType::CmdR5, flag: Flags::CommandSdioBlock};
pub const SDIO_CMD53_IO_W_BLOCK_EXTENDED: Command = Command { number: 53, response: ResponseType::CmdR5, flag: Flags::CommandSdioWriteBlock};