pub enum CalibrationMode {
    /// Correction is triggered every minute for faster adjustment but more power consumption.
    /// Pulses are then applied once per second.
    Fast = 0x1,
    /// Correction is triggered every two hours for slower adjustment but less power consumption.
    /// Pulses are then applied once per minute.
    Slow = 0x0
}