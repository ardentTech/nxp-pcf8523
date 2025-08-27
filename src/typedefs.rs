pub enum CorrectionMode {
    /// Correction is triggered every minute for faster adjustment but more power consumption.
    /// Pulses are then applied once per second.
    Fast = 0x1,
    /// Correction is triggered every two hours for slower adjustment but less power consumption.
    /// Pulses are then applied once per minute.
    Slow = 0x0
}

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
    SwitchOverOffLowDetectionOff = 0x7
}

pub enum TimerSourceClock {
    Frequency4096Hz = 0x0,
    Frequency64Hz = 0x1,
    Frequency1Hz = 0x2,
    Frequency1_60Hz = 0x3,
    Frequency1_3600Hz = 0x4
}

pub enum PulsedInterrupt {
    Width46_875ms = 0x0,
    Width62_500ms = 0x1,
    Width78_125ms = 0x2,
    Width93_750ms = 0x3,
    Width125ms = 0x4,
    Width156_250ms = 0x5,
    Width187_500ms = 0x6,
    Width218_750ms = 0x7,
}

pub enum InterruptMode {
    PermanentlyActive(TimerSourceClock),
    Pulsed(PulsedInterrupt)
}
impl From<InterruptMode> for u8 {
    fn from(value: InterruptMode) -> Self {
        match value {
            InterruptMode::PermanentlyActive(_) => 0x0,
            InterruptMode::Pulsed(_) => 0x1
        }
    }
}

pub struct TimerB {
    pub interrupt: InterruptMode,
    pub value: u8
}

pub enum TimerAMode {
    Countdown,
    Watchdog(TimerSourceClock)
}
impl From<TimerAMode> for u8 {
    fn from(value: TimerAMode) -> Self {
        match value {
            TimerAMode::Countdown => 0x1,
            TimerAMode::Watchdog(_) => 0x2
        }
    }
}

pub struct TimerA {
    pub interrupt: InterruptMode,
    pub mode: TimerAMode,
    pub value: u8
}