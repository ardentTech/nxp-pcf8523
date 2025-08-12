# NXP PCF8523
`#![no_std]` driver for the NXP PCF8523 RTC and calendar module built on top of the Rust [embedded-hal](https://github.com/rust-embedded/embedded-hal).
RX/TX are handled via I2C, and the module has a fixed address of `0x68`.

### Resources
[Datasheet](www.nxp.com/docs/en/data-sheet/PCF8523.pdf)

### License
* [MIT](https://github.com/ardentTech/nxp-pcf8523/blob/main/LICENSE-MIT)
* [Apache](https://github.com/ardentTech/nxp-pcf8523/blob/main/LICENSE-APACHE)