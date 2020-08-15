#![no_std]
use mci::command::mmc_commands::BusWidth;
use mci::command::MciCommand;
use mci::mci::Mci;
use atsamd_hal::target_device::SDHC0;
use bit_field::BitField;

pub struct AtsamdMci {
    sdhc: SDHC0,
    trans_pos: u64,
    block_size: u16,
    block_amount: u16,
}

impl AtsamdMci {
    pub fn new(sdhc: SDHC0) -> Result<AtsamdMci, ()> {
        Ok(AtsamdMci {
            sdhc,
            trans_pos: 0,
            block_size: 0,
            block_amount: 0,
        })
    }

    pub fn reset(&mut self) {
        self.sdhc.srr.modify(|_, w| w.swrstcmd().set_bit());
    }

    pub fn wait_busy(&mut self) -> Result<(), ()> {
        for n in (0u32..=0xFFFF_FFFFu32).rev() {
            if n == 0 {
                self.reset();
                return Err(());
            }
            if self.sdhc.psr.read().datll().bits() == 0x1 {
                return Ok(());
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
        let mut div: u32;

        // If programmable clock mode is enabled, baseclk is divided by 2
        if clk_mult > 0 {
            clk_base /= 2;
        }

        if prog_clock_mode == 0 {
            // divided clock mode
            self.sdhc.ccr.modify(|_, w| w.clkgsel().clear_bit());
            div = ((clk_base as u32) / speed) / 2;
        } else {
            // programmable clock mode
            self.sdhc.ccr.modify(|_, w| w.clkgsel().set_bit());
            // Specific constraint for SDHC/SDMMC IP
            // speed = Base Clock * Multi Clock / (div+1) */
            div = ((clk_base as u32) * ((clk_mult as u32) + 1)) / speed;
            div = if div > 0 { div - 1 } else { div };
        }

        // Specific constraint for SDHC/SDMMC IP
        // The clock divider (DIV) in SDMMC_CCR must be set to a value different from 0 when HSEN is 1.
        div = if self.sdhc.hc1r().read().hsen().bit_is_set() && div == 0 {
            1
        } else {
            div
        };

        // Set clock divider
        self.sdhc.ccr.modify(|_, w| unsafe {
            w.sdclkfsel()
                .bits(div as u8)
                .usdclkfsel()
                .bits((div >> 8) as u8)
        });

        self.sdhc.ccr.modify(|_, w| w.intclken().set_bit());

        // Repeat this step until Clock Stable is 1
        while self.sdhc.ccr.read().intclks().bit_is_clear() {}

        // Output the clock to the card -- Set SD Clock Enable
        self.sdhc.ccr.modify(|_, w| w.sdclken().set_bit());
    }

    /// Send a command
    pub fn send_command_execute(&mut self, mut cmdr: u16, cmd: u32, arg: u32) -> Result<(), ()> {
        cmdr.set_bits(8..16, cmd as u16);
        let cmd: MciCommand = cmd.into();

        if cmd.have_response() {
            cmdr.set_bits(
                0..2,
                if cmd.have_136bit_response() {
                    0x1
                } else if cmd.card_may_send_busy() {
                    0x3
                } else {
                    0x2
                },
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
        self.sdhc.cr.write(|w| unsafe { w.bits(cmdr) });

        loop {
            let sr = self.sdhc.eister().read();
            if (sr.cmdteo().bit_is_set()
                || sr.cmdend().bit_is_set()
                || sr.cmdidx().bit_is_set()
                || sr.datteo().bit_is_set()
                || sr.datend().bit_is_set()
                || sr.adma().bit_is_set())
                || (cmd.expect_valid_crc()
                    && (sr.cmdcrc().bit_is_set() || sr.datcrc().bit_is_set()))
            {
                self.reset();
                self.sdhc.eister().write(|w| unsafe { w.bits(0x03FF) });
                return Err(());
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

    pub fn eistr_err(&mut self) -> Result<(), ()> {
        let sr = self.sdhc.eistr().read();
        if sr.datteo().bit_is_set() || sr.datcrc().bit_is_set() || sr.datend().bit_is_set() {
            self.reset();
            return Err(()); // TODO proper error
        }
        Ok(())
    }

    pub fn loop_or_on_eistr_err<F: FnMut(&mut AtsamdMci) -> bool>(
        &mut self,
        mut f: F,
    ) -> Result<(), ()> {
        loop {
            self.eistr_err()?; // TODO proper error
            if f(self) {
                break;
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
        self.sdhc.pcr.write(|w| w.sdbpwr().on().sdbvsel()._3v3());

        self.sdhc.nister().write(|w| unsafe { w.bits(0x01FF) });
        self.sdhc.eister().write(|w| unsafe { w.bits(0x03FF) });

        Ok(())
    }

    fn send_command(&mut self, cmd: u32, arg: u32) -> Result<(), ()> {
        if self.sdhc.psr.read().cmdinhc().bit_is_set() {
            return Err(()); // TODO proper error
        }

        self.sdhc.tmr.modify(|_, w| w.dmaen().clear_bit());
        self.sdhc.bcr.modify(|_, w| unsafe { w.bits(0) });
        self.send_command_execute(0, cmd, arg)
    }

    fn deinit(&mut self) -> Result<(), ()> {
        // NOP
        Ok(())
    }

    fn select_device(
        &mut self,
        _slot: u8,
        clock: u32,
        bus_width: &BusWidth,
        high_speed: bool,
    ) -> Result<(), ()> {
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
            _ => return Err(()), // TODO proper error for invalid argument
        }
        Ok(())
    }

    fn deselect_device(&mut self, _slot: u8) -> Result<(), ()> {
        // NOP
        Ok(())
    }

    fn get_bus_width(&mut self, slot: u8) -> Result<BusWidth, ()> {
        match slot {
            0 => Ok(BusWidth::_4BIT),
            _ => Err(()), // TOD proper error for invalid argument
        }
    }

    fn is_high_speed_capable(&mut self) -> Result<bool, ()> {
        Ok(self.sdhc.ca0r.read().hssup().bit_is_set())
    }

    /// Send 74 clock cycles on the line.
    /// Note: It is required after card plug and before card install.
    fn send_clock(&mut self) -> Result<(), ()> {
        for _m in 0..5000u32 {
            // Nop
        }
        Ok(())
    }

    fn get_response(&mut self) -> u32 {
        self.sdhc.rr[0].read().cmdresp().bits()
    }

    fn get_response128(&mut self) -> [u32; 4] {
        [
            self.sdhc.rr[0].read().cmdresp().bits(),
            self.sdhc.rr[1].read().cmdresp().bits(),
            self.sdhc.rr[2].read().cmdresp().bits(),
            self.sdhc.rr[3].read().cmdresp().bits(),
        ]
    }

    fn adtc_start(
        &mut self,
        command: u32,
        argument: u32,
        block_size: u16,
        block_amount: u16,
        _access_in_blocks: bool,
    ) -> Result<(), ()> {
        let psr = self.sdhc.psr.read();
        // Check Command Inhibit (CMD/DAT) in the Present State register
        if psr.cmdinhc().bit_is_set() || psr.cmdinhd().bit_is_set() {
            return Err(()); // TODO proper error why we're erroring
        }

        let command: MciCommand = command.into();
        if !command.sdio_multi_byte_transfer()
            && !command.sdio_block_mode_transfer()
            && !command.single_block_data_transfer()
            && !command.multi_block_data_transfer()
        {
            return Err(()); // TODO proper error
        }

        self.sdhc.tmr.write(|w| {
            if command.data_write_command() {
                w.dtdsel().write();
            } else {
                w.dtdsel().read();
            }
            if command.sdio_multi_byte_transfer() {
                w.msbsel().single();
            } else if command.sdio_block_mode_transfer() {
                w.bcen().enable().msbsel().multiple();
            } else if command.single_block_data_transfer() {
                w.msbsel().single();
            } else if command.multi_block_data_transfer() {
                w.bcen().enable().msbsel().multiple();
            }
            w
        });

        self.sdhc
            .bsr
            .write(|w| unsafe { w.blocksize().bits(block_size) }.boundary()._4k());
        self.sdhc
            .bcr
            .write(|w| unsafe { w.bcnt().bits(block_amount) });

        self.block_amount = block_amount;
        self.block_size = block_size;
        self.trans_pos = 0;

        self.send_command_execute(1 << 5, command.val, argument)
    }

    fn adtc_stop(&self, _command: u32, _argument: u32) -> Result<(), ()> {
        // Nop
        Ok(())
    }

    fn read_word(&mut self) -> Result<(u32, u8), ()> {
        let nbytes: u8 =
            if ((self.block_size as u64) * (self.block_amount as u64)) - self.trans_pos > 4 {
                (self.block_size % 4) as u8
            } else {
                4
            };

        if self.trans_pos % (self.block_size as u64) == 0 {
            self.loop_or_on_eistr_err(|f| f.sdhc.nistr().read().brdrdy().bit_is_set())?;
        }

        // Read data
        let val = self.sdhc.bdpr.read().bits()
            & match nbytes {
                3 => 0xFF_FFFF,
                2 => 0xFFFF,
                1 => 0xFF,
                _ => 0xFFFF_FFFF,
            };

        self.trans_pos += nbytes as u64;

        if (self.block_size as u64) * (self.block_amount as u64) > self.trans_pos {
            return Ok((val, nbytes));
        }

        // Wait end of transfer
        self.loop_or_on_eistr_err(|f| f.sdhc.nistr().read().trfc().bit_is_set())?;
        self.sdhc.nistr().modify(|_, w| w.trfc().yes());
        Ok((val, nbytes))
    }

    fn write_word(&mut self, val: u32) -> Result<bool, ()> {
        let nbytes = 4u64; // self.block_size & 0x3 ? 1 : 4
        if self.trans_pos % (self.block_size as u64) == 0 {
            self.loop_or_on_eistr_err(|f| f.sdhc.nistr().read().bwrrdy().bit_is_set())?;
        }

        // Write data
        self.sdhc.bdpr.write(|w| unsafe { w.bits(val) });
        self.trans_pos += nbytes;

        if (self.block_size as u64) * (self.block_amount as u64) > self.trans_pos {
            return Ok(true);
        }

        // Wait end of transfer
        self.loop_or_on_eistr_err(|f| f.sdhc.nistr().read().trfc().bit_is_set())?; //TODO proper error
        self.sdhc.nistr().modify(|_, w| w.trfc().yes());
        Ok(true)
    }

    fn read_blocks(&mut self, destination: &mut [u8], number_of_blocks: u16) -> Result<bool, ()> {
        let mut data = (number_of_blocks as u64) * (self.block_size as u64);
        let len = data as usize;
        let mut index = 0usize;

        while data > 0 {
            let (val, nbytes) = self.read_word()?;
            for m in 0..nbytes {
                let mm = m as usize;
                if mm + index >= len {
                    break;
                }
                destination[index + mm] = val.get_bits((mm * 8)..((mm + 1) * 8)) as u8;
            }
            let nbytes = if (nbytes as u64) > data {
                (self.block_size % (nbytes as u16)) as u8
            } else {
                nbytes
            };
            index += nbytes as usize;
            data -= nbytes as u64;
        }
        Ok(true)
    }

    fn write_blocks(&mut self, write_data: &[u8], number_of_blocks: u16) -> Result<bool, ()> {
        let mut data = (number_of_blocks as u64) * (self.block_size as u64);
        let len = data as usize;
        let mut index = 0usize;

        while data > 0 {
            let mut nbytes = 0u8;
            let mut val = 0u32;
            for m in 0..4 {
                let mm = m as usize;
                if mm + index >= len {
                    break;
                }
                val <<= 8;
                nbytes += 1;
                val |= write_data[index + mm] as u32;
            }
            self.write_word(val)?;
            data -= nbytes as u64;
            index += nbytes as usize;
        }
        Ok(true)
    }

    fn wait_until_read_finished(&self) -> Result<(), ()> {
        // Nop
        Ok(())
    }

    fn wait_until_write_finished(&self) -> Result<(), ()> {
        // Nop
        Ok(())
    }
}
