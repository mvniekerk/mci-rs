use crate::sd_mmc::command::flags::{
    CommandFlag, MultiBlock, NoFlag, OpenDrain, SdioBlock, SdioByte, SdioWriteBlock, SdioWriteByte,
    SingleBlock, WriteMultiBlock, WriteSingleBlock,
};
use crate::sd_mmc::command::response_type::{
    CmdR1B, CmdR1R6, CmdR2, CmdR3R4, CmdR5, CmdR7, NoResponse, Response,
};

pub struct Command<RESP: Response, FLAG: CommandFlag> {
    number: u32,
    response: RESP,
    flag: FLAG,
}

impl<RESP: Response, FLAG: CommandFlag> From<Command<RESP, FLAG>> for u32 {
    fn from(val: Command<RESP, FLAG>) -> Self {
        val.number | val.response.val() | val.flag.val()
    }
}

// Cmd0(bc): Reset all cards to idle state
pub const SDMMC_SPI_CMD0_GO_IDLE_STATE: Command<CmdR1R6, NoFlag> = Command {
    number: 0,
    response: CmdR1R6,
    flag: NoFlag,
};
pub const SDMMC_MCI_CMD0_GO_IDLE_STATE: Command<NoResponse, OpenDrain> = Command {
    number: 0,
    response: NoResponse,
    flag: OpenDrain,
};

// MMC Cmd1(bcr, R3): Ask the card to send its Operating Conditions
pub const MMC_SPI_CMD1_SEND_OP_COND: Command<CmdR1R6, NoFlag> = Command {
    number: 1,
    response: CmdR1R6,
    flag: NoFlag,
};
pub const MMC_MCI_CMD1_SEND_OP_COND: Command<CmdR3R4, OpenDrain> = Command {
    number: 1,
    response: CmdR3R4,
    flag: OpenDrain,
};

// Cmd2(bcr, R2): Ask the card to send its CID number (stuff but arg 0 used)
pub const SDMMC_CMD2_ALL_SEND_CID: Command<CmdR2, OpenDrain> = Command {
    number: 2,
    response: CmdR2,
    flag: OpenDrain,
};

// SD Cmd3(bcr, R6): Ask the card to publish a new relative address (RCA)
pub const SD_CMD3_SEND_RELATIVE_ADDR: Command<CmdR1R6, OpenDrain> = Command {
    number: 3,
    response: CmdR1R6,
    flag: OpenDrain,
};

// MMC Cmd3(ac, R1): Assigns relative address to the card
pub const MMC_CMD3_SET_RELATIVE_ADDR: Command<CmdR1R6, NoFlag> = Command {
    number: 3,
    response: CmdR1R6,
    flag: NoFlag,
};

// Cmd4(bc): Program the DSR of all cards (MCI only)
pub const SDMMC_CMD4_SET_DSR: Command<NoResponse, NoFlag> = Command {
    number: 4,
    response: NoResponse,
    flag: NoFlag,
};

// MMC Cmd5(ac, R1b): Toggle the card between Sleep state and Standby state.
pub const MMC_CMD5_SLEEP_AWAKE: Command<CmdR1B, NoFlag> = Command {
    number: 5,
    response: CmdR1B,
    flag: NoFlag,
};

// Cmd7(ac, R1/R1b): Select/Deselect card
// For SD: R1b only from the selected card.
// For MMC: R1 while selecting from Stand-By State to Transfer State;
//           R1b while selecting from Disconnected State to Programming State.
//
pub const SDMMC_CMD7_SELECT_CARD_CMD: Command<CmdR1B, NoFlag> = Command {
    number: 7,
    response: CmdR1B,
    flag: NoFlag,
};
pub const SDMMC_CMD7_DESELECT_CARD_CMD: Command<CmdR1R6, NoFlag> = Command {
    number: 7,
    response: CmdR1R6,
    flag: NoFlag,
};

// MMC Cmd8(adtc, R1): Send EXT_CSD register as a block of data
pub const MMC_CMD8_SEND_EXT_CSD: Command<CmdR1R6, SingleBlock> = Command {
    number: 7,
    response: CmdR1R6,
    flag: SingleBlock,
};
// SD Cmd8(bcr, R7) : Send SD Memory Card interface condition
pub const SD_CMD8_SEND_IF_COND: Command<CmdR7, OpenDrain> = Command {
    number: 8,
    response: CmdR7,
    flag: OpenDrain,
};

// Cmd9 SPI (R1): Addressed card sends its card-specific data (CSD)
pub const SDMMC_SPI_CMD9_SEND_CSD: Command<CmdR1R6, SingleBlock> = Command {
    number: 9,
    response: CmdR1R6,
    flag: SingleBlock,
};
// Cmd9 MCI (ac, R2): Addressed card sends its card-specific data (CSD)
pub const SDMMC_MCI_CMD9_SEND_CSD: Command<CmdR2, NoFlag> = Command {
    number: 9,
    response: CmdR2,
    flag: NoFlag,
};

