
#![no_std]
#![allow(deprecated)]
pub mod card_state;
pub mod card_type;
pub mod card_version;
pub mod command_arguments;
pub mod command_responses;
pub mod commands;
pub mod dummy_input_pin;
pub mod error;
pub mod command_flags;
pub mod mci;
pub mod mci_card;
#[cfg(feature = "mmc")]
pub mod mmc;
pub mod mode_index;
pub mod registers;
pub mod sd;
#[cfg(feature = "sdio")]
pub mod sdio;
#[cfg(feature = "sdio")]
pub mod sdio_state;
pub mod sdmmc;
pub mod transfer;
