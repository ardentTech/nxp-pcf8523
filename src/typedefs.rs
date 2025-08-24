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