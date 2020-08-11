use crate::sd_mmc::mci::Mci;
use crate::sd_mmc::command::mmc_commands::BusWidth;
use atsamd_hal::target_device::SDHC0;
use bit_field::BitField;
use crate::sd_mmc::command::MciCommand;

pub struct AtsamdMci {
    sdhc: SDHC0
}

impl AtsamdMci {
    pub fn new(sdhc: SDHC0) -> Result<AtsamdMci, ()> {
        Ok(AtsamdMci{ sdhc })
    }

    pub fn reset(&mut self) {
        self.sdhc.srr.modify(|_, w| w.swrstcmd().set_bit());
    }

    pub fn wait_busy(&mut self) -> Result<(), ()> {
        for n in (0u32..=0xFFFFFFFFu32).rev() {
            if n == 0 {
                self.reset();
                return Err(())
            }
            if self.sdhc.psr.read().datll().bits() == 0x1 {
                return Ok(())
            }
        }
        Ok(())
    }
}

impl Mci for AtsamdMci {
    fn init(&mut self) -> Result<(), ()> {
        self.sdhc.srr.modify(|_, w| w.swrstall().set_bit());
        loop {
            if self.sdhc.srr.read().swrstall().bit_is_clear() {
                break;
            }
        }

        /* Set the Data Timeout Register to 2 Mega Cycles */
        self.sdhc.tcr.write(|w| unsafe { w.bits(0xe) });

        /* Set 3v3 power supply */
        self.sdhc.pcr.write(|w|
            w
            .sdbpwr().on()
            .sdbvsel()._3v3()
        );

        self.sdhc.nister().write(|w| unsafe { w.bits(0x01FF) });
        self.sdhc.eister().write(|w| unsafe { w.bits(0x03FF) });

        Ok(())
    }

    /// Send a command
    fn send_command(&mut self, mut cmdr: u16, cmd: u32, arg: u32) -> Result<(), ()> {
        cmdr.set_bits(8..16, cmd as u16);
        let cmd: MciCommand = cmd.into();

        if cmd.have_response() {
            cmdr.set_bits(0..2,
                if cmd.have_136bit_response() { 0x1 }
                    else if cmd.card_may_send_busy() { 0x3 }
                    else { 0x2 }
            );
        }

        self.sdhc.mc1r.modify(|_, w| {
            if cmd.open_drain_broadcast_command() {
                w.opd().set_bit()
            } else {
                w.opd().clear_bit()
            }
        });

        self.sdhc.arg1r.write(|w| unsafe { w.bits(arg) });
        self.sdhc.cr.write(|w| unsafe { w.bits(cmdr)} );

        loop {
            let sr = self.sdhc.eister().read();
            if (
                sr.cmdteo().bit_is_set() ||
                sr.cmdend().bit_is_set() ||
                sr.cmdidx().bit_is_set() ||
                sr.datteo().bit_is_set() ||
                sr.datend().bit_is_set() ||
                sr.adma().bit_is_set()
            ) || (cmd.expect_valid_crc() && (
                sr.cmdcrc().bit_is_set() ||
                sr.datcrc().bit_is_set()
            )) {
                self.reset();
                self.sdhc.eister().write(|w| unsafe { w.bits(0x03FF) } );
                return Err(())
            }
            if self.sdhc.nistr().read().cmdc().bit_is_clear() {
                break;
            }
        }

        if !cmdr.get_bit(5) {
            self.sdhc.nistr().write(|w| w.cmdc().set_bit());
        }

        if cmd.card_may_send_busy() {
            return self.wait_busy();
        }

        Ok(())
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