use embedded_sdmmc::{BlockDevice, BlockCount, Block, BlockIdx};
use crate::mci::Mci;
use embedded_error::mci::MciError;
use crate::mci_card::MciCard;
use embedded_hal::digital::InputPin;

impl<MCI, WP, DETECT> BlockDevice for MciCard<MCI, WP, DETECT>
    where
        MCI: Mci,
        WP: InputPin,
        DETECT: InputPin {
    type Error = ();

    fn read(&self, blocks: &mut [Block], start_block_idx: BlockIdx, reason: &str) -> Result<(), Self::Error> {
        self.sd_mmc_init_read_blocks()
        unimplemented!()
    }

    fn write(&self, blocks: &[Block], start_block_idx: BlockIdx) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn num_blocks(&self) -> Result<BlockCount, Self::Error> {
        unimplemented!()
    }
}