use crate::card_state::CardState;
use crate::command_arguments::mmc::BusWidth;
use crate::commands::{
    SDMMC_CMD12_STOP_TRANSMISSION, SDMMC_CMD17_READ_SINGLE_BLOCK, SDMMC_CMD18_READ_MULTIPLE_BLOCK,
    SDMMC_CMD24_WRITE_BLOCK, SDMMC_CMD25_WRITE_MULTIPLE_BLOCK, SDMMC_MCI_CMD13_SEND_STATUS,
    SDMMC_MCI_CMD9_SEND_CSD,
};
use crate::mci::Mci;
use crate::mci_card::MciCard;
use crate::registers::csd::CsdRegister;
use crate::registers::sd::card_status::CardStatusRegister;
use crate::transfer::TransferTransaction;
use embedded_hal::digital::v2::InputPin;

pub const SD_MMC_BLOCK_SIZE: u32 = 512;

impl<MCI, WP, DETECT> MciCard<MCI, WP, DETECT>
where
    MCI: Mci,
    WP: InputPin,
    DETECT: InputPin,
{
    /// CMD9: Card sends its card specific data (CSD)
    /// self.csd is updated
    pub fn sd_mmc_cmd9_mci(&mut self) -> Result<(), ()> {
        let arg = (self.rca as u32) << 16;
        self.mci.send_command(SDMMC_MCI_CMD9_SEND_CSD.into(), arg)?;
        self.csd = CsdRegister {
            val: self.mci.get_response128(),
        };
        Ok(())
    }

    /// CMD13: Get status register.
    /// Waits for the clear of the busy flag
    pub fn sd_mmc_cmd13_get_status_and_wait_for_ready_for_data_flag(
        &mut self,
    ) -> Result<CardStatusRegister, ()> {
        let mut status = CardStatusRegister::default();
        // TODO maybe proper timeout
        for i in (0..200_000u32).rev() {
            if i == 0 {
                return Err(()); // TODO proper timeout error
            }
            self.mci
                .send_command(SDMMC_MCI_CMD13_SEND_STATUS.into(), (self.rca as u32) << 16)?;
            status = CardStatusRegister {
                val: self.mci.get_response(),
            };
            if status.ready_for_data() {
                break;
            }
        }
        Ok(status)
    }

    pub fn sd_mmc_deselect_this_device(&mut self) -> Result<(), ()> {
        self.mci.deselect_device(self.slot)
    }

    pub fn sd_mmc_select_this_device_on_mci_and_configure_mci(&mut self) -> Result<(), ()> {
        self.mci
            .select_device(self.slot, self.clock, &self.bus_width, self.high_speed)
        // TODO proper error
    }

    /// Select this instance's card slot and initialize the associated driver
    pub fn sd_mmc_select_slot(&mut self) -> Result<(), ()> {
        // Check card detection
        if self.wp.is_high().map_err(|_| ())? != self.wp_high_activated {
            // TODO proper error for pin check
            if self.state == CardState::Debounce {
                // TODO Timeout stop?
            }
            self.state = CardState::NoCard;
            return Err(()); // TODO no card error
        }

        if self.state == CardState::Debounce {
            if false {
                // TODO check if timed out
                return Err(()); // TODO proper timeout
            }
            self.state = CardState::Init;
            // Set 1-bit bus width and low clock for initialization
            self.clock = 400_000;
            self.bus_width = BusWidth::_1BIT;
            self.high_speed = false;
        }
        if self.state == CardState::Unusable {
            return Err(()); // TODO proper error
        }
        self.sd_mmc_select_this_device_on_mci_and_configure_mci()?; // TODO proper error
        if self.state == CardState::Init {
            Ok(())
        } else {
            Ok(())
        } // TODO if it is still ongoing should return ongoing
    }

    pub fn sd_mmc_init_read_blocks(
        &mut self,
        start: u32,
        blocks_amount: u16,
    ) -> Result<TransferTransaction, ()> {
        self.sd_mmc_select_this_device_on_mci_and_configure_mci()?;
        // Wait for data status
        self.sd_mmc_cmd13_get_status_and_wait_for_ready_for_data_flag()?;
        let cmd: u32 = if blocks_amount > 1 {
            SDMMC_CMD18_READ_MULTIPLE_BLOCK.into()
        } else {
            SDMMC_CMD17_READ_SINGLE_BLOCK.into()
        };

        // SDSC Card (CCS=0) uses byte unit address,
        // SDHC and SDXC Cards (CCS=1) use block unit address (512 Bytes unit).
        let arg = if self.card_type.high_capacity() {
            start
        } else {
            start * SD_MMC_BLOCK_SIZE
        };
        self.mci
            .adtc_start(cmd, arg, SD_MMC_BLOCK_SIZE as u16, blocks_amount, true)?;
        Ok(TransferTransaction {
            amount: blocks_amount,
            remaining: blocks_amount,
        })
    }

    pub fn sd_mmc_start_read_blocks(
        &mut self,
        transaction: &mut TransferTransaction,
        destination: &mut [u8],
        amount_of_blocks: u16,
    ) -> Result<(), ()> {
        if self.mci.read_blocks(destination, amount_of_blocks).is_err() {
            transaction.remaining = 0;
            return Err(()); // TODO proper read error
        }
        transaction.remaining -= amount_of_blocks;
        Ok(())
    }

    pub fn sd_mmc_wait_end_of_read_blocks(
        &mut self,
        abort: bool,
        transaction: &mut TransferTransaction,
    ) -> Result<(), ()> {
        self.mci.wait_until_read_finished()?;
        if abort {
            transaction.remaining = 0;
        } else if transaction.remaining > 0 {
            return Ok(());
        }

        // All blocks are transferred then stop read operation
        if transaction.remaining == 1 {
            return Ok(());
        }

        // WORKAROUND for no compliance card (Atmel Internal ref. !MMC7 !SD19)
        // The errors on this cmmand must be ignored and one retry can be necessary in SPI mode
        // for non-complying card
        if self
            .mci
            .adtc_stop(SDMMC_CMD12_STOP_TRANSMISSION.into(), 0)
            .is_err()
        {
            self.mci
                .adtc_stop(SDMMC_CMD12_STOP_TRANSMISSION.into(), 0)?; // TODO proper error
        }
        Ok(())
    }

    pub fn sd_mmc_init_write_blocks(
        &mut self,
        start: u32,
        blocks_amount: u16,
    ) -> Result<TransferTransaction, ()> {
        self.sd_mmc_select_this_device_on_mci_and_configure_mci()?;
        if self.write_protected()? {
            return Err(()); // TODO proper write protection error
        }

        let cmd: u32 = if blocks_amount > 1 {
            SDMMC_CMD25_WRITE_MULTIPLE_BLOCK.into()
        } else {
            SDMMC_CMD24_WRITE_BLOCK.into()
        };

        // SDSC Card (CCS=0) uses byte unit address,
        // SDHC and SDXC Cards (CCS=1) use block unit address (512 Bytes unit).
        let arg = if self.card_type.high_capacity() {
            start
        } else {
            start * SD_MMC_BLOCK_SIZE
        };

        self.mci
            .adtc_start(cmd, arg, SD_MMC_BLOCK_SIZE as u16, blocks_amount, true)?; // TODO proper error

        let resp = CardStatusRegister {
            val: self.mci.get_response(),
        };
        if resp.write_protect_violation() {
            return Err(()); // TODO proper error
        }

        Ok(TransferTransaction {
            remaining: blocks_amount,
            amount: blocks_amount,
        })
    }

    pub fn sd_mmc_start_write_blocks(
        &mut self,
        transaction: &mut TransferTransaction,
        data: &[u8],
        blocks_amount: u16,
    ) -> Result<(), ()> {
        if self.mci.write_blocks(data, blocks_amount).is_err() {
            transaction.remaining = 0;
            return Err(()); // TODO proper error
        }
        transaction.remaining -= blocks_amount;
        Ok(())
    }

    pub fn sd_mmc_wait_end_of_write_blocks(
        &mut self,
        abort: bool,
        transaction: &mut TransferTransaction,
    ) -> Result<(), ()> {
        self.mci.wait_until_write_finished()?; // TODO proper error
        if abort {
            transaction.remaining = 0;
        } else if transaction.remaining > 0 {
            return Ok(()); // TODO proper return?
        }

        // All blocks are transferred then stop write operation
        if transaction.remaining == 1 {
            // Single block transfer, then nothing to do
            return Ok(()); // TODO proper return?
        }

        // Note SPI multi-block writes terminate using a special token, not a STOP_TRANSMISSION request
        self.mci
            .adtc_stop(SDMMC_CMD12_STOP_TRANSMISSION.into(), 0)?; // TODO proper error
        Ok(())
    }
}
