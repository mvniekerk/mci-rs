# MCI

## Description
MultiMedia Card Interface. MCI supports SD, MMC and SDIO cards.

This crate implements the SDIO, MMC and SD protocols.

## Where to start

Example for ATSAMD51:
```rust
pub fn mci() {
    use mci::mci_card::MciCard;
    use mci_atsamd51::AtsamdMci;

    let mut device: hal::pac::Peripherals = c.device;

    let mci = AtsamdMci::new(device.SDHC0);
    let mut pins = hal::Pins::new(device.PORT);
    // Write protect pin
    let mut wp = pins.wp.into_open_drain_output(&mut pins.port);
    // Card detect pin
    let mut detect = pins.detect.into_open_drain_output(&mut pins.port);
    let card = MciCard::new(
        mci,
        wp, true,       // Write protect pin must be pulled high in order to be protected
        detect, true,   // Detect pin must be pulled high in order to signal a card to be detected
        0               // Slot 0. ATSAMD51 can only support 1 slot in anyway
    );
}
```

## Folder structure

### mci/

The crate that provides and implements the SD/MMC/SDIO protocols

| Folder | Description |
| ------ | ----------- |
| command | Create a command to be sent to the card |
| command/mmc_commands | MMC specific commands |
| command/sd_commands | SD specific commands |
| command/sdio_commands / SDIO specific commands |
| registers | Describing the return values of commands and/or registers |
| registers/sd | SD specific registers |
| registers/sdio | SDIO specific registers |
| sd | SD specific enums and functions |


### mci-atsamd51

Crate for the implementation of MCI interface for ATSAMD51 devices