// Cmd10(ac, R2): Addressed card sends its card identification (CID)
pub const SDMMC_CMD10_SEND_CID: Command<CmdR2, NoFlag> = Command {
    number: 10,
    response: CmdR2,
    flag: NoFlag,
};

//
//  MMC Cmd11(adtc, R1): Read data stream from the card, starting at the given
//  address, until a STOP_TRANSMISSION follows.
//
pub const MMC_CMD11_READ_DAT_UNTIL_STOP: Command<CmdR1R6, NoFlag> = Command {
    number: 11,
    response: CmdR1R6,
    flag: NoFlag,
};
// SD Cmd11 MCI (ac, R1): Voltage switching
pub const SD_CMD11_READ_DAT_UNTIL_STOP: Command<CmdR1R6, NoFlag> = Command {
    number: 11,
    response: CmdR1R6,
    flag: NoFlag,
};

// Cmd12(ac, R1b): Force the card to stop transmission
pub const SDMMC_CMD12_STOP_TRANSMISSION: Command<CmdR1B, NoFlag> = Command {
    number: 12,
    response: CmdR1B,
    flag: NoFlag,
};

// Cmd13(R2): Addressed card sends its status register.
pub const SDMMC_SPI_CMD13_SEND_STATUS: Command<CmdR2, NoFlag> = Command {
    number: 13,
    response: CmdR2,
    flag: NoFlag,
};
// Cmd13(ac, R1): Addressed card sends its status register.
pub const SDMMC_MCI_CMD13_SEND_STATUS: Command<CmdR1R6, NoFlag> = Command {
    number: 13,
    response: CmdR1R6,
    flag: NoFlag,
};

// MMC Cmd14(adtc, R1): Read the reversed bus testing data pattern from a card.
pub const MMC_CMD14_BUSTEST_R: Command<CmdR1R6, NoFlag> = Command {
    number: 14,
    response: CmdR1R6,
    flag: NoFlag,
};

// Cmd15(ac): Send an addressed card into the Inactive State.
// Note: It is a ac cmd, but it must be send like bc cmd to open drain
pub const SDMMC_CMD15_GO_INACTIVE_STATE: Command<NoResponse, OpenDrain> = Command {
    number: 15,
    response: NoResponse,
    flag: OpenDrain,
};

// MMC Cmd19(adtc, R1): Send the bus test data pattern
pub const MMC_CMD19_BUSTEST_W: Command<CmdR1R6, NoFlag> = Command {
    number: 19,
    response: CmdR1R6,
    flag: NoFlag,
};

// Cmd58(R3): Reads the OCR register of a card
pub const SDMMC_SPI_CMD58_READ_OCR: Command<CmdR3R4, NoFlag> = Command {
    number: 19,
    response: CmdR3R4,
    flag: NoFlag,
};

// Cmd59(R1): Turns the CRC option on or off
pub const SDMMC_SPI_CMD59_CRC_ON_OFF: Command<CmdR1R6, NoFlag> = Command {
    number: 59,
    response: CmdR1R6,
    flag: NoFlag,
};

//
//  --- Block-oriented read commands (class 2) --
//

// Cmd16(ac, R1): Set the block length (in bytes)
pub const SDMMC_CMD16_SET_BLOCKLEN: Command<CmdR1R6, NoFlag> = Command {
    number: 16,
    response: CmdR1R6,
    flag: NoFlag,
};

// Cmd17(adtc, R1): Read single block
pub const SDMMC_CMD17_READ_SINGLE_BLOCK: Command<CmdR1R6, SingleBlock> = Command {
    number: 17,
    response: CmdR1R6,
    flag: SingleBlock,
};
// Cmd18(adtc, R1): Read multiple block
pub const SDMMC_CMD18_READ_MULTIPLE_BLOCK: Command<CmdR1R6, MultiBlock> = Command {
    number: 18,
    response: CmdR1R6,
    flag: MultiBlock,
};

//
//  --- Sequential write commands (class 3) ---
//

//  MMC Cmd20(adtc, R1): Write a data stream from the host, starting at the
//  given address, until a STOP_TRANSMISSION follows.
pub const MMC_CMD20_WRITE_DAT_UNTIL_STOP: Command<CmdR1R6, NoFlag> = Command {
    number: 20,
    response: CmdR1R6,
    flag: NoFlag,
};

//
//  --- Block-oriented write commands (class 4) ---
//

