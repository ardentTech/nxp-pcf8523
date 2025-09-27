//! Demonstrates RTC functionality by reading the current datetime as a timestamp and then toggling
//! the LED state on a 0.5Hz frequency.
//!
//! See datasheet section 8.6

#![no_std]
#![no_main]

extern crate nxp_pcf8523;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::StatefulOutputPin;
use nxp_pcf8523::Pcf8523;
use nxp_pcf8523::datetime::Pcf8523DateTime;
use nxp_pcf8523::typedefs::Pcf8523T;
use panic_halt as _;
use rp2040_hal::clocks::init_clocks_and_plls;
use rp2040_hal::fugit::RateExtU32;
use rp2040_hal::gpio::{FunctionI2C, FunctionSioOutput, Pin, Pins, PullDown, PullUp};
use rp2040_hal::{I2C, Sio, Timer, Watchdog, pac};

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[unsafe(link_section = ".boot2")]
#[unsafe(no_mangle)]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_GD25Q64CS;

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

    let mut led_pin: Pin<_, FunctionSioOutput, PullDown> = pins.gpio13.reconfigure();

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
    let dt = Pcf8523DateTime::new(0, 0, 0, 8, 19, 25).unwrap();
    pcf8523.set_datetime(dt).unwrap();
    pcf8523.start().unwrap();

    loop {
        let _ = pcf8523.now().unwrap().timestamp();
        led_pin.toggle().unwrap();
        timer.delay_ms(2_000);
    }
}
