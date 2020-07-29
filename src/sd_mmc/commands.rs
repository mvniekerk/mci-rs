use std::ops::{BitOr, Add};
use std::marker::PhantomData;
use crate::sd_mmc::command::flags;
use crate::sd_mmc::command::response_type::{CmdR1R6, NoResponse, CmdR3R4, CmdR2, CmdR1B, CmdR7, CmdR5};
use crate::sd_mmc::command::device_mode::{SpiMode, MciMode, DontCareMode};
use crate::sd_mmc::command::flags::{NoFlag, OpenDrain, SingleBlock, MultiBlock, WriteSingleBlock, WriteMultiBlock, SdioByte, SdioWriteByte, SdioBlock, SdioWriteBlock};
use crate::sd_mmc::command::device_type::{SdMmcDevice, MmcDevice, SdDevice, SdioDevice};

pub struct Command<RESP, FLAG, MODE, DEVICE> {
    number: usize,
    _response: PhantomData<RESP>,
    _flag: PhantomData<FLAG>,
    _mode: PhantomData<MODE>,
    _device: PhantomData<DEVICE>
}

// Cmd0(bc): Reset all cards to idle state
pub const SDMMC_SPI_CMD0_GO_IDLE_STATE: Command<CmdR1R6, NoFlag, SpiMode, SdMmcDevice> =
    Command { number: 0, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };
