use embedded_hal::i2c::ErrorKind::{NoAcknowledge, Other};
use embedded_hal::i2c::NoAcknowledgeSource::Address;
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
use nxp_pcf8523::datetime::Pcf8523DateTime;
use nxp_pcf8523::driver::{Pcf8523, Pcf8523Error, PCF8523_I2C_ADDRESS};
use nxp_pcf8523::driver::Pcf8523Error::InvalidArgument;
use nxp_pcf8523::registers::*;
use nxp_pcf8523::typedefs::CorrectionMode::{Fast, Slow};
use nxp_pcf8523::typedefs::{PowerManagement, TimerB, TimerA, TimerBInterruptMode};
use nxp_pcf8523::typedefs::TimerSourceClock::{Frequency1Hz, Frequency1_60Hz};
use nxp_pcf8523::typedefs::LowPulseWidth::Width93_750ms;
use nxp_pcf8523::typedefs::TimerAInterruptMode::{PermanentlyActive, Pulsed};
use nxp_pcf8523::typedefs::TimerMode::{Countdown, Watchdog};

#[test]
fn calibrate_offset_below_floor_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec())
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    let err = driver.calibrate(Fast, -65).unwrap_err();
    assert_eq!(err, InvalidArgument);
    i2c.done();
}

#[test]
fn calibrate_offset_above_ceiling_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec())
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    let err = driver.calibrate(Fast, 64).unwrap_err();
    assert_eq!(err, InvalidArgument);
    i2c.done();
}

#[test]
fn calibrate_offset_fast_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_write(PCF8523_OFFSET, 0b1000_1111),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.calibrate(Fast, 15).unwrap();
    i2c.done();
}

#[test]
fn calibrate_offset_slow_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_write(PCF8523_OFFSET, 0b0111_0001),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.calibrate(Slow, -15).unwrap();
    i2c.done();
}

#[test]
fn clear_alarm_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_2, 0b0010_1011),
        i2c_reg_write(PCF8523_CONTROL_2, 0b0111_0011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.clear_alarm_interrupt().unwrap();
    i2c.done();
}

#[test]
fn clear_second_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_2, 0b0111_0011),
        i2c_reg_write(PCF8523_CONTROL_2, 0b0110_1011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.clear_second_interrupt().unwrap();
    i2c.done();
}

// #[test]
// fn clear_timer_a_countdown_interrupt_ok() {
//     let expectations = [
//         I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
//         i2c_reg_read(PCF8523_CONTROL_2, 0b0111_0011),
//         i2c_reg_write(PCF8523_CONTROL_2, 0b0011_1011),
//     ];
//     let mut i2c = I2cMock::new(&expectations);
//     let mut driver = Pcf8523::new(&mut i2c).unwrap();
//     driver.clear_timer_a_countdown_interrupt().unwrap();
//     i2c.done();
// }

// #[test]
// fn clear_timer_a_watchdog_interrupt_ok() {
//     let expectations = [
//         I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
//         i2c_reg_read(PCF8523_CONTROL_2, 0b0111_0011),
//     ];
//     let mut i2c = I2cMock::new(&expectations);
//     let mut driver = Pcf8523::new(&mut i2c).unwrap();
//     driver.clear_timer_a_watchdog_interrupt().unwrap();
//     i2c.done();
// }

#[test]
fn clear_timer_b_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_2, 0b0111_0011),
        i2c_reg_write(PCF8523_CONTROL_2, 0b0101_1011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.clear_timer_b_interrupt().unwrap();
    i2c.done();
}

#[test]
fn disable_alarm_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0010_0010),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0010_0000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.disable_alarm_interrupt().unwrap();
    i2c.done();
}

#[test]
fn disable_correction_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0010_0001),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0010_0000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.disable_correction_interrupt().unwrap();
    i2c.done();
}

#[test]
fn disable_day_alarm_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_DAY_ALARM, 0b0001_0101),
        i2c_reg_write(PCF8523_DAY_ALARM, 0b1001_0101),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.disable_day_alarm().unwrap();
    i2c.done();
}

#[test]
fn disable_hour_alarm_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_HOUR_ALARM, 0b0001_0101),
        i2c_reg_write(PCF8523_HOUR_ALARM, 0b1001_0101),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.disable_hour_alarm().unwrap();
    i2c.done();
}

#[test]
fn disable_minute_alarm_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_MINUTE_ALARM, 0b0101_1001),
        i2c_reg_write(PCF8523_MINUTE_ALARM, 0b1101_1001),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.disable_minute_alarm().unwrap();
    i2c.done();
}

#[test]
fn disable_second_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0010_0100),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0010_0000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.disable_second_interrupt().unwrap();
    i2c.done();
}

