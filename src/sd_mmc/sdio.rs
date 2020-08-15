use crate::sd_mmc::sd_mmc::{SdMmcCard, ocr_voltage_support, SD_MMC_TRANS_UNITS, SD_TRANS_MULTIPLIERS};
use crate::sd_mmc::mci::Mci;
use atsamd_hal::hal::digital::v2::InputPin;
use crate::sd_mmc::commands::{SDIO_CMD5_SEND_OP_COND, SDIO_CMD52_IO_RW_DIRECT, SDIO_CMD53_IO_R_BLOCK_EXTENDED, SDIO_CMD53_IO_W_BLOCK_EXTENDED};
use crate::sd_mmc::registers::ocr::OcrRegister;
use crate::sd_mmc::registers::registers::Register;
use crate::sd_mmc::command::sdio_commands::cmd52::{Direction, Cmd52};
use crate::sd_mmc::registers::sdio::cccr::function_select::FunctionSelection;
use crate::sd_mmc::registers::sdio::cccr::bus_interface::{BusInterfaceControlRegister};
use crate::sd_mmc::registers::sdio::cccr::card_capability::CardCapabilityRegister;
use crate::sd_mmc::registers::sdio::cccr::high_speed::HighSpeedRegister;
use crate::sd_mmc::command::mmc_commands::BusWidth;
use crate::sd_mmc::command::sdio_commands::cmd53::Cmd53;

pub const SDIO_CCCR_CIS_PTR: u32 = 0x09;
pub const SDIO_CISTPL_END: u8 = 0xFF;
pub const SDIO_CISTPL_FUNCE: u8 = 0x22;

