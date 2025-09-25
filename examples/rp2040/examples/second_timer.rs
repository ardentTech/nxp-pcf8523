//! Second Timer Pulse Mode Example
//!
//! See datasheet section 8.9.4

#![no_std]
#![no_main]

extern crate nxp_pcf8523;

use cortex_m::asm::wfi;
use panic_halt as _;
use rp2040_hal::{pac, Sio, Timer, Watchdog, I2C};
use rp2040_hal::clocks::init_clocks_and_plls;
use rp2040_hal::fugit::RateExtU32;
use rp2040_hal::gpio::{FunctionI2C, FunctionSioOutput, Pin, Pins, PullDown, PullUp};
use rp2040_hal::gpio::bank0::{Gpio13, Gpio2, Gpio3};
use rp2040_hal::pac::I2C1;
use nxp_pcf8523::Pcf8523;
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
    pcf8523.enable_second_interrupt(true).unwrap();

    loop {
        wfi();
    }
}