# NXP PCF8523

`#![no_std]` driver for the NXP PCF8523 RTC and calendar module built on top of the
Rust [embedded-hal](https://github.com/rust-embedded/embedded-hal). Supported
I2C modes include standard (100 kHz), fast (400 kHz) and fast+ (1_000 kHz), and the module has a fixed I2C address of
`0x68`.

### Usage

```rust
use nxp_pcf8523::Pcf8523;
use nxp_pcf8523::datetime::Pcf8523DateTime;

// configure I2C bus at 100, 400 or 1_000 kHz...

let mut pcf8523 = Pcf8523::new(i2c_bus, Pcf8523T {}).unwrap();
// 1:41:13PM on 08.21.2025
let dt = Pcf8523DateTime::new(13, 41, 13, 8, 21, 25).unwrap();
pcf8523.set_datetime(dt).unwrap();
pcf8523.set_power_management(PowerManagement::SwitchOverStandardOnLowDetectionOn).unwrap();
pcf8523.start().unwrap();
let now = pcf8523.now().unwrap().timestamp();
```

### Examples

* [RP2040](https://github.com/ardentTech/nxp-pcf8523/tree/main/examples/rp2040)

### Tests

From the root dir: `$ cargo test`

### TODO

* Async

### Resources

* [Datasheet](www.nxp.com/docs/en/data-sheet/PCF8523.pdf)

### Acknowledgements

* [RTClib](https://github.com/adafruit/RTClib)

### License

* [MIT](https://github.com/ardentTech/nxp-pcf8523/blob/main/LICENSE-MIT)
* [Apache](https://github.com/ardentTech/nxp-pcf8523/blob/main/LICENSE-APACHE)