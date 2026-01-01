//! Demonstrates RTC functionality by reading the current datetime as a timestamp and then toggling
//! the LED state on a 0.5Hz frequency.
//!
//! See datasheet section 8.6

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::i2c::{Config, I2c, InterruptHandler};
use embassy_rp::peripherals::I2C1;
use embassy_rp::{bind_interrupts, gpio};
use embassy_time::Timer;
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

    let mut config = Config::default();
    config.frequency = 1_000_000;
    let i2c = I2c::new_async(p.I2C1, p.PIN_3, p.PIN_2, Irqs, config);

    let mut pcf8523 = Pcf8523::new(i2c, Pcf8523T {}).await.unwrap();
    let dt = Pcf8523DateTime::new(0, 0, 0, 8, 19, 25).unwrap();
    pcf8523.set_datetime(dt).await.unwrap();
    pcf8523.start().await.unwrap();

    loop {
        let _ = pcf8523.now().await.unwrap().timestamp();
        led.toggle();
        Timer::after_secs(2).await;
    }
}
