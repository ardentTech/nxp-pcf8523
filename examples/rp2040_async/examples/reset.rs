//! Demonstrates a software reset and then validates relevant register states according to spec.
//!
//! NOTE: "A reset is automatically generated at power-on."
//!
//! See datasheet section 8.3 table 10

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::i2c::{self, Config, InterruptHandler};
use embassy_rp::peripherals::I2C1;
use embassy_rp::{bind_interrupts, gpio};
use embassy_time::Timer;
use embedded_hal_async::i2c::I2c;
use gpio::{Level, Output};
use nxp_pcf8523::Pcf8523;
use nxp_pcf8523::registers::*;
use nxp_pcf8523::typedefs::{Pcf8523T, Variant};
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
    let i2c = i2c::I2c::new_async(p.I2C1, p.PIN_3, p.PIN_2, Irqs, config);

    let mut pcf8523 = Pcf8523::new(i2c, Pcf8523T {}).await.unwrap();

    match validate_reset(&mut pcf8523).await {
        Ok(_) => {
            loop {
                // slow frequency heartbeat
                led.toggle();
                Timer::after_millis(500).await;
                led.toggle();
                Timer::after_millis(5_500).await;
            }
        }
        Err(reg) => {
            loop {
                // fast frequency reg #
                for _ in 0..(reg) {
                    led.toggle();
                    Timer::after_millis(1_000).await;
                    led.toggle();
                    Timer::after_millis(1_000).await;
                }
            }
        }
    }
}

fn get_bits(byte: u8, num_bits: u8, lsb_offset: u8) -> u8 {
    (byte >> lsb_offset) & ((1 << (num_bits)) - 1)
}

async fn validate_reset<I2C: I2c, V: Variant>(rtc: &mut Pcf8523<I2C, V>) -> Result<(), u8> {
    if rtc.read_reg(PCF8523_CONTROL_1).await.unwrap() != 0b0 {
        return Err(PCF8523_CONTROL_1);
    }

    if rtc.read_reg(PCF8523_CONTROL_2).await.unwrap() != 0b0 {
        return Err(PCF8523_CONTROL_2);
    }

    let control_3 = rtc.read_reg(PCF8523_CONTROL_3).await.unwrap();
    if (control_3 >> 5) != 0b111 || get_bits(control_3, 4, 0) != 0b0 {
        return Err(PCF8523_CONTROL_3);
    }

    if get_bits(rtc.read_reg(PCF8523_SECONDS).await.unwrap(), 1, 7) != 0b1 {
        return Err(PCF8523_SECONDS);
    }

    if get_bits(rtc.read_reg(PCF8523_MINUTE_ALARM).await.unwrap(), 1, 7) != 0b1 {
        return Err(PCF8523_MINUTE_ALARM);
    }

    if get_bits(rtc.read_reg(PCF8523_HOUR_ALARM).await.unwrap(), 1, 7) != 0b1 {
        return Err(PCF8523_HOUR_ALARM);
    }

    if get_bits(rtc.read_reg(PCF8523_DAY_ALARM).await.unwrap(), 1, 7) != 0b1 {
        return Err(PCF8523_DAY_ALARM);
    }

    if get_bits(rtc.read_reg(PCF8523_WEEKDAY_ALARM).await.unwrap(), 1, 7) != 0b1 {
        return Err(PCF8523_WEEKDAY_ALARM);
    }

    if rtc.read_reg(PCF8523_OFFSET).await.unwrap() != 0b0 {
        return Err(PCF8523_OFFSET);
    }

    if rtc.read_reg(PCF8523_TMR_CLKOUT_CTRL).await.unwrap() != 0b0 {
        return Err(PCF8523_TMR_CLKOUT_CTRL);
    }

    if get_bits(rtc.read_reg(PCF8523_TMR_A_FREQ_CTRL).await.unwrap(), 3, 0) != 0b111 {
        return Err(PCF8523_TMR_A_FREQ_CTRL);
    }

    let tmr_b_freq_ctrl = rtc.read_reg(PCF8523_TMR_B_FREQ_CTRL).await.unwrap();
    if get_bits(tmr_b_freq_ctrl, 3, 4) != 0b0 || get_bits(tmr_b_freq_ctrl, 3, 0) != 0b111 {
        return Err(PCF8523_TMR_B_FREQ_CTRL);
    }

    Ok(())
}
