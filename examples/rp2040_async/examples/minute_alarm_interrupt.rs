//! NOTE: this doesn't work bc the alarm never triggers and pulls the INT1 line low. The setup
//! is seemingly identical to the blocking example, and I'm out of ideas as to what could be going
//! on. I can toggle between blocking and async, and blocking works consistently and async never
//! does while the test suite continues to pass.
//!
//! Try: dump+hash all registers from the RTC after `set_minute_alarm` and compare against blocking
//! example.
//!
//! Demonstrates the minute alarm interrupt by turning the LED off one minute after starting the
//! PCF8523 module.
//!
//! See datasheet section 8.7.1

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::i2c::{Config, I2c, InterruptHandler};
use embassy_rp::peripherals::I2C1;
use embassy_rp::{bind_interrupts, gpio};
use gpio::{Level, Output};
use nxp_pcf8523::Pcf8523;
use nxp_pcf8523::datetime::Pcf8523DateTime;
use nxp_pcf8523::typedefs::Pcf8523T;
#[allow(unused_imports)]
use panic_halt as _;

bind_interrupts!(struct Irqs {
    I2C1_IRQ => InterruptHandler<I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_13, Level::Low);
    let mut int1_pin = Input::new(p.PIN_11, Pull::Up);

    let mut config = Config::default();
    config.frequency = 1_000_000;
    let i2c = I2c::new_async(p.I2C1, p.PIN_3, p.PIN_2, Irqs, config);

    let mut pcf8523 = Pcf8523::new(i2c, Pcf8523T {}).await.unwrap();
    let dt = Pcf8523DateTime::new(0, 0, 0, 8, 19, 25).unwrap();
    pcf8523.set_datetime(dt).await.unwrap();
    pcf8523.start().await.unwrap();
    pcf8523.enable_minute_alarm(1).await.unwrap();
    // enable_minute_alarm(...) will disable clkout which briefly pulls CLKOUT/INT1 low. to see the
    // LED turn off after 1m, set it high after enabling the alarm.
    led.set_high();

    loop {
        int1_pin.wait_for_falling_edge().await;
        led.set_low();
        pcf8523.clear_alarm_interrupt().await.unwrap();
    }
}
