use embedded_hal::i2c::ErrorKind::{NoAcknowledge, Other};
use embedded_hal::i2c::NoAcknowledgeSource::Address;
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
use nxp_pcf8523::datetime::Pcf8523DateTime;
use nxp_pcf8523::driver::{Pcf8523, Pcf8523Error, PCF8523_I2C_ADDRESS};
use nxp_pcf8523::registers::*;

#[test]
fn get_datetime_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        I2cTransaction::transaction_start(PCF8523_I2C_ADDRESS),
        // all payloads are bcd-encoded
        I2cTransaction::write(PCF8523_I2C_ADDRESS, [PCF8523_SECONDS].to_vec()),
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b1_0101].to_vec()),

        I2cTransaction::write(PCF8523_I2C_ADDRESS, [PCF8523_MINUTES].to_vec()),
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b101].to_vec()),

        I2cTransaction::write(PCF8523_I2C_ADDRESS, [PCF8523_HOURS].to_vec()),
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),

        I2cTransaction::write(PCF8523_I2C_ADDRESS, [PCF8523_DAYS].to_vec()),
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),

        I2cTransaction::write(PCF8523_I2C_ADDRESS, [PCF8523_MONTHS].to_vec()),
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b1].to_vec()),

        I2cTransaction::write(PCF8523_I2C_ADDRESS, [PCF8523_YEARS].to_vec()),
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),

        I2cTransaction::transaction_end(PCF8523_I2C_ADDRESS),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    let dt = driver.now().unwrap();
    assert_eq!(dt.seconds, 15);
    // TODO other assertions
    i2c.done();
}

#[test]
fn initialized_false() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_3, 0b1110_0000)
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    assert!(!driver.initialized().unwrap());
    i2c.done();
}

#[test]
fn initialized_true() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_3, 0b0)
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    assert!(driver.initialized().unwrap());
    i2c.done();
}

#[test]
fn lost_power_false() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_SECONDS, 0b0111_1111)
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    assert!(!driver.lost_power().unwrap());
    i2c.done();
}

#[test]
fn lost_power_true() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_SECONDS, 0b1111_1111)
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    assert!(driver.lost_power().unwrap());
    i2c.done();
}

#[test]
fn new_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()).with_error(NoAcknowledge(Address)),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let err = Pcf8523::new(&mut i2c).unwrap_err();
    assert_eq!(err, Pcf8523Error::I2C(NoAcknowledge(Address)));
    i2c.done();
}

#[test]
fn read_reg_i2c_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0101_0110).with_error(Other),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    let err = driver.read_reg(PCF8523_CONTROL_1).unwrap_err();
    assert_eq!(err, Pcf8523Error::I2C(Other));
    i2c.done();
}

#[test]
fn read_reg_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0101_0110),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    let val = driver.read_reg(PCF8523_CONTROL_1).unwrap();
    assert_eq!(val, 0b0101_0110);
    i2c.done();
}

#[test]
fn reset_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0101_1000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.reset().unwrap();
    i2c.done();
}

#[test]
fn set_datetime_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        I2cTransaction::transaction_start(PCF8523_I2C_ADDRESS),
        i2c_reg_write(PCF8523_SECONDS, 0b0101_1001),
        i2c_reg_write(PCF8523_MINUTES, 0b1_0101),
        i2c_reg_write(PCF8523_HOURS, 0b10),
        i2c_reg_write(PCF8523_DAYS, 0b1_0001),
        i2c_reg_write(PCF8523_MONTHS, 0b1_0000),
        i2c_reg_write(PCF8523_YEARS, 0b100_0101),
        I2cTransaction::transaction_end(PCF8523_I2C_ADDRESS),
        i2c_reg_write(PCF8523_CONTROL_3, 0b0),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    let dt = Pcf8523DateTime {
        seconds: 59,
        minutes: 15,
        hours: 2,
        day: 11,
        month: 10,
        year: 45,
    };
    driver.set_datetime(dt).unwrap();
    i2c.done();
}

#[test]
fn start_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0101_1010),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0111_1010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.start().unwrap();
    i2c.done();
}

#[test]
fn stop_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0111_1010),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0101_1010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.stop().unwrap();
    i2c.done();
}

#[test]
fn write_reg_i2c_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0101_0110).with_error(Other),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    let err = driver.write_reg(PCF8523_CONTROL_1, 0b0101_0110).unwrap_err();
    assert_eq!(err, Pcf8523Error::I2C(Other));
    i2c.done();
}

#[test]
fn write_reg_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0101_0010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.write_reg(PCF8523_CONTROL_1, 0b0101_0010).unwrap();
    i2c.done();
}

fn i2c_reg_read(reg: u8, payload: u8) -> I2cTransaction {
    I2cTransaction::write_read(PCF8523_I2C_ADDRESS, [reg].to_vec(), [payload].to_vec())
}

fn i2c_reg_write(reg: u8, payload: u8) -> I2cTransaction {
    I2cTransaction::write(PCF8523_I2C_ADDRESS, [reg, payload].to_vec())
}