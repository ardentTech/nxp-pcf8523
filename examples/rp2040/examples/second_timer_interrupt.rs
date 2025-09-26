//! Demonstrates pulse mode for the second timer interrupt.
//!
//! In this example, the PCF8523 CLKOUT/INT1 pin is mapped to D11 and LED to D13. The LED quickly
//! flickers as its state is toggled in the interrupt handler.
//!
//! See datasheet section 8.9.4 and https://github.com/rp-rs/rp-hal/blob/main/rp2040-hal-examples/src/bin/gpio_irq_example.rs.

#![no_std]
#![no_main]

extern crate nxp_pcf8523;

use core::cell::RefCell;
use cortex_m::asm::wfi;
use critical_section::Mutex;
use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use panic_halt as _;
use rp2040_hal::{pac, Sio, Watchdog, I2C};
use rp2040_hal::clocks::init_clocks_and_plls;
use rp2040_hal::fugit::RateExtU32;
use rp2040_hal::gpio::{FunctionI2C, FunctionSioInput, FunctionSioOutput, Pin, Pins, PullNone, PullUp};
use rp2040_hal::gpio::bank0::{Gpio2, Gpio3, Gpio11, Gpio13};
use rp2040_hal::gpio::Interrupt::LevelLow;
use rp2040_hal::pac::{interrupt, I2C1};
use nxp_pcf8523::Pcf8523;
use nxp_pcf8523::typedefs::Pcf8523T;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[unsafe(link_section = ".boot2")]
#[unsafe(no_mangle)]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_GD25Q64CS;

type Int1Pin = Pin<Gpio11, FunctionSioInput, PullUp>;
type LedPin = Pin<Gpio13, FunctionSioOutput, PullNone>;
type Rtc = Pcf8523<I2C<I2C1, (Pin<Gpio2, FunctionI2C, PullUp>, Pin<Gpio3, FunctionI2C, PullUp>)>, Pcf8523T>;
type Int1Led = (Int1Pin, LedPin);
static GLOBAL_PINS: Mutex<RefCell<Option<Int1Led>>> = Mutex::new(RefCell::new(None));

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
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin: LedPin = pins.gpio13.reconfigure();
    led_pin.set_high().unwrap();
    let int1_pin: Int1Pin = pins.gpio11.reconfigure();
    int1_pin.set_interrupt_enabled(LevelLow, true);

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

    let mut pcf8523: Rtc = Pcf8523::new(i2c_bus, Pcf8523T {}).unwrap();
    // TODO need software reset?
    pcf8523.reset().unwrap();
    pcf8523.enable_second_timer_interrupt(true).unwrap();

    // Give away our pins by moving them into the `GLOBAL_PINS` variable.
    // We won't need to access them in the main thread again
    critical_section::with(|cs| {
        GLOBAL_PINS.borrow(cs).replace(Some((int1_pin, led_pin)));
    });

    // Unmask the IO_BANK0 IRQ so that the NVIC interrupt controller
    // will jump to the interrupt function when the interrupt occurs.
    // We do this last so that the interrupt can't go off while
    // it is in the middle of being configured
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::IO_IRQ_BANK0);
    }

    loop {
        wfi();
    }
}

#[allow(static_mut_refs)] // See https://github.com/rust-embedded/cortex-m/pull/561
#[interrupt]
fn IO_IRQ_BANK0() {
    // The `#[interrupt]` attribute covertly converts this to `&'static mut Option<LedAndButton>`
    static mut INT1_LED: Option<Int1Led> = None;

    // This is one-time lazy initialisation. We steal the variables given to us
    // via `GLOBAL_PINS`.
    if INT1_LED.is_none() {
        critical_section::with(|cs| {
            *INT1_LED = GLOBAL_PINS.borrow(cs).take();
        });
    }

    // Need to check if our Option<LedAndButtonPins> contains our pins
    if let Some(gpios) = INT1_LED {
        // borrow led and button by *destructuring* the tuple
        // these will be of type `&mut LedPin` and `&mut ButtonPin`, so we don't have
        // to move them back into the static after we use them
        let (int1, led) = gpios;
        // toggle can't fail, but the embedded-hal traits always allow for it
        // we can discard the return value by assigning it to an unnamed variable
        let _ = led.toggle();
        // Our interrupt doesn't clear itself.
        // Do that now so we don't immediately jump back to this interrupt handler.
        int1.clear_interrupt(LevelLow);
    }
}