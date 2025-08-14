use embedded_hal::i2c::ErrorKind::Other;
use embedded_hal::i2c::Operation;
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
use nxp_pcf8523::datetime::{DateTime, DayOfWeek, Month};
use nxp_pcf8523::driver::{Pcf8523, PCF8523_I2C_ADDRESS};
use nxp_pcf8523::typedefs::{Pcf8523Error, Pcf8523Interrupt, PowerManagement};
use nxp_pcf8523::registers::*;

#[test]
fn clear_bsf_interrupt_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_3, 0b1100_0111),
        i2c_reg_write(PCF8523_CONTROL_3, 0b1100_0011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    driver.clear_interrupt(Pcf8523Interrupt::BatterySwitchOver).unwrap();
    i2c.done();
}

#[test]
fn clear_countdown_timer_a_interrupt_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_2, 0b1100_0111),
        i2c_reg_write(PCF8523_CONTROL_2, 0b1000_0111),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    driver.clear_interrupt(Pcf8523Interrupt::CountdownTimerA).unwrap();
    i2c.done();
}

#[test]
fn clear_countdown_timer_b_interrupt_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_2, 0b1010_0111),
        i2c_reg_write(PCF8523_CONTROL_2, 0b1000_0111),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    driver.clear_interrupt(Pcf8523Interrupt::CountdownTimerB).unwrap();
    i2c.done();
}

#[test]
fn clear_second_interrupt_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_2, 0b1001_0111),
        i2c_reg_write(PCF8523_CONTROL_2, 0b1000_0111),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    driver.clear_interrupt(Pcf8523Interrupt::Second).unwrap();
    i2c.done();
}

#[test]
fn clear_watchdog_timer_a_interrupt_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_2, 0b1010_0101),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    driver.clear_interrupt(Pcf8523Interrupt::WatchdogTimerA).unwrap();
    i2c.done();
}

#[test]
fn freeze_rtc_time_circuits_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_1, 0b0101_1110),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0111_1110),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    driver.freeze_rtc_time_circuits(true).unwrap();
    i2c.done();
}

#[test]
fn get_battery_status_low_false_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_3, 0b0),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let low = driver.get_battery_status().unwrap();
    assert!(!low);
    i2c.done();
}

#[test]
fn get_battery_status_low_true_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_3, 0b100),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let low = driver.get_battery_status().unwrap();
    assert!(low);
    i2c.done();
}

#[test]
fn get_clock_integrity_guaranteed_false_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_SECONDS, 0b1011_0010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let guaranteed = driver.get_clock_integrity_guaranteed().unwrap();
    assert!(!guaranteed);
    i2c.done();
}

#[test]
fn get_clock_integrity_guaranteed_true_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_SECONDS, 0b0011_0010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let guaranteed = driver.get_clock_integrity_guaranteed().unwrap();
    assert!(guaranteed);
    i2c.done();
}

#[test]
fn get_correction_interrupt_generated_false_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_1, 0b0),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let guaranteed = driver.get_correction_interrupt_generated().unwrap();
    assert!(!guaranteed);
    i2c.done();
}

#[test]
fn get_correction_interrupt_generated_true_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_1, 0b1),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let guaranteed = driver.get_correction_interrupt_generated().unwrap();
    assert!(guaranteed);
    i2c.done();
}

#[test]
fn get_datetime_ok() {
    let expectations = [
        I2cTransaction::transaction_start(PCF8523_I2C_ADDRESS),
        // all payloads are bcd-encoded
        I2cTransaction::write(PCF8523_I2C_ADDRESS, [PCF8523_SECONDS].to_vec()),
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        I2cTransaction::write(PCF8523_I2C_ADDRESS, [PCF8523_MINUTES].to_vec()),
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        I2cTransaction::write(PCF8523_I2C_ADDRESS, [PCF8523_HOURS].to_vec()),
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        I2cTransaction::write(PCF8523_I2C_ADDRESS, [PCF8523_DAYS].to_vec()),
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        I2cTransaction::write(PCF8523_I2C_ADDRESS, [PCF8523_WEEKDAYS].to_vec()),
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        I2cTransaction::write(PCF8523_I2C_ADDRESS, [PCF8523_MONTHS].to_vec()),
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b1].to_vec()),
        I2cTransaction::write(PCF8523_I2C_ADDRESS, [PCF8523_YEARS].to_vec()),
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        I2cTransaction::transaction_end(PCF8523_I2C_ADDRESS),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let dt = driver.get_datetime().unwrap();
    assert_eq!(dt.seconds, 0);
    i2c.done();
}