#[test]
fn disable_timer_a_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_2, 0b0010_0110),
        i2c_reg_write(PCF8523_CONTROL_2, 0b0010_0000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.disable_timer_a_interrupt().unwrap();
    i2c.done();
}

#[test]
fn disable_timer_b_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_2, 0b0010_0001),
        i2c_reg_write(PCF8523_CONTROL_2, 0b0010_0000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.disable_timer_b_interrupt().unwrap();
    i2c.done();
}

#[test]
fn disable_weekday_alarm_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_WEEKDAY_ALARM, 0b0001_0101),
        i2c_reg_write(PCF8523_WEEKDAY_ALARM, 0b1001_0101),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.disable_weekday_alarm().unwrap();
    i2c.done();
}

#[test]
fn enable_alarm_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0010_0100),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0010_0110),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.enable_alarm_interrupt().unwrap();
    i2c.done();
}

#[test]
fn enable_correction_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0010_0100),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0010_0101),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.enable_correction_interrupt().unwrap();
    i2c.done();
}

#[test]
fn enable_day_alarm_invalid_arg_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec())
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    let err = driver.enable_day_alarm(32).unwrap_err();
    assert_eq!(err, InvalidArgument);
    i2c.done();
}

#[test]
fn enable_weekday_alarm_invalid_arg_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec())
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    let err = driver.enable_weekday_alarm(7).unwrap_err();
    assert_eq!(err, InvalidArgument);
    i2c.done();
}

#[test]
fn enable_day_alarm_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_write(PCF8523_DAY_ALARM, 0b0001_0011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.enable_day_alarm(13).unwrap();
    i2c.done();
}

#[test]
fn enable_second_interrupt_pulsed_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0010_0011),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0010_0111),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0010_0011),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b1010_0011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.enable_second_interrupt(true).unwrap();
    i2c.done();
}

#[test]
fn enable_second_interrupt_permanent_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0010_0011),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0010_0111),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b1010_0011),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b0010_0011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.enable_second_interrupt(false).unwrap();
    i2c.done();
}

// #[test]
// fn enable_timer_a_interrupt_countdown_ok() {
//     let expectations = [
//         I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
//         i2c_reg_read(PCF8523_CONTROL_2, 0b0001_1000),
//         i2c_reg_write(PCF8523_CONTROL_2, 0b0001_1010),
//     ];
//     let mut i2c = I2cMock::new(&expectations);
//     let mut driver = Pcf8523::new(&mut i2c).unwrap();
//     driver.enable_timer_a_interrupt(TimerMode::Countdown).unwrap();
//     i2c.done();
// }

// #[test]
// fn enable_timer_a_interrupt_watchdog_ok() {
//     let expectations = [
//         I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
//         i2c_reg_read(PCF8523_CONTROL_2, 0b0001_1000),
//         i2c_reg_write(PCF8523_TMR_A_FREQ_CTRL, 0b0000_0001),
//         i2c_reg_write(PCF8523_CONTROL_2, 0b0001_1100),
//     ];
//     let mut i2c = I2cMock::new(&expectations);
//     let mut driver = Pcf8523::new(&mut i2c).unwrap();
//     driver.enable_timer_a_interrupt(TimerMode::Watchdog(TimerSourceClock::Frequency64Hz)).unwrap();
//     i2c.done();
// }

#[test]
fn enable_timer_b_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_2, 0b0001_1010),
        i2c_reg_write(PCF8523_CONTROL_2, 0b0001_1011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.enable_timer_b_interrupt().unwrap();
    i2c.done();
}

#[test]
fn enable_weekday_alarm_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_write(PCF8523_WEEKDAY_ALARM, 0b0000_0100),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.enable_weekday_alarm(4).unwrap();
    i2c.done();
}

#[test]
fn enable_hour_alarm_invalid_arg_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec())
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    let err = driver.enable_hour_alarm(24).unwrap_err();
    assert_eq!(err, InvalidArgument);
    i2c.done();
}

#[test]
fn enable_hour_alarm_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_write(PCF8523_HOUR_ALARM, 0b0001_0111),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.enable_hour_alarm(17).unwrap();
    i2c.done();
}

#[test]
fn enable_minute_alarm_invalid_arg_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec())
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    let err = driver.enable_minute_alarm(60).unwrap_err();
    assert_eq!(err, InvalidArgument);
    i2c.done();
}

#[test]
fn enable_minute_alarm_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_write(PCF8523_MINUTE_ALARM, 0b0101_1001),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.enable_minute_alarm(59).unwrap();
    i2c.done();
}

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
    match Pcf8523::new(&mut i2c) {
        Err(err) => assert_eq!(err, Pcf8523Error::I2C(NoAcknowledge(Address))),
        _ => panic!(),
    }
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
fn running_false() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0110_1001)
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    assert!(!driver.running().unwrap());
    i2c.done();
}

