//! Demonstrates the second timer interrupt in pulse mode by toggling the LED state on a 1Hz frequency.
//!
//! See datasheet section 8.9.4

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
use nxp_pcf8523::typedefs::TimerInterruptMode::Pulsed;
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
    pcf8523.start_second_timer(Pulsed).await.unwrap();
    // start_second_timer(...) will disable CLKOUT and briefly pull CLKOUT/INT1 low.
    // this triggers the interrupt handler, so set the LED high after enabling the alarm.
    led.set_high();

    loop {
        int1_pin.wait_for_falling_edge().await;
        led.toggle();
    }
}
