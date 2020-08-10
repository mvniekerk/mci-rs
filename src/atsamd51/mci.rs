use crate::sd_mmc::mci::Mci;
use crate::sd_mmc::command::mmc_commands::BusWidth;

pub struct AtsamdMci {
}

impl Mci for AtsamdMci {
    fn init(&self) -> Result<(), ()> {
        // mci_dev->hw = hw;

        // hri_sdhc_set_SRR_SWRSTALL_bit(hw);
        // while (hri_sdhc_get_SRR_SWRSTALL_bit(hw))
        // ;
        //
        // /* Set the Data Timeout Register to 2 Mega Cycles */
        // hri_sdhc_write_TCR_reg(hw, SDHC_TCR_DTCVAL(0xE));
        //
        // /* Set 3v3 power supply */
        // hri_sdhc_write_PCR_reg(hw, SDHC_PCR_SDBPWR_ON | SDHC_PCR_SDBVSEL_3V3);
        //
        // hri_sdhc_set_NISTER_reg(hw, SDHC_NISTER_MASK);
        // hri_sdhc_set_EISTER_reg(hw, SDHC_EISTER_MASK);
        //
        // return ERR_NONE;
        unimplemented!()

    }

    fn deinit(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn select_device(&self, clock: u32, bus_width: BusWidth, high_speed: bool) -> Result<(), ()> {
        unimplemented!()
    }

    fn deselect_device(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn get_bus_width(&self) -> Result<BusWidth, ()> {
        unimplemented!()
    }

    fn is_high_speed_capable(&self) -> Result<bool, ()> {
        unimplemented!()
    }

    fn send_clock(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn get_response(&self) -> u32 {
        unimplemented!()
    }

    fn get_response127(&self) -> u128 {
        unimplemented!()
    }

    fn adtc_start(&self, command: u32, argument: u32, block_size: u16, block_amount: u16, access_in_blocks: bool) -> Result<(), ()> {
        unimplemented!()
    }

    fn adtc_stop(&self, command: u32, argument: u32) -> Result<(), ()> {
        unimplemented!()
    }

    fn read_word(&self) -> Result<u32, ()> {
        unimplemented!()
    }

    fn write_word(&self, val: u32) -> Result<bool, ()> {
        unimplemented!()
    }

    fn read_blocks(&self, destination: &mut [u8], number_of_blocks: usize) -> Result<bool, ()> {
        unimplemented!()
    }

    fn write_blocks(&self, data: &[u8], number_of_blocks: usize) -> Result<bool, ()> {
        unimplemented!()
    }

    fn wait_until_read_finished(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn wait_until_write_finished(&self) -> Result<(), ()> {
        unimplemented!()
    }
}