#[test]
fn running_true() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0100_1001)
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    assert!(driver.running().unwrap());
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
fn set_power_management_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_3, 0b0001_1010),
        i2c_reg_write(PCF8523_CONTROL_3, 0b1001_1010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.set_power_management(PowerManagement::SwitchOverStandardOnLowDetectionOff).unwrap();
    i2c.done();
}

// #[test]
// fn set_timer_a_mode_countdown_ok() {
//     let expectations = [
//         I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
//         i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0001_1100),
//         i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b0001_1010),
//     ];
//     let mut i2c = I2cMock::new(&expectations);
//     let mut driver = Pcf8523::new(&mut i2c).unwrap();
//     driver.set_timer_a_mode(TimerMode::Countdown).unwrap();
//     i2c.done();
// }

// #[test]
// fn set_timer_a_mode_watchdog_ok() {
//     let expectations = [
//         I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
//         i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0001_1000),
//         i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b0001_1100),
//     ];
//     let mut i2c = I2cMock::new(&expectations);
//     let mut driver = Pcf8523::new(&mut i2c).unwrap();
//     driver.set_timer_a_mode(TimerMode::Watchdog(TimerSourceClock::Frequency1_3600Hz)).unwrap();
//     i2c.done();
// }

// #[test]
// fn set_timer_a_interrupt_mode_permanently_active_ok() {
//     let expectations = [
//         I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
//         i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b1001_1100),
//         i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b0001_1100),
//     ];
//     let mut i2c = I2cMock::new(&expectations);
//     let mut driver = Pcf8523::new(&mut i2c).unwrap();
//     driver.set_timer_a_interrupt_mode(PermanentlyActive).unwrap();
//     i2c.done();
// }

// #[test]
// fn set_timer_a_interrupt_mode_pulsed_ok() {
//     let expectations = [
//         I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
//         i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0001_1100),
//         i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b1001_1100),
//     ];
//     let mut i2c = I2cMock::new(&expectations);
//     let mut driver = Pcf8523::new(&mut i2c).unwrap();
//     driver.set_timer_a_interrupt_mode(Pulsed(Width218_750ms)).unwrap();
//     i2c.done();
// }

// #[test]
// fn set_timer_b_interrupt_mode_permanently_active_ok() {
//     let expectations = [
//         I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
//         i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b010_0010),
//         I2cTransaction::transaction_start(PCF8523_I2C_ADDRESS),
//         I2cTransaction::write(PCF8523_I2C_ADDRESS, [PCF8523_TMR_B_FREQ_CTRL, 0b100].to_vec()),
//         I2cTransaction::write(PCF8523_I2C_ADDRESS, [PCF8523_TMR_CLKOUT_CTRL, 0b010_0010].to_vec()),
//         I2cTransaction::transaction_end(PCF8523_I2C_ADDRESS),
//     ];
//     let mut i2c = I2cMock::new(&expectations);
//     let mut driver = Pcf8523::new(&mut i2c).unwrap();
//     driver.set_timer_b_interrupt_mode(PermanentlyActive).unwrap();
//     i2c.done();
// }

// #[test]
// fn set_timer_b_interrupt_mode_pulsed_ok() {
//     let expectations = [
//         I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
//         i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b010_0010),
//         I2cTransaction::transaction_start(PCF8523_I2C_ADDRESS),
//         I2cTransaction::write(PCF8523_I2C_ADDRESS, [PCF8523_TMR_B_FREQ_CTRL, 0b11_0000].to_vec()),
//         I2cTransaction::write(PCF8523_I2C_ADDRESS, [PCF8523_TMR_CLKOUT_CTRL, 0b110_0010].to_vec()),
//         I2cTransaction::transaction_end(PCF8523_I2C_ADDRESS),
//     ];
//     let mut i2c = I2cMock::new(&expectations);
//     let mut driver = Pcf8523::new(&mut i2c).unwrap();
//     driver.set_timer_b_interrupt_mode(Pulsed(LowPulseWidth::Width93_750ms)).unwrap();
//     i2c.done();
// }

#[test]
fn start_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0111_1010),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0101_1010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.start().unwrap();
    i2c.done();
}

#[test]
fn start_timer_a_countdown_invalid_timer_countdown_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec())
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    let timer = TimerA {
        countdown: 0,
        interrupt_mode: Pulsed,
        mode: Countdown,
        source_clock: Frequency1_60Hz,
    };
    driver.start_timer_a(&timer).unwrap_err();
    i2c.done();
}

