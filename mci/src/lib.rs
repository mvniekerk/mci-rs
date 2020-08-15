#![no_std]
#![allow(deprecated)]

pub mod card_state;
pub mod card_type;
pub mod card_version;
pub mod command;
pub mod commands;
pub mod dummy_input_pin;
pub mod error;
pub mod mci;
#[cfg(feature = "mmc")]
pub mod mmc;
pub mod mode_index;
pub mod registers;
pub mod sd;
pub mod sd_mmc;
#[cfg(feature = "sdio")]
pub mod sdio;
#[cfg(feature = "sdio")]
pub mod sdio_state;
pub mod transfer;
