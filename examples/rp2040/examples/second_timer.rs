//! Second Timer Interrupt Example
//!
//! To run this example as-is, you'll need am Adalogger FeatherWing RTC + SD (https://www.adafruit.com/product/2922)
//! and to solder a jumper from INT to D12. Once flashed and reset, the on-board LED will toggle
//! on/off at 1Hz.
//!
//! See https://github.com/rp-rs/rp-hal/blob/main/rp2040-hal-examples/src/bin/gpio_irq_example.rs
#![no_std]
#![no_main]

extern crate nxp_pcf8523;

use core::cell::RefCell;
use critical_section::Mutex;
use embedded_hal::digital::StatefulOutputPin;
use panic_halt as _;
use rp2040_hal::{gpio, pac, Sio, Watchdog, I2C};
use rp2040_hal::clocks::init_clocks_and_plls;
use rp2040_hal::fugit::RateExtU32;
use rp2040_hal::gpio::{FunctionI2C, FunctionSioInput, FunctionSioOutput, Pin, Pins, PullNone, PullUp};
use rp2040_hal::gpio::bank0::{Gpio2, Gpio3};
use rp2040_hal::gpio::Interrupt::EdgeLow;
use rp2040_hal::pac::{interrupt, I2C1};
use nxp_pcf8523::datetime::Pcf8523DateTime;
use nxp_pcf8523::Pcf8523;
use nxp_pcf8523::typedefs::Pcf8523T;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[unsafe(link_section = ".boot2")]
#[unsafe(no_mangle)]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_GD25Q64CS;

const XOSC_CRYSTAL_FREQ_HZ: u32 = 12_000_000;

type LedPin = Pin<gpio::bank0::Gpio13, FunctionSioOutput, PullNone>;
type IntPin = Pin<gpio::bank0::Gpio12, FunctionSioInput, PullUp>;
type Rtc = Pcf8523<I2C<I2C1, (Pin<Gpio2, FunctionI2C, PullUp>, Pin<Gpio3, FunctionI2C, PullUp>)>, Pcf8523T>;
type IntLedRtc = (IntPin, LedPin, Rtc);
static GLOBAL_PINS: Mutex<RefCell<Option<IntLedRtc>>> = Mutex::new(RefCell::new(None));

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
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let led_pin: Pin<_, FunctionSioOutput, PullNone> = pins.gpio13.reconfigure();
    let int_pin: Pin<_, FunctionSioInput, PullUp> = pins.gpio12.reconfigure();
    int_pin.set_interrupt_enabled(EdgeLow, true);

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
    pcf8523.enable_second_interrupt(false).unwrap();

    critical_section::with(|cs| {
        GLOBAL_PINS.borrow(cs).replace(Some((int_pin, led_pin, pcf8523)));
    });

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::IO_IRQ_BANK0);
    }

    loop {
        cortex_m::asm::wfi();
    }
}


#[allow(static_mut_refs)] // See https://github.com/rust-embedded/cortex-m/pull/561
#[interrupt]
fn IO_IRQ_BANK0() {
    // The `#[interrupt]` attribute covertly converts this to `&'static mut Option<LedAndInt>`
    static mut INT_LED_RTC: Option<IntLedRtc> = None;

    // This is one-time lazy initialisation. We steal the variables given to us
    // via `GLOBAL_PINS`.
    if INT_LED_RTC.is_none() {
        critical_section::with(|cs| {
            *INT_LED_RTC = GLOBAL_PINS.borrow(cs).take();
        });
    }

    if let Some(gpios) = INT_LED_RTC {
        // borrow led and button by *destructuring* the tuple
        // these will be of type `&mut LedPin` and `&mut IntPin`, so we don't have
        // to move them back into the static after we use them
        let (int, led, rtc) = gpios;
        let _ = led.toggle();
        int.clear_interrupt(EdgeLow);
        rtc.clear_second_interrupt().unwrap();
    }
}