#[test]
fn start_timer_a_countdown_already_running_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b10),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b00),
        i2c_reg_write(PCF8523_TMR_A_FREQ_CTRL, 0b11),
        i2c_reg_read(PCF8523_CONTROL_2, 0b0),
        i2c_reg_write(PCF8523_CONTROL_2, 0b10),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b1000_0010),
        i2c_reg_write(PCF8523_TMR_A_REG, 0b1101),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    let timer = TimerA {
        countdown: 13,
        interrupt_mode: Pulsed,
        mode: Countdown,
        source_clock: Frequency1_60Hz,
    };
    driver.start_timer_a(&timer).unwrap();
    i2c.done();
}

#[test]
fn start_timer_a_countdown_not_running_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0),
        i2c_reg_write(PCF8523_TMR_A_FREQ_CTRL, 0b11),
        i2c_reg_read(PCF8523_CONTROL_2, 0b0),
        i2c_reg_write(PCF8523_CONTROL_2, 0b10),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b1000_0010),
        i2c_reg_write(PCF8523_TMR_A_REG, 0b1101),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    let timer = TimerA {
        countdown: 13,
        interrupt_mode: Pulsed,
        mode: Countdown,
        source_clock: Frequency1_60Hz,
    };
    driver.start_timer_a(&timer).unwrap();
    i2c.done();
}

#[test]
fn start_timer_a_watchdog_already_running_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b10),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b00),
        i2c_reg_write(PCF8523_TMR_A_FREQ_CTRL, 0b11),
        i2c_reg_read(PCF8523_CONTROL_2, 0b0),
        i2c_reg_write(PCF8523_CONTROL_2, 0b100),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b100),
        i2c_reg_write(PCF8523_TMR_A_REG, 0b1001),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    let timer = TimerA {
        countdown: 9,
        interrupt_mode: PermanentlyActive,
        mode: Watchdog,
        source_clock: Frequency1_60Hz,
    };
    driver.start_timer_a(&timer).unwrap();
    i2c.done();
}


#[test]
fn start_timer_a_watchdog_not_running_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0),
        i2c_reg_write(PCF8523_TMR_A_FREQ_CTRL, 0b10),
        i2c_reg_read(PCF8523_CONTROL_2, 0b0),
        i2c_reg_write(PCF8523_CONTROL_2, 0b100),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b100),
        i2c_reg_write(PCF8523_TMR_A_REG, 0b1110),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    let timer = TimerA {
        countdown: 14,
        interrupt_mode: PermanentlyActive,
        mode: Watchdog,
        source_clock: Frequency1Hz,
    };
    driver.start_timer_a(&timer).unwrap();
    i2c.done();
}

#[test]
fn start_timer_b_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        // disable timer
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b1),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b0),
        // set frequency
        i2c_reg_read(PCF8523_TMR_B_FREQ_CTRL, 0b0),
        i2c_reg_write(PCF8523_TMR_B_FREQ_CTRL, 0b11_0010),
        // enable interrupt
        i2c_reg_read(PCF8523_CONTROL_2, 0b0),
        i2c_reg_write(PCF8523_CONTROL_2, 0b1),
        // // set countdown val
        i2c_reg_write(PCF8523_TMR_B_REG, 0b111),
        // enable timer
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b100_0001)
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    let timer_b = TimerB {
        countdown: 7,
        interrupt_mode: TimerBInterruptMode::Pulsed(Width93_750ms),
        source_clock: Frequency1Hz,
    };
    driver.start_timer_b(timer_b).unwrap();
    i2c.done();
}

#[test]
fn stop_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0101_1010),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0111_1010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.stop().unwrap();
    i2c.done();
}

#[test]
fn stop_timer_a_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_write(PCF8523_TMR_A_REG, 0b0),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.stop_timer_a().unwrap();
    i2c.done();
}

#[test]
fn stop_timer_b_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0101_1011),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b0101_1010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.stop_timer_b().unwrap();
    i2c.done();
}

#[test]
fn timer_a_counter_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_A_REG, 0b011),
        i2c_reg_read(PCF8523_TMR_A_REG, 0b010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.timer_a_counter().unwrap_err();
    i2c.done();
}

#[test]
fn timer_a_counter_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_A_REG, 0b011),
        i2c_reg_read(PCF8523_TMR_A_REG, 0b011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.timer_a_counter().unwrap();
    i2c.done();
}

#[test]
fn timer_b_counter_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_B_REG, 0b011),
        i2c_reg_read(PCF8523_TMR_B_REG, 0b010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.timer_b_counter().unwrap_err();
    i2c.done();
}

#[test]
fn timer_b_counter_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_B_REG, 0b011),
        i2c_reg_read(PCF8523_TMR_B_REG, 0b011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c).unwrap();
    driver.timer_b_counter().unwrap();
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