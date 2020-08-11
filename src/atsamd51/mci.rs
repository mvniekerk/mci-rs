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

    pub fn set_speed(&mut self, speed: u32, prog_clock_mode: u8) {
        if self.sdhc.ccr.read().sdclken().bit_is_set() {
            let mut psr = self.sdhc.psr.read();
            while psr.cmdinhc().bit_is_set() || psr.cmdinhd().bit_is_set() {
                psr = self.sdhc.psr.read();
            }
        }
        // let clk_base = CONF_BASE_FREQ;
        let mut clk_base = self.sdhc.ca0r.read().baseclkf().bits();
        let clk_mult = self.sdhc.ca1r.read().clkmult().bits();
        let mut div: u8 = 0;

        // If programmable clock mode is enabled, baseclk is divided by 2
        if clk_mult > 0 {
            clk_base /= 2;
        }

        if prog_clock_mode == 0 {
            // divided clock mode
            self.sdhc.ccr.modify(|_, w| w.clkgsel().clear_bit());
            div = (clk_base / speed) / 2;
        } else {
            // programmable clock mode
            self.sdhc.ccr.modify(|_, w| w.clkgsel().set_bit());
            // Specific constraint for SDHC/SDMMC IP
            // speed = Base Clock * Multi Clock / (div+1) */
            div = (clk_base * (clk_mult + 1)) / speed;
            div = if div > 0 { div - 1 } else { div };
        }

        // Specific constraint for SDHC/SDMMC IP
        // The clock divider (DIV) in SDMMC_CCR must be set to a value different from 0 when HSEN is 1.
        div = if self.sdhc.hc1r().read().hsen().bit_is_set() && div == 0 { 1 } else { div };

        // Set clock divider
        self.sdhc.ccr.modify(|_, w|
            unsafe {
                w
                    .sdclkfsel().bits( div & 0xFF)
                    .usdclkfsel().bits( div >> 8)
            }
        );

        self.sdhc.ccr.modify(|_, w| w.intclken().set_bit());

        // Repeat this step until Clock Stable is 1
        while self.sdhc.ccr.read().intclks().bit_is_clear() {}

        // Output the clock to the card -- Set SD Clock Enable
        self.sdhc.ccr.modify(|_, w| w.sdclken().set_bit());
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

    fn deinit(&mut self) -> Result<(), ()> {
        /// NOP
        Ok(())
    }

    fn select_device(&mut self, _slot: u8, clock: u32, bus_width: BusWidth, high_speed: bool) -> Result<(), ()> {
        self.sdhc.hc1r().modify(|_, w| {
            if high_speed {
                w.hsen().set_bit()
            } else {
                w.hsen().clear_bit()
            }
        });

        if self.sdhc.hc2r().read().pvalen().bit_is_clear() {
            self.set_speed(clock, 0);
        }

        match bus_width {
            BusWidth::_1BIT => self.sdhc.hc1r().modify(|_, w| w.dw().clear_bit()),
            BusWidth::_4BIT => self.sdhc.hc1r().modify(|_, w| w.dw().set_bit()),
            _ => return Err(()) // TODO proper error for invalid argument
        }
        Ok(())
    }

    fn deselect_device(&self) -> Result<(), ()> {
        /// NOP
        Ok(())
    }

    fn get_bus_width(&self, slot: u8) -> Result<BusWidth, ()> {
        match slot {
            0 => Ok(BusWidth::_4BIT),
            _ => Err(()) // TOD proper error for invalid argument
        }
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