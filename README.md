# NXP PCF8523
`#![no_std]` driver for the NXP PCF8523 RTC and calendar module built on top of the Rust [embedded-hal](https://github.com/rust-embedded/embedded-hal).
RX and TX are handled via I2C, and the module has a fixed address of `0x68`.

### Usage
```rust
use nxp_pcf8523::Pcf8523;
use nxp_pcf8523::datetime::Pcf8523DateTime;

// configure I2C bus...

let mut pcf8523 = Pcf8523::new(i2c_bus).unwrap();
// 10:41:13 08.21.2025
let dt = Pcf8523DateTime::new(10, 41, 13, 8, 21, 25).unwrap();
pcf8523.set_datetime(dt).unwrap();
pcf8523.start().unwrap();
let now = pcf8523.now().unwrap();
let timestamp = now.timestamp();
```

### Resources
[Datasheet](www.nxp.com/docs/en/data-sheet/PCF8523.pdf)

### Acknowledgements
* [RTClib](https://github.com/adafruit/RTClib)

### License
* [MIT](https://github.com/ardentTech/nxp-pcf8523/blob/main/LICENSE-MIT)
* [Apache](https://github.com/ardentTech/nxp-pcf8523/blob/main/LICENSE-APACHE)