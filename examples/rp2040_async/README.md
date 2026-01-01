# RP2040 Examples

The RP2040 examples are based upon an [Adafruit Feather RP2040 RFM95](https://www.adafruit.com/product/5714) with an
[Adalogger FeatherWing RTC + SD](https://www.adafruit.com/product/2922). The number of board-specific variables
(e.g. LED out, CLKOUT/INT1 in) utilized the examples have been minimized to make porting to other boards easy.

NOTE: This board uses the Pcf8523T chip variant, which does not have an INT2 pin, so Timer B cannot be tested.

### Usage

From this directory:

1. Attach RP2040 feather target to host machine
2. Flash firmware: `$ cargo run --example now`
3. Attach featherwing to feather
4. Press reset btn on featherwing