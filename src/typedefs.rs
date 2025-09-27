#[derive(Copy, Clone)]
pub enum ClkOut {
    Frequency32_768Hz = 0x0,
    Frequency16_384Hz = 0x1,
    Frequency8_192Hz = 0x2,
    Frequency4_096Hz = 0x3,
    Frequency1_024Hz = 0x4,
    Frequency32Hz = 0x5,
    Frequency1Hz = 0x6,
    Frequency0Hz = 0x7,
}

#[derive(Copy, Clone)]
pub enum CorrectionMode {
    /// Correction is triggered every minute for faster adjustment but more power consumption.
    /// Pulses are then applied once per second.
    Fast = 0x1,
    /// Correction is triggered every two hours for slower adjustment but less power consumption.
    /// Pulses are then applied once per minute.
    Slow = 0x0,
}

#[derive(Copy, Clone)]
pub enum PowerManagement {
    /// Battery switch-over function is enabled in standard mode; battery low detection function is
    /// enabled.
    SwitchOverStandardOnLowDetectionOn = 0x0,
    /// Battery switch-over function is enabled in direct switching mode; battery low detection
    /// function is enabled.
    SwitchOverDirectOnLowDetectionOn = 0x1,
    /// Battery switch-over function is disabled - only one power supply (VDD); battery low
    /// detection function is enabled.
    SwitchOverOffLowDetectionOn = 0x2,
    /// Battery switch-over function is enabled in standard mode; battery low detection function is
    /// disabled.
    SwitchOverStandardOnLowDetectionOff = 0x4,
    /// Battery switch-over function is enabled in direct switching mode; battery low detection
    /// function is disabled.
    SwitchOverDirectOnLowDetectionOff = 0x5,
    /// Battery switch-over function is disabled - only one power supply (VDD); battery low
    /// detection function is disabled.
    SwitchOverOffLowDetectionOff = 0x7,
}
impl TryFrom<u8> for PowerManagement {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(PowerManagement::SwitchOverStandardOnLowDetectionOn),
            0x1 => Ok(PowerManagement::SwitchOverDirectOnLowDetectionOn),
            0x2 => Ok(PowerManagement::SwitchOverOffLowDetectionOn),
            0x4 => Ok(PowerManagement::SwitchOverStandardOnLowDetectionOff),
            0x5 => Ok(PowerManagement::SwitchOverDirectOnLowDetectionOff),
            0x7 => Ok(PowerManagement::SwitchOverOffLowDetectionOff),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
pub enum TimerSourceClock {
    Frequency4096Hz = 0x0,
    Frequency64Hz = 0x1,
    Frequency1Hz = 0x2,
    Frequency1_60Hz = 0x3,
    Frequency1_3600Hz = 0x4,
}

#[derive(Copy, Clone)]
pub enum LowPulseWidth {
    Width46_875ms = 0x0,
    Width62_500ms = 0x1,
    Width78_125ms = 0x2,
    Width93_750ms = 0x3,
    Width125ms = 0x4,
    Width156_250ms = 0x5,
    Width187_500ms = 0x6,
    Width218_750ms = 0x7,
}

/// PCF8523T chip variant
pub struct Pcf8523T {}

/// PCF8523TK chip variant
pub struct Pcf8523TK {}

/// PCF8523TS chip variant
pub struct Pcf8523TS {}

/// PCF8523U chip variant
pub struct Pcf8523U {}

/// Trait for all chip variants
pub trait Variant {}
impl Variant for Pcf8523T {}
impl Variant for Pcf8523TK {}
impl Variant for Pcf8523TS {}
impl Variant for Pcf8523U {}

/// Trait for chip variants which have a physical CLKOUT/INT1 pin
pub(crate) trait ClkOutInt1Pin {}
impl ClkOutInt1Pin for Pcf8523TS {}
impl ClkOutInt1Pin for Pcf8523U {}

/// Trait for chip variants which have a physical INT2 pin
pub trait Int2Pin {}
impl Int2Pin for Pcf8523TS {}
impl Int2Pin for Pcf8523U {}

#[derive(Copy, Clone)]
pub enum TimerAInterruptMode {
    PermanentlyActive,
    Pulsed,
}
impl From<TimerAInterruptMode> for u8 {
    fn from(value: TimerAInterruptMode) -> Self {
        match value {
            TimerAInterruptMode::PermanentlyActive => 0x0,
            TimerAInterruptMode::Pulsed => 0x1,
        }
    }
}

#[derive(Copy, Clone)]
pub enum TimerBInterruptMode {
    PermanentlyActive,
    Pulsed(LowPulseWidth),
}
impl From<TimerBInterruptMode> for u8 {
    fn from(value: TimerBInterruptMode) -> Self {
        match value {
            TimerBInterruptMode::PermanentlyActive => 0x0,
            TimerBInterruptMode::Pulsed(_) => 0x1,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum TimerMode {
    Countdown,
    Watchdog,
}
impl From<TimerMode> for u8 {
    fn from(value: TimerMode) -> Self {
        match value {
            TimerMode::Countdown => 0x1,
            TimerMode::Watchdog => 0x2,
        }
    }
}

#[derive(Copy, Clone)]
pub struct TimerA {
    pub countdown: u8,
    pub interrupt_mode: TimerAInterruptMode,
    /// When TimerMode::Countdown is selected, the `countdown` value will automatically reload. When
    /// TimerMode::Watchdog is selected, it will not automatically reload.
    pub mode: TimerMode,
    pub source_clock: TimerSourceClock,
}
impl TimerA {
    /// Creates a new TimerA instance.
    pub fn new(
        countdown: u8,
        interrupt_mode: TimerAInterruptMode,
        mode: TimerMode,
        source_clock: TimerSourceClock,
    ) -> Self {
        Self {
            countdown,
            interrupt_mode,
            mode,
            source_clock,
        }
    }
}

#[derive(Copy, Clone)]
pub struct TimerB {
    pub countdown: u8,
    pub interrupt_mode: TimerBInterruptMode,
    pub source_clock: TimerSourceClock,
}
impl TimerB {
    /// Creates a new TimerB instance.
    pub fn new(
        countdown: u8,
        interrupt_mode: TimerBInterruptMode,
        source_clock: TimerSourceClock,
    ) -> Self {
        Self {
            countdown,
            interrupt_mode,
            source_clock,
        }
    }
}