#[test]
fn get_day_of_month_leap_year_ok() {
    // TODO
}

#[test]
fn get_day_of_month_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_DAYS, 0b1_1111),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let day = driver.get_day_of_month().unwrap();
    assert_eq!(day, 25u8);
    i2c.done();
}

#[test]
fn get_day_of_week_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_WEEKDAYS, 0b110),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let day = driver.get_day_of_week().unwrap();
    assert_eq!(day, 6u8);
    i2c.done();
}

#[test]
fn get_interrupt_enabled_alarm_false() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_1, 0b0),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let enabled = driver.get_interrupt_enabled(Pcf8523Interrupt::Alarm).unwrap();
    assert!(!enabled);
    i2c.done();
}

#[test]
fn get_interrupt_enabled_alarm_true() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_1, 0b10),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let enabled = driver.get_interrupt_enabled(Pcf8523Interrupt::Alarm).unwrap();
    assert!(enabled);
    i2c.done();
}

#[test]
fn get_interrupt_enabled_battery_switch_over_false() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_3, 0b00),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let enabled = driver.get_interrupt_enabled(Pcf8523Interrupt::BatterySwitchOver).unwrap();
    assert!(!enabled);
    i2c.done();
}

#[test]
fn get_interrupt_enabled_battery_switch_over_true() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_3, 0b10),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let enabled = driver.get_interrupt_enabled(Pcf8523Interrupt::BatterySwitchOver).unwrap();
    assert!(enabled);
    i2c.done();
}

#[test]
fn get_interrupt_enabled_countdown_timer_a_false() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_2, 0b00),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let enabled = driver.get_interrupt_enabled(Pcf8523Interrupt::CountdownTimerA).unwrap();
    assert!(!enabled);
    i2c.done();
}

#[test]
fn get_interrupt_enabled_countdown_timer_a_true() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_2, 0b10),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let enabled = driver.get_interrupt_enabled(Pcf8523Interrupt::CountdownTimerA).unwrap();
    assert!(enabled);
    i2c.done();
}

#[test]
fn get_interrupt_enabled_countdown_timer_b_false() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_2, 0b0),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let enabled = driver.get_interrupt_enabled(Pcf8523Interrupt::CountdownTimerB).unwrap();
    assert!(!enabled);
    i2c.done();
}

#[test]
fn get_interrupt_enabled_countdown_timer_b_true() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_2, 0b1),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let enabled = driver.get_interrupt_enabled(Pcf8523Interrupt::CountdownTimerB).unwrap();
    assert!(enabled);
    i2c.done();
}

#[test]
fn get_interrupt_enabled_second_false() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_1, 0b011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let enabled = driver.get_interrupt_enabled(Pcf8523Interrupt::Second).unwrap();
    assert!(!enabled);
    i2c.done();
}

#[test]
fn get_interrupt_enabled_second_true() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_1, 0b111),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let enabled = driver.get_interrupt_enabled(Pcf8523Interrupt::Second).unwrap();
    assert!(enabled);
    i2c.done();
}

#[test]
fn get_interrupt_enabled_watchdog_timer_a_false() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_2, 0b011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let enabled = driver.get_interrupt_enabled(Pcf8523Interrupt::WatchdogTimerA).unwrap();
    assert!(!enabled);
    i2c.done();
}

#[test]
fn get_interrupt_enabled_watchdog_timer_a_true() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_2, 0b100),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let enabled = driver.get_interrupt_enabled(Pcf8523Interrupt::WatchdogTimerA).unwrap();
    assert!(enabled);
    i2c.done();
}

#[test]
fn get_minutes_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_MINUTES, 0b0001_1011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let minutes = driver.get_minutes().unwrap();
    assert_eq!(minutes, 21u8);
    i2c.done();
}