impl <MCI, WP, DETECT> SdMmcCard<MCI, WP, DETECT>
    where MCI: Mci,
    WP: InputPin,
    DETECT: InputPin
{
    /// Try to get the SDIO card's operating condition
    pub fn sdio_send_operation_condition_command(&mut self) -> Result<(), ()> {
        if self.mci.send_command(SDIO_CMD5_SEND_OP_COND.into(), 0).is_err() {
            // No error but card type not updated
            return Ok(())
        }
        let resp = self.mci.get_response();
        let resp = OcrRegister { val: resp };
        if !resp.number_of_io_functions() {
            // No error but card type not updated
            return Ok(())
        }

        let arg = OcrRegister { val: resp.val & ocr_voltage_support().value() };
        let arg = arg.value();

        // Wait until card is ready
        // Timeout 1s = 400KHz / ((6+4)*8) cycles = 5000 retry
        // TODO use proper delay?
        for i in (0..5000).rev() {
            if i == 0 {
                return Err(()) // TODO proper error
            }
            self.mci.send_command(SDIO_CMD5_SEND_OP_COND.into(), arg)?;
            let resp = OcrRegister { val: self.mci.get_response() };
            if resp.card_powered_up_status() {
                self.card_type.set_sdio(true);
                if resp.memory_present() {
                    self.card_type.set_sd(true);
                }
                break;
            }
        }

        Ok(())
    }

    /// SDIO IO_RW_DIRECT command
    /// # Arguments
    /// * `direction` Read or write
    /// * `function` Function number
    /// * `register_address` Register address
    /// * `read_after_write` Read after write flag
    /// * `write_data` Write data
    pub fn sdio_cmd52(&mut self, direction: Direction, function: FunctionSelection, register_address: u32, read_after_write: bool, write_data: u8) -> Result<u8, ()> {
        let mut arg = Cmd52 { val: 0 };
        arg
            .set_write_data(write_data)
            .set_direction(direction)
            .set_function_number(function as u8)
            .set_read_after_write(read_after_write)
            .set_register_address(register_address);
        self.mci.send_command(SDIO_CMD52_IO_RW_DIRECT.into(), arg.value())?;
        let resp = self.mci.get_response() as u8;
        Ok(resp)
    }

    pub fn sdio_read_cia(&mut self, address: u32, buf: &mut [u8], byte_count: usize) -> Result<(), ()> {
        if byte_count > buf.len() {
            return Err(())  // Going to cause a buffer overflow
        }
        for i in 0..byte_count {
            buf[i] = self.sdio_cmd52(Direction::Read, FunctionSelection::FunctionCia0, address + (i as u32), false, 0)?;
        }
        Ok(())
    }

    pub fn sdio_read_cia_32bits(&mut self, address: u32) -> Result<u32, ()> {
        let mut buf = [0u8; 4];
        self.sdio_read_cia(address, &mut buf, 4)?; // TODO proper error
        let ret =
            ((buf[0] as u32) << 0) +
            ((buf[1] as u32) << 8) +
            ((buf[2] as u32) << 16) +
            ((buf[3] as u32) << 24);
        Ok(ret)
    }

    pub fn sdio_cis_area_in_ccr_address(&mut self) -> Result<u32, ()> {
        self.sdio_read_cia_32bits(SDIO_CCCR_CIS_PTR)
    }

    // TODO it says get max speed but it updates _self_. FIXME
    /// Compute SDIO max transfer speed in Hz and update self.clock.
    pub fn sdio_get_max_speed(&mut self) -> Result<(), ()> {
        let cis_address = self.sdio_cis_area_in_ccr_address()?;
        let mut buf = [0u8; 6];
        let mut addr = cis_address;

        loop {
            // Read a sample of CIA area
            self.sdio_read_cia(addr, &mut buf, 4)?;
            if buf[0] == SDIO_CISTPL_END {
                return Err(()) // TODO Error: Tuple error
            }
            if buf[0] == SDIO_CISTPL_FUNCE && buf[2] == 0x0 {
                break; // Fun0 tuple found
            }
            if buf[1] == 0 {
                return Err(()) // TODO proper error: Tuple error
            }

            // Compute next address
            addr += (buf[1] as u32) -1;
            if addr > (cis_address + 256) {
                return Err(()) // TODO proper error: Out of CIS area
            }
        }

        // Read all Fun0 tuple field: fn0_blk_size & max_tran_speed
        addr -= 3;
        self.sdio_read_cia(addr, &mut buf, 6)?; // TODO proper error

        let tplfe_max_tran_speed = if buf[5] > 0x32 {
            // Error on SDIO register, the high speed is not activated and the clock can't be more
            // than 25MHz. This error is present on specific SDIO card (H&D wireless card - HDG104 WiFi SIP)
            0x32
        } else { buf[5] } as usize;

        // Decode transfer speed in Hz
        let unit = SD_MMC_TRANS_UNITS[tplfe_max_tran_speed & 0x7];
        let mult = SD_TRANS_MULTIPLIERS[(tplfe_max_tran_speed >> 3) & 0xF];
        self.clock = unit * mult * 1000;

        // Note: A combo card shall be a Full-Speed SDIO card
        // which supports upto 25MHz.
        // A SDIO card alone can be:
        // - a Low-Speed SDIO card which supports 400Khz minimum
        // - a Full-Speed SDIO card which supports upto 25MHz
        Ok(())
    }

    /// Switch bus width to mode. self.bus_width is update
    /// Returns final bus_width
    ///
    ///	SD memory cards always supports bus 4bit
    /// SD COMBO card always supports bus 4bit
    /// SDIO Full-Speed alone always supports 4bit
    /// SDIO Low-Speed alone can support 4bit (Optional)
    pub fn sdio_cmd52_switch_to_4_bus_width_mode(&mut self) -> Result<BusWidth, ()> {
        use crate::sd_mmc::registers::sdio::cccr::bus_interface::BusWidth as SdioBusWidth;
        let cccr_cap = CardCapabilityRegister { val:
            self.sdio_cmd52(
                Direction::Read, FunctionSelection::FunctionCia0, CardCapabilityRegister::address() as u32, false, 0
            )?
        };
        if !cccr_cap.low_speed_card_supports_4bit_mode() {
            return Ok(BusWidth::_1BIT);
        }
        let mut bus_ctrl = BusInterfaceControlRegister { val: 0 };
        bus_ctrl.set_bus_width(SdioBusWidth::_4bit);
        self.sdio_cmd52(Direction::Write, FunctionSelection::FunctionCia0, BusInterfaceControlRegister::address() as u32, true, bus_ctrl.value())?;
        self.bus_width = BusWidth::_4BIT;
        Ok(BusWidth::_4BIT)
    }

    /// Enable High Speed mode
    /// self.high_speed updated
    /// self.clock updated
    ///
    /// Returns a true result if put in high speed mode, false if not possible
    pub fn sdio_cmd52_set_high_speed_mode(&mut self) -> Result<bool, ()> {
        let high_speed = HighSpeedRegister {
            val: self.sdio_cmd52(Direction::Read, FunctionSelection::FunctionCia0, HighSpeedRegister::address() as u32, false, 0)?
        };

        // Not supported, not a protocol error
        if !high_speed.supports_high_speed() {
            return Ok(false)
        }

        // TODO: Check if already in high speed using flag otherwise could lead to faulty state

        let mut high_speed = HighSpeedRegister { val: 0 };
        high_speed.set_enable_high_speed(true);

        self.sdio_cmd52(Direction::Write, FunctionSelection::FunctionCia0, HighSpeedRegister::address() as u32, true, high_speed.value())?;
        self.high_speed = true;
        self.clock *= 2;
        Ok(true)
    }

    /// CMD53 - SDIO IO_RW_EXTENDED command
    /// This implementation support only the SDIO multi-byte transfer mode which is similar to the
    /// single block transfer on memory.
    /// Note: The SDIO block transfer mode is optional for SDIO card.
    ///
    pub fn sdio_cmd53_io_rw_extended(&mut self, direction: Direction, function: FunctionSelection, register_address: u16, increment_address: bool, data_size: u16, access_block: bool) -> Result<(), ()> {
        let command: u32 = if direction == Direction::Read { SDIO_CMD53_IO_R_BLOCK_EXTENDED.into()} else { SDIO_CMD53_IO_W_BLOCK_EXTENDED.into() };
        let mut arg = Cmd53::default();

        if data_size == 0 || data_size > 512 {
            return Err(()) // TODO proper error for not having correct size
        }

        arg.set_block_or_bytes_count(data_size % 512)
            .set_address(register_address)
            .set_op_code_increment_address(increment_address.into())
            .set_block_mode(false)
            .set_function_number(function as u8)
            .set_direction(direction);
        self.mci.adtc_start(command, arg.val, data_size, 1, access_block)
    }

    pub fn sdio_read_direct(&mut self, function: FunctionSelection, address: u32) -> Result<u8, ()> {
        self.sd_select_this_device_on_mci_and_configure_mci()?; // TODO proper error
        self.sdio_cmd52(Direction::Read, function, address, false, 0)
    }

    pub fn sdio_write_direct(&mut self, function: FunctionSelection, address: u32, data: u8) -> Result<(), ()> {
        self.sd_select_this_device_on_mci_and_configure_mci()?; // TODO proper error
        self.sdio_cmd52(Direction::Write, function, address, false, data).map(|_| ())   // TODO proper error
    }

    pub fn sdio_read_extended(&mut self, function: FunctionSelection, address: u16, increment_address: bool, destination: &mut [u8], size: u16) -> Result<(), ()> {
        self.sd_select_this_device_on_mci_and_configure_mci()?;
        self.sdio_cmd53_io_rw_extended(Direction::Read, function, address, increment_address, size, true)?; // TODO proper error
        self.mci.read_blocks(destination,1)?;   // TODO proper error
        self.mci.wait_until_read_finished() // TODO proper error
    }

    pub fn sdio_write_extended(&mut self, function: FunctionSelection, address: u16, increment_address: bool, source: &[u8], size: u16) -> Result<(), ()> {
        self.sd_select_this_device_on_mci_and_configure_mci()?; // TODO proper error
        self.sdio_cmd53_io_rw_extended(Direction::Write, function, address, increment_address, size, true)?; // TODO proper error
        self.mci.write_blocks(source, 1)?; // TODO proper error
        self.mci.wait_until_write_finished() // TODO proper error
    }
}