pub const SDMMC_MCI_CMD0_GO_IDLE_STATE: Command<NoResponse, OpenDrain, MciMode, SdMmcDevice> =
    Command { number: 0,  _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// MMC Cmd1(bcr, R3): Ask the card to send its Operating Conditions
pub const MMC_SPI_CMD1_SEND_OP_COND: Command<CmdR1R6, NoFlag, SpiMode, MmcDevice> =
    Command { number: 1,  _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };
pub const MMC_MCI_CMD1_SEND_OP_COND: Command<CmdR3R4, OpenDrain, MciMode, MmcDevice> =
    Command { number: 1,  _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// Cmd2(bcr, R2): Ask the card to send its CID number (stuff but arg 0 used)
pub const SDMMC_CMD2_ALL_SEND_CID: Command<CmdR2, OpenDrain, DontCareMode, SdMmcDevice> =
    Command { number: 2,  _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// SD Cmd3(bcr, R6): Ask the card to publish a new relative address (RCA)
pub const SD_CMD3_SEND_RELATIVE_ADDR: Command<CmdR1R6, OpenDrain, DontCareMode, SdDevice> =
    Command { number: 3,  _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// MMC Cmd3(ac, R1): Assigns relative address to the card
pub const MMC_CMD3_SET_RELATIVE_ADDR: Command<CmdR1R6, NoFlag, DontCareMode, MmcDevice> =
    Command { number: 3,  _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// Cmd4(bc): Program the DSR of all cards (MCI only)
pub const SDMMC_CMD4_SET_DSR: Command<NoResponse, NoFlag, DontCareMode, SdMmcDevice> =
    Command { number: 4, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// MMC Cmd5(ac, R1b): Toggle the card between Sleep state and Standby state.
pub const MMC_CMD5_SLEEP_AWAKE: Command<CmdR1B, NoFlag, DontCareMode, MmcDevice> =
    Command { number: 5,  _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// Cmd7(ac, R1/R1b): Select/Deselect card
// For SD: R1b only from the selected card.
// For MMC: R1 while selecting from Stand-By State to Transfer State;
//           R1b while selecting from Disconnected State to Programming State.
//
pub const SDMMC_CMD7_SELECT_CARD_CMD: Command<CmdR1B, NoFlag, DontCareMode, SdMmcDevice> =
    Command { number: 7, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };
pub const SDMMC_CMD7_DESELECT_CARD_CMD: Command<CmdR1R6, NoFlag, DontCareMode, SdMmcDevice> =
    Command { number: 7, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// MMC Cmd8(adtc, R1): Send EXT_CSD register as a block of data
pub const MMC_CMD8_SEND_EXT_CSD: Command<CmdR1R6, SingleBlock, DontCareMode, MmcDevice> =
    Command { number: 7, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };
// SD Cmd8(bcr, R7) : Send SD Memory Card interface condition
pub const SD_CMD8_SEND_IF_COND: Command<CmdR7, OpenDrain, DontCareMode, SdDevice> =
    Command { number: 8, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// Cmd9 SPI (R1): Addressed card sends its card-specific data (CSD)
pub const SDMMC_SPI_CMD9_SEND_CSD: Command<CmdR1R6, SingleBlock, SpiMode, SdMmcDevice> =
    Command { number: 9, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };
// Cmd9 MCI (ac, R2): Addressed card sends its card-specific data (CSD)
pub const SDMMC_MCI_CMD9_SEND_CSD: Command<CmdR2, NoFlag, MciMode, SdMmcDevice> =
    Command { number: 9, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// Cmd10(ac, R2): Addressed card sends its card identification (CID)
pub const SDMMC_CMD10_SEND_CID: Command<CmdR2, NoFlag, DontCareMode, SdMmcDevice> =
    Command { number: 10, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

//
//  MMC Cmd11(adtc, R1): Read data stream from the card, starting at the given
//  address, until a STOP_TRANSMISSION follows.
//
pub const MMC_CMD11_READ_DAT_UNTIL_STOP: Command<CmdR1R6, NoFlag, DontCareMode, MmcDevice> =
    Command { number: 11, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };
// SD Cmd11 MCI (ac, R1): Voltage switching
pub const SD_CMD11_READ_DAT_UNTIL_STOP: Command<CmdR1R6, NoFlag, DontCareMode, SdDevice> =
    Command { number: 11, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// Cmd12(ac, R1b): Force the card to stop transmission
pub const SDMMC_CMD12_STOP_TRANSMISSION: Command<CmdR1B, NoFlag, DontCareMode, SdMmcDevice> =
    Command { number: 12, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// Cmd13(R2): Addressed card sends its status register.
pub const SDMMC_SPI_CMD13_SEND_STATUS: Command<CmdR2, NoFlag, SpiMode, SdMmcDevice> =
    Command { number: 13, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };
// Cmd13(ac, R1): Addressed card sends its status register.
pub const SDMMC_MCI_CMD13_SEND_STATUS: Command<CmdR1R6, NoFlag, MciMode, SdMmcDevice> =
    Command { number: 13, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// MMC Cmd14(adtc, R1): Read the reversed bus testing data pattern from a card.
pub const MMC_CMD14_BUSTEST_R: Command<CmdR1R6, NoFlag, DontCareMode, MmcDevice> =
    Command { number: 14,  _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// Cmd15(ac): Send an addressed card into the Inactive State.
// Note: It is a ac cmd, but it must be send like bc cmd to open drain
pub const SDMMC_CMD15_GO_INACTIVE_STATE: Command<NoResponse, OpenDrain, DontCareMode, SdMmcDevice> =
    Command {number: 15, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// MMC Cmd19(adtc, R1): Send the bus test data pattern
pub const MMC_CMD19_BUSTEST_W: Command<CmdR1R6, NoFlag, DontCareMode, MmcDevice> =
    Command { number: 19, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// Cmd58(R3): Reads the OCR register of a card
pub const SDMMC_SPI_CMD58_READ_OCR: Command<CmdR3R4, NoFlag, SpiMode, SdMmcDevice> =
    Command { number: 19, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// Cmd59(R1): Turns the CRC option on or off
pub const SDMMC_SPI_CMD59_CRC_ON_OFF: Command<CmdR1R6, NoFlag, SpiMode, SdMmcDevice> =
    Command { number: 59, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };


//
//  --- Block-oriented read commands (class 2) --
//

// Cmd16(ac, R1): Set the block length (in bytes)
pub const SDMMC_CMD16_SET_BLOCKLEN: Command<CmdR1R6, NoFlag, DontCareMode, SdMmcDevice> =
    Command { number: 16, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// Cmd17(adtc, R1): Read single block
pub const SDMMC_CMD17_READ_SINGLE_BLOCK: Command<CmdR1R6, SingleBlock, DontCareMode, SdMmcDevice> =
    Command { number: 17,  _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };
// Cmd18(adtc, R1): Read multiple block
pub const SDMMC_CMD18_READ_MULTIPLE_BLOCK: Command<CmdR1R6, MultiBlock, DontCareMode, SdMmcDevice> =
    Command { number: 18,  _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

//
//  --- Sequential write commands (class 3) ---
//

//  MMC Cmd20(adtc, R1): Write a data stream from the host, starting at the
//  given address, until a STOP_TRANSMISSION follows.
pub const MMC_CMD20_WRITE_DAT_UNTIL_STOP: Command<CmdR1R6, NoFlag, DontCareMode, MmcDevice> =
    Command { number: 20,  _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

//
//  --- Block-oriented write commands (class 4) ---
//

// MMC Cmd23(ac, R1): Set block count
pub const MMC_CMD23_SET_BLOCK_COUNT: Command<CmdR1R6, NoFlag, DontCareMode, MmcDevice> =
    Command { number: 23, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// Cmd24(adtc, R1): Write block
pub const SDMMC_CMD24_WRITE_BLOCK: Command<CmdR1R6, WriteSingleBlock, DontCareMode, SdMmcDevice> =
    Command { number: 24, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// Cmd25(adtc, R1): Write multiple block
pub const SDMMC_CMD25_WRITE_MULTIPLE_BLOCK: Command<CmdR1R6, WriteMultiBlock, DontCareMode, SdMmcDevice> =
    Command { number: 25, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// MMC Cmd26(adtc, R1): Programming of the card identification register.
pub const MMC_CMD26_PROGRAM_CID: Command<CmdR1R6, NoFlag, DontCareMode, MmcDevice> =
    Command { number: 26, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// Cmd27(adtc, R1): Programming of the programmable bits of the CSD.
pub const SDMMC_CMD27_PROGRAM_CSD: Command<CmdR1R6, NoFlag, DontCareMode, SdMmcDevice> =
    Command { number: 27, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

//
//  --- Erase commands  (class 5) ---
//
// SD Cmd32(ac, R1):
pub const SD_CMD32_ERASE_WR_BLK_START: Command<CmdR1R6, NoFlag, DontCareMode, SdDevice> =
    Command { number: 32, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// SD Cmd33(ac, R1):
pub const SD_CMD33_ERASE_WR_BLK_END: Command<CmdR1R6, NoFlag, DontCareMode, SdDevice> =
    Command { number: 33, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// MMC Cmd35(ac, R1):
pub const MMC_CMD35_ERASE_GROUP_START: Command<CmdR1R6, NoFlag, DontCareMode, MmcDevice> =
    Command { number: 35, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// MMC Cmd36(ac, R1):
pub const MMC_CMD36_ERASE_GROUP_END: Command<CmdR1R6, NoFlag, DontCareMode, MmcDevice> =
    Command { number: 36, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// Cmd38(ac, R1B):
pub const SDMMC_CMD38_ERASE: Command<CmdR1B, NoFlag, DontCareMode, SdMmcDevice> =
    Command { number: 38, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

//
//  --- Block Oriented Write Protection Commands (class 6) ---
//

// Cmd28(ac, R1b): Set write protection
pub const SDMMC_CMD28_SET_WRITE_PROT: Command<CmdR1B, NoFlag, DontCareMode, SdMmcDevice> =
    Command { number: 28, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// Cmd29(ac, R1b): Clr write protection
pub const SDMMC_CMD29_CLR_WRITE_PROT: Command<CmdR1B, NoFlag, DontCareMode, SdMmcDevice> =
    Command { number: 29, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// Cmd30(adtc, R1b): Send write protection
pub const SDMMC_CMD30_SEND_WRITE_PROT: Command<CmdR1R6, NoFlag, DontCareMode, SdMmcDevice> =
    Command { number: 30, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

//
//  --- Lock Card (class 7) ---
//

// Cmd42(adtc, R1): Used to set/reset the password or lock/unlock the card.
pub const SDMMC_CMD42_LOCK_UNLOCK: Command<CmdR1R6, NoFlag, DontCareMode, SdMmcDevice> =
    Command { number: 42, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

//
//  --- Application-specific commands (class 8) ---
//

//  Cmd55(ac, R1): Indicate to the card that the next command is an application
//  specific command rather than a standard command.
pub const SDMMC_CMD55_APP_CMD: Command<CmdR1R6, NoFlag, DontCareMode, SdMmcDevice> =
    Command { number: 55, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

//  Cmd 56(adtc, R1): Used either to transfer a data block to the card or to get
//  a data block from the card for general purpose/application specific commands.
pub const SDMMC_CMD56_GEN_CMD: Command<CmdR1R6, NoFlag, DontCareMode, SdMmcDevice> =
    Command { number: 56, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

//  MMC Cmd6(ac, R1b) : Switche the mode of operation of the selected card
//  or modifies the EXT_CSD registers.
pub const MMC_CMD6_SWITCH: Command<CmdR1B, NoFlag, DontCareMode, MmcDevice> =
    Command { number: 6, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

//  SD Cmd6(adtc, R1) : Check switchable function (mode 0)
//  and switch card function (mode 1).
pub const SD_CMD6_SWITCH_FUNC: Command<CmdR1R6, SingleBlock, DontCareMode, SdDevice> =
    Command { number: 6, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// ACMD6(ac, R1): Define the data bus width
pub const SD_ACMD6_SET_BUS_WIDTH: Command<CmdR1R6, NoFlag, DontCareMode, SdDevice> =
    Command { number: 6, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// ACMD13(adtc, R1): Send the SD Status
pub const SD_ACMD13_SD_STATUS: Command<CmdR1R6, NoFlag, DontCareMode, SdDevice> =
    Command { number: 13, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

//  ACMD22(adtc, R1): Send the number of the written (with-out errors) write blocks.
pub const SD_ACMD22_SEND_NUM_WR_BLOCKS: Command<CmdR1R6, NoFlag, DontCareMode, SdDevice> =
    Command { number: 22, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

//  ACMD23(ac, R1): Set the number of write blocks to be pre-erased before writing
pub const SD_ACMD23_SET_WR_BLK_ERASE_COUNT: Command<CmdR1R6, NoFlag, DontCareMode, SdDevice> =
    Command { number: 23, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

//  ACMD41(bcr, R3): Send host capacity support information (HCS) and asks the accessed card to send
//  its operating condition register (OCR) content in the response
pub const SD_MCI_ACMD41_SD_SEND_OP_COND: Command<CmdR3R4, OpenDrain, MciMode, SdDevice> =
    Command { number: 41, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

//  ACMD41(R1): Send host capacity support information (HCS) and activates the card's initialization
//  process
pub const SD_SPI_ACMD41_SD_SEND_OP_COND: Command<CmdR1R6, NoFlag, SpiMode, SdDevice> =
    Command { number: 41, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

//  ACMD42(ac, R1): Connect[1]/Disconnect[0] the 50 KOhm pull-up resistor on CD/DAT3 (pin 1) of the card.
pub const SD_ACMD42_SET_CLR_CARD_DETECT: Command<CmdR1R6, NoFlag, DontCareMode, SdDevice> =
    Command { number: 42, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// ACMD51(adtc, R1): Read the SD Configuration Register (SCR)
pub const SD_ACMD51_SEND_SCR: Command<CmdR1R6, NoFlag, DontCareMode, SdDevice> =
    Command { number: 51, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

//
//  --- I/O mode commands (class 9) ---
//

// MMC Cmd39(ac, R4): Used to write and read 8 bit (register) data fields.
pub const MMC_CMD39_FAST_IO: Command<CmdR3R4, NoFlag, DontCareMode, MmcDevice> =
    Command { number: 39, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// MMC Cmd40(bcr, R5): Set the system into interrupt mode
pub const MMC_CMD40_GO_IRQ_STATE: Command<CmdR5, OpenDrain, DontCareMode, MmcDevice> =
    Command { number: 40, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// SDIO Cmd5(R4): Send operation condition
pub const SDIO_CMD5_SEND_OP_COND: Command<CmdR3R4, OpenDrain, DontCareMode, SdioDevice> =
    Command { number: 5, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// SDIO CMD52(R5): Direct IO read/write
pub const SDIO_CMD52_IO_RW_DIRECT: Command<CmdR5, NoFlag, DontCareMode, SdioDevice> =
    Command { number: 52, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };

// /** SDIO CMD53(R5): Extended IO read/write */
pub const SDIO_CMD53_IO_R_BYTE_EXTENDED: Command<CmdR5, SdioByte, DontCareMode, SdioDevice> =
    Command { number: 53, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };
pub const SDIO_CMD53_IO_W_BYTE_EXTENDED: Command<CmdR5, SdioWriteByte, DontCareMode, SdioDevice> =
    Command { number: 53, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };
pub const SDIO_CMD53_IO_R_BLOCK_EXTENDED: Command<CmdR5, SdioBlock, DontCareMode, SdioDevice> =
    Command { number: 53, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };
pub const SDIO_CMD53_IO_W_BLOCK_EXTENDED: Command<CmdR5, SdioWriteBlock, DontCareMode, SdioDevice> =
    Command { number: 53, _response: PhantomData, _flag: PhantomData, _mode: PhantomData, _device: PhantomData };