#[test]
fn get_month_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_MONTHS, 0b1_0001),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let month = driver.get_month().unwrap();
    assert_eq!(month, 11u8);
    i2c.done();
}

#[test]
fn get_seconds_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_SECONDS, 0b0101_1000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let seconds = driver.get_seconds().unwrap();
    assert_eq!(seconds, 58u8);
    i2c.done();
}

#[test]
fn get_year_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_YEARS, 0b10_0101),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let year = driver.get_year().unwrap();
    assert_eq!(year, 25u8);
    i2c.done();
}

#[test]
fn read_reg_i2c_err() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_1, 0b0101_0110).with_error(Other),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let err = driver.read_reg(PCF8523_CONTROL_1).unwrap_err();
    assert_eq!(err, Pcf8523Error::I2C(Other));
    i2c.done();
}

#[test]
fn read_reg_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_1, 0b0101_0110),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let val = driver.read_reg(PCF8523_CONTROL_1).unwrap();
    assert_eq!(val, 0b0101_0110);
    i2c.done();
}

#[test]
fn reset_ok() {
    let expectations = [
        i2c_reg_write(PCF8523_CONTROL_1, 0b0101_1000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    driver.software_reset().unwrap();
    i2c.done();
}

#[test]
fn select_oscillator_capacitor_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_1, 0b0101_0110),
        i2c_reg_write(PCF8523_CONTROL_1, 0b1101_0110),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    driver.select_oscillator_capacitor(true).unwrap();
    i2c.done();
}

#[test]
fn select_hour_mode_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_1, 0b1001_0010),
        i2c_reg_write(PCF8523_CONTROL_1, 0b1001_1010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    driver.select_hour_mode(true).unwrap();
    i2c.done();
}

#[test]
fn select_power_management_ok() {
    let expectations = [
        i2c_reg_read(PCF8523_CONTROL_3, 0b0001_1010),
        i2c_reg_write(PCF8523_CONTROL_3, 0b0101_1010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    driver.select_power_management(PowerManagement::SwitchOverDisabledLowDetectionEnabled).unwrap();
    i2c.done();
}

#[test]
fn set_datetime_ok() {
    let expectations = [
        I2cTransaction::transaction_start(PCF8523_I2C_ADDRESS),
        i2c_reg_write(PCF8523_SECONDS, 0b11_1011),
        i2c_reg_write(PCF8523_MINUTES, 0b1111),
        i2c_reg_write(PCF8523_HOURS, 0b10),
        i2c_reg_write(PCF8523_DAYS, 0b1011),
        i2c_reg_write(PCF8523_WEEKDAYS, 0b11),
        i2c_reg_write(PCF8523_MONTHS, 0b1010),
        i2c_reg_write(PCF8523_YEARS, 0b10_1101),
        I2cTransaction::transaction_end(PCF8523_I2C_ADDRESS),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let datetime = DateTime::new(59, 15, 2, 11, DayOfWeek::Wednesday, Month::October, 45).unwrap();
    driver.set_datetime(datetime).unwrap();
    i2c.done();
}

#[test]
fn write_reg_i2c_err() {
    let expectations = [
        i2c_reg_write(PCF8523_CONTROL_1, 0b0101_0110).with_error(Other),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    let err = driver.write_reg(PCF8523_CONTROL_1, 0b0101_0110).unwrap_err();
    assert_eq!(err, Pcf8523Error::I2C(Other));
    i2c.done();
}

#[test]
fn write_reg_ok() {
    let expectations = [
        i2c_reg_write(PCF8523_CONTROL_1, 0b0101_0010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c);
    driver.write_reg(PCF8523_CONTROL_1, 0b0101_0010).unwrap();
    i2c.done();
}

fn i2c_reg_read(reg: u8, payload: u8) -> I2cTransaction {
    I2cTransaction::write_read(PCF8523_I2C_ADDRESS, [reg].to_vec(), [payload].to_vec())
}

fn i2c_reg_write(reg: u8, payload: u8) -> I2cTransaction {
    I2cTransaction::write(PCF8523_I2C_ADDRESS, [reg, payload].to_vec())
}