// MMC Cmd23(ac, R1): Set block count
pub const MMC_CMD23_SET_BLOCK_COUNT: Command<CmdR1R6, NoFlag> = Command {
    number: 23,
    response: CmdR1R6,
    flag: NoFlag,
};

// Cmd24(adtc, R1): Write block
pub const SDMMC_CMD24_WRITE_BLOCK: Command<CmdR1R6, WriteSingleBlock> = Command {
    number: 24,
    response: CmdR1R6,
    flag: WriteSingleBlock,
};

// Cmd25(adtc, R1): Write multiple block
pub const SDMMC_CMD25_WRITE_MULTIPLE_BLOCK: Command<CmdR1R6, WriteMultiBlock> = Command {
    number: 25,
    response: CmdR1R6,
    flag: WriteMultiBlock,
};

// MMC Cmd26(adtc, R1): Programming of the card identification register.
pub const MMC_CMD26_PROGRAM_CID: Command<CmdR1R6, NoFlag> = Command {
    number: 26,
    response: CmdR1R6,
    flag: NoFlag,
};

// Cmd27(adtc, R1): Programming of the programmable bits of the CSD.
pub const SDMMC_CMD27_PROGRAM_CSD: Command<CmdR1R6, NoFlag> = Command {
    number: 27,
    response: CmdR1R6,
    flag: NoFlag,
};

//
//  --- Erase commands  (class 5) ---
//
// SD Cmd32(ac, R1):
pub const SD_CMD32_ERASE_WR_BLK_START: Command<CmdR1R6, NoFlag> = Command {
    number: 32,
    response: CmdR1R6,
    flag: NoFlag,
};

// SD Cmd33(ac, R1):
pub const SD_CMD33_ERASE_WR_BLK_END: Command<CmdR1R6, NoFlag> = Command {
    number: 33,
    response: CmdR1R6,
    flag: NoFlag,
};

// MMC Cmd35(ac, R1):
pub const MMC_CMD35_ERASE_GROUP_START: Command<CmdR1R6, NoFlag> = Command {
    number: 35,
    response: CmdR1R6,
    flag: NoFlag,
};

// MMC Cmd36(ac, R1):
pub const MMC_CMD36_ERASE_GROUP_END: Command<CmdR1R6, NoFlag> = Command {
    number: 36,
    response: CmdR1R6,
    flag: NoFlag,
};

// Cmd38(ac, R1B):
pub const SDMMC_CMD38_ERASE: Command<CmdR1B, NoFlag> = Command {
    number: 38,
    response: CmdR1B,
    flag: NoFlag,
};

//
//  --- Block Oriented Write Protection Commands (class 6) ---
//

// Cmd28(ac, R1b): Set write protection
pub const SDMMC_CMD28_SET_WRITE_PROT: Command<CmdR1B, NoFlag> = Command {
    number: 28,
    response: CmdR1B,
    flag: NoFlag,
};

// Cmd29(ac, R1b): Clr write protection
pub const SDMMC_CMD29_CLR_WRITE_PROT: Command<CmdR1B, NoFlag> = Command {
    number: 29,
    response: CmdR1B,
    flag: NoFlag,
};

// Cmd30(adtc, R1b): Send write protection
pub const SDMMC_CMD30_SEND_WRITE_PROT: Command<CmdR1R6, NoFlag> = Command {
    number: 30,
    response: CmdR1R6,
    flag: NoFlag,
};

//
//  --- Lock Card (class 7) ---
//

// Cmd42(adtc, R1): Used to set/reset the password or lock/unlock the card.
pub const SDMMC_CMD42_LOCK_UNLOCK: Command<CmdR1R6, NoFlag> = Command {
    number: 42,
    response: CmdR1R6,
    flag: NoFlag,
};

//
//  --- Application-specific commands (class 8) ---
//

//  Cmd55(ac, R1): Indicate to the card that the next command is an application
//  specific command rather than a standard command.
pub const SDMMC_CMD55_APP_CMD: Command<CmdR1R6, NoFlag> = Command {
    number: 55,
    response: CmdR1R6,
    flag: NoFlag,
};

//  Cmd 56(adtc, R1): Used either to transfer a data block to the card or to get
//  a data block from the card for general purpose/application specific commands.
pub const SDMMC_CMD56_GEN_CMD: Command<CmdR1R6, NoFlag> = Command {
    number: 56,
    response: CmdR1R6,
    flag: NoFlag,
};

//  MMC Cmd6(ac, R1b) : Switche the mode of operation of the selected card
//  or modifies the EXT_CSD registers.
pub const MMC_CMD6_SWITCH: Command<CmdR1B, NoFlag> = Command {
    number: 6,
    response: CmdR1B,
    flag: NoFlag,
};

