use embedded_hal::i2c::ErrorKind::Other;
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
use nxp_pcf8523::driver::{Pcf8523, PCF8523_I2C_ADDRESS};
use nxp_pcf8523::typedefs::{Pcf8523Error, Pcf8523Interrupt};
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
fn get_battery_status_low() {
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
fn get_battery_status_ok() {
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