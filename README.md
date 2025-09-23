# NXP PCF8523
`#![no_std]` driver for the NXP PCF8523 RTC and calendar module built on top of the Rust [embedded-hal](https://github.com/rust-embedded/embedded-hal). Supported
I2C modes include standard (100 kHz), fast (400 kHz) and fast+ (1000 kHz), and the module has a fixed I2C address of
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
let now = pcf8523.now().unwrap();
let timestamp = now.timestamp();
```

### Example
The current example is based upon an [Adafruit Feather RP2040 RFM95](https://www.adafruit.com/product/5714) with an
[Adalogger FeatherWing RTC + SD](https://www.adafruit.com/product/2922) (which has an on-board PCF8523):
1. `$ cd examples/rp2040`
2. Attach RP2040 feather target to host
3. `$ cargo run --example now`
4. Attach featherwing to feather
5. Press reset btn on featherwing
6. The on-board LED will toggle on a 2s interval

### Tests
From the root dir: `$ cargo test`

### TODO
* Hardware tests

### Resources
[Datasheet](www.nxp.com/docs/en/data-sheet/PCF8523.pdf)

### Acknowledgements
* [RTClib](https://github.com/adafruit/RTClib)

### License
* [MIT](https://github.com/ardentTech/nxp-pcf8523/blob/main/LICENSE-MIT)
* [Apache](https://github.com/ardentTech/nxp-pcf8523/blob/main/LICENSE-APACHE)