//  SD Cmd6(adtc, R1) : Check switchable function (mode 0)
//  and switch card function (mode 1).
pub const SD_CMD6_SWITCH_FUNC: Command<CmdR1R6, SingleBlock> = Command {
    number: 6,
    response: CmdR1R6,
    flag: SingleBlock,
};

// ACMD6(ac, R1): Define the data bus width
pub const SD_ACMD6_SET_BUS_WIDTH: Command<CmdR1R6, NoFlag> = Command {
    number: 6,
    response: CmdR1R6,
    flag: NoFlag,
};

// ACMD13(adtc, R1): Send the SD Status
pub const SD_ACMD13_SD_STATUS: Command<CmdR1R6, NoFlag> = Command {
    number: 13,
    response: CmdR1R6,
    flag: NoFlag,
};

//  ACMD22(adtc, R1): Send the number of the written (with-out errors) write blocks.
pub const SD_ACMD22_SEND_NUM_WR_BLOCKS: Command<CmdR1R6, NoFlag> = Command {
    number: 22,
    response: CmdR1R6,
    flag: NoFlag,
};

//  ACMD23(ac, R1): Set the number of write blocks to be pre-erased before writing
pub const SD_ACMD23_SET_WR_BLK_ERASE_COUNT: Command<CmdR1R6, NoFlag> = Command {
    number: 23,
    response: CmdR1R6,
    flag: NoFlag,
};

//  ACMD41(bcr, R3): Send host capacity support information (HCS) and asks the accessed card to send
//  its operating condition register (OCR) content in the response
pub const SD_MCI_ACMD41_SD_SEND_OP_COND: Command<CmdR3R4, OpenDrain> = Command {
    number: 41,
    response: CmdR3R4,
    flag: OpenDrain,
};

//  ACMD41(R1): Send host capacity support information (HCS) and activates the card's initialization
//  process
pub const SD_SPI_ACMD41_SD_SEND_OP_COND: Command<CmdR1R6, NoFlag> = Command {
    number: 41,
    response: CmdR1R6,
    flag: NoFlag,
};

//  ACMD42(ac, R1): Connect[1]/Disconnect[0] the 50 KOhm pull-up resistor on CD/DAT3 (pin 1) of the card.
pub const SD_ACMD42_SET_CLR_CARD_DETECT: Command<CmdR1R6, NoFlag> = Command {
    number: 42,
    response: CmdR1R6,
    flag: NoFlag,
};

// ACMD51(adtc, R1): Read the SD Configuration Register (SCR)
pub const SD_ACMD51_SEND_SCR: Command<CmdR1R6, NoFlag> = Command {
    number: 51,
    response: CmdR1R6,
    flag: NoFlag,
};

//
//  --- I/O mode commands (class 9) ---
//

// MMC Cmd39(ac, R4): Used to write and read 8 bit (register) data fields.
pub const MMC_CMD39_FAST_IO: Command<CmdR3R4, NoFlag> = Command {
    number: 39,
    response: CmdR3R4,
    flag: NoFlag,
};

// MMC Cmd40(bcr, R5): Set the system into interrupt mode
pub const MMC_CMD40_GO_IRQ_STATE: Command<CmdR5, OpenDrain> = Command {
    number: 40,
    response: CmdR5,
    flag: OpenDrain,
};

// SDIO Cmd5(R4): Send operation condition
pub const SDIO_CMD5_SEND_OP_COND: Command<CmdR3R4, OpenDrain> = Command {
    number: 5,
    response: CmdR3R4,
    flag: OpenDrain,
};

// SDIO CMD52(R5): Direct IO read/write
pub const SDIO_CMD52_IO_RW_DIRECT: Command<CmdR5, NoFlag> = Command {
    number: 52,
    response: CmdR5,
    flag: NoFlag,
};

// SDIO CMD53(R5): Extended IO read/write */
pub const SDIO_CMD53_IO_R_BYTE_EXTENDED: Command<CmdR5, SdioByte> = Command {
    number: 53,
    response: CmdR5,
    flag: SdioByte,
};
pub const SDIO_CMD53_IO_W_BYTE_EXTENDED: Command<CmdR5, SdioWriteByte> = Command {
    number: 53,
    response: CmdR5,
    flag: SdioWriteByte,
};
pub const SDIO_CMD53_IO_R_BLOCK_EXTENDED: Command<CmdR5, SdioBlock> = Command {
    number: 53,
    response: CmdR5,
    flag: SdioBlock,
};
pub const SDIO_CMD53_IO_W_BLOCK_EXTENDED: Command<CmdR5, SdioWriteBlock> = Command {
    number: 53,
    response: CmdR5,
    flag: SdioWriteBlock,
};
