use crate::mci::Mci;
use crate::mci_card::MciCard;
use embedded_error::mci::MciError;
use embedded_hal::digital::InputPin;
use embedded_sdmmc::{Block, BlockCount, BlockDevice, BlockIdx};

impl<MCI, WP, DETECT> BlockDevice for MciCard<MCI, WP, DETECT>
where
    MCI: Mci,
    WP: InputPin,
    DETECT: InputPin,
{
    type Error = MciError;

    fn read(
        &self,
        blocks: &mut [Block],
        start_block_idx: BlockIdx,
        _reason: &str,
    ) -> Result<(), Self::Error> {
        let mut transaction =
            self.sd_mmc_init_read_blocks(start_block_idx.0, blocks.len() as u16)?;
        for i in blocks.iter_mut() {
            let mut content = i.contents;
            self.sd_mmc_start_read_blocks(&mut transaction, &mut content, 1)?;
            self.sd_mmc_wait_end_of_read_blocks(false, &mut transaction)?;
        }
        Ok(())
    }

    fn write(&self, blocks: &[Block], start_block_idx: BlockIdx) -> Result<(), Self::Error> {
        let mut transaction =
            self.sd_mmc_init_write_blocks(start_block_idx.0, blocks.len() as u16)?;
        for i in blocks.iter() {
            let content = i.contents;
            self.sd_mmc_start_write_blocks(&mut transaction, &content, 1)?;
            self.sd_mmc_wait_end_of_write_blocks(false, &mut transaction)?;
        }
        Ok(())
    }

    fn num_blocks(&self) -> Result<BlockCount, Self::Error> {
        Ok(BlockCount(self.capacity / 512))
    }
}
