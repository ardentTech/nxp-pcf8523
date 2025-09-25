//! Post-Reset Register Validation Example
//!
//! "A reset is automatically generated at power-on."
//!
//! See datasheet section 8.3 table 10

#![no_std]
#![no_main]

extern crate nxp_pcf8523;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::StatefulOutputPin;
use panic_halt as _;
use rp2040_hal::{pac, Sio, Timer, Watchdog, I2C};
use rp2040_hal::clocks::init_clocks_and_plls;
use rp2040_hal::fugit::RateExtU32;
use rp2040_hal::gpio::{FunctionI2C, FunctionSioOutput, Pin, Pins, PullDown, PullUp};
use rp2040_hal::gpio::bank0::{Gpio13, Gpio2, Gpio3};
use rp2040_hal::pac::I2C1;
use nxp_pcf8523::Pcf8523;
use nxp_pcf8523::registers::*;
use nxp_pcf8523::typedefs::Pcf8523T;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[unsafe(link_section = ".boot2")]
#[unsafe(no_mangle)]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_GD25Q64CS;

type LedPin = Pin<Gpio13, FunctionSioOutput, PullDown>;
type Rtc = Pcf8523<I2C<I2C1, (Pin<Gpio2, FunctionI2C, PullUp>, Pin<Gpio3, FunctionI2C, PullUp>)>, Pcf8523T>;

const XOSC_CRYSTAL_FREQ_HZ: u32 = 12_000_000;

#[rp2040_hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
        .ok()
        .unwrap();
    let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led: LedPin = pins.gpio13.reconfigure();

    // I2C
    let sda: Pin<_, FunctionI2C, PullUp> = pins.gpio2.reconfigure();
    let scl: Pin<_, FunctionI2C, PullUp> = pins.gpio3.reconfigure();
    let i2c_bus = I2C::i2c1(
        pac.I2C1,
        sda,
        scl,
        1000.kHz(), // 1 MHz fast-mode plus
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    let mut pcf8523 = Pcf8523::new(i2c_bus, Pcf8523T {}).unwrap();

    match validate_reset(&mut pcf8523) {
        Ok(_) => {
            loop {
                // slow frequency heartbeat
                led.toggle().unwrap();
                timer.delay_ms(500);
                led.toggle().unwrap();
                timer.delay_ms(5_500);
            }
        }
        Err(reg) => {
            loop {
                // fast frequency reg #
                for _ in 0..(reg) {
                    led.toggle().unwrap();
                    timer.delay_ms(1_000);
                    led.toggle().unwrap();
                    timer.delay_ms(1_000);
                }
            }
        }
    }
}

fn get_bits(byte: u8, num_bits: u8, lsb_offset: u8) -> u8 {
    (byte >> lsb_offset) & ((1 << (num_bits)) - 1)
}

fn validate_reset(rtc: &mut Rtc) -> Result<(), u8> {
    if  rtc.read_reg(PCF8523_CONTROL_1).unwrap() != 0b0 { return Err(PCF8523_CONTROL_1) }

    if rtc.read_reg(PCF8523_CONTROL_2).unwrap() != 0b0 { return Err(PCF8523_CONTROL_2) }

    let control_3 = rtc.read_reg(PCF8523_CONTROL_3).unwrap();
    if (control_3 >> 5) != 0b111 || get_bits(control_3, 4, 0) != 0b0 { return Err(PCF8523_CONTROL_3); }

    if get_bits(rtc.read_reg(PCF8523_SECONDS).unwrap(), 1, 7) != 0b1 { return Err(PCF8523_SECONDS) }

    if get_bits(rtc.read_reg(PCF8523_MINUTE_ALARM).unwrap(), 1, 7) != 0b1 { return Err(PCF8523_MINUTE_ALARM) }

    if get_bits(rtc.read_reg(PCF8523_HOUR_ALARM).unwrap(), 1, 7) != 0b1 { return Err(PCF8523_HOUR_ALARM) }

    if get_bits(rtc.read_reg(PCF8523_DAY_ALARM).unwrap(), 1, 7) != 0b1 { return Err(PCF8523_DAY_ALARM) }

    if get_bits(rtc.read_reg(PCF8523_WEEKDAY_ALARM).unwrap(), 1, 7) != 0b1 { return Err(PCF8523_WEEKDAY_ALARM) }

    if rtc.read_reg(PCF8523_OFFSET).unwrap() != 0b0 { return Err(PCF8523_OFFSET) }

    if rtc.read_reg(PCF8523_TMR_CLKOUT_CTRL).unwrap() != 0b0 { return Err(PCF8523_TMR_CLKOUT_CTRL) }

    if get_bits(rtc.read_reg(PCF8523_TMR_A_FREQ_CTRL).unwrap(), 3, 0) != 0b111 {
        return Err(PCF8523_TMR_A_FREQ_CTRL)
    }

    let tmr_b_freq_ctrl = rtc.read_reg(PCF8523_TMR_B_FREQ_CTRL).unwrap();
    if get_bits(tmr_b_freq_ctrl, 3, 4) != 0b0 || get_bits(tmr_b_freq_ctrl, 3, 0) != 0b111 {
        return Err(PCF8523_TMR_B_FREQ_CTRL)
    }

    Ok(())
}