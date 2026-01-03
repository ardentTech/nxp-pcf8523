use embedded_hal_async::i2c::ErrorKind::{NoAcknowledge, Other};
use embedded_hal_async::i2c::NoAcknowledgeSource::Address;
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
use nxp_pcf8523::datetime::Pcf8523DateTime;
use nxp_pcf8523::driver::Pcf8523Error::{Internal, InvalidArgument, InvalidState};
use nxp_pcf8523::driver::{PCF8523_I2C_ADDRESS, Pcf8523, Pcf8523Error};
use nxp_pcf8523::registers::*;
use nxp_pcf8523::typedefs::CorrectionMode::{Fast, Slow};
use nxp_pcf8523::typedefs::LowPulseWidth::Width93_750ms;
use nxp_pcf8523::typedefs::Pcf8523T;
use nxp_pcf8523::typedefs::TimerInterruptMode::{PermanentlyActive, Pulsed};
use nxp_pcf8523::typedefs::TimerMode::{Countdown, Watchdog};
use nxp_pcf8523::typedefs::TimerSourceClock::{Frequency1_60Hz, Frequency1Hz};
use nxp_pcf8523::typedefs::{
    ClkOut, Pcf8523U, PowerManagement, TimerA, TimerB, TimerBInterruptMode,
};

#[tokio::test]
async fn calibrate_offset_below_floor_err() {
    let expectations = [I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec())];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let err = driver.set_offset(Fast, -65).await.unwrap_err();
    assert_eq!(err, InvalidArgument);
    i2c.done();
}

#[tokio::test]
async fn calibrate_offset_above_ceiling_err() {
    let expectations = [I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec())];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let err = driver.set_offset(Fast, 64).await.unwrap_err();
    assert_eq!(err, InvalidArgument);
    i2c.done();
}

#[tokio::test]
async fn calibrate_offset_fast_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_write(PCF8523_OFFSET, 0b1000_1111),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.set_offset(Fast, 15).await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn calibrate_offset_slow_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_write(PCF8523_OFFSET, 0b0111_0001),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.set_offset(Slow, -15).await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn clear_alarm_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_2, 0b0010_1011),
        i2c_reg_write(PCF8523_CONTROL_2, 0b0111_0011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.clear_alarm_interrupt().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn clear_battery_switch_over_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_3, 0b0011_1000),
        i2c_reg_write(PCF8523_CONTROL_3, 0b0011_0000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.clear_battery_switch_over_interrupt().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn clear_second_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_2, 0b0111_0011),
        i2c_reg_write(PCF8523_CONTROL_2, 0b0110_1011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.clear_second_interrupt().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn clear_timer_a_interrupt_countdown_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_2, 0b0111_0011),
        i2c_reg_write(PCF8523_CONTROL_2, 0b0011_1011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let timer = TimerA {
        countdown: 0,
        interrupt_mode: Pulsed,
        mode: Countdown,
        source_clock: Frequency1_60Hz,
    };
    driver.clear_timer_a_interrupt(&timer).await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn clear_timer_a_interrupt_watchdog_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_2, 0b0111_0011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let timer = TimerA {
        countdown: 0,
        interrupt_mode: Pulsed,
        mode: Watchdog,
        source_clock: Frequency1_60Hz,
    };
    driver.clear_timer_a_interrupt(&timer).await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn clear_timer_b_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_2, 0b0111_0011),
        i2c_reg_write(PCF8523_CONTROL_2, 0b0101_1011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523U {}).await.unwrap();
    driver.clear_timer_b_interrupt().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn disable_alarm_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0010_0010),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0010_0000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.disable_alarm_interrupt().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn disable_battery_low_detection_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_3, 0b0010_0001),
        i2c_reg_write(PCF8523_CONTROL_3, 0b0010_0000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver
        .disable_battery_low_detection_interrupt()
        .await
        .unwrap();
    i2c.done();
}

#[tokio::test]
async fn disable_battery_switch_over_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_3, 0b0011_0010),
        i2c_reg_write(PCF8523_CONTROL_3, 0b0011_0000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver
        .disable_battery_switch_over_interrupt()
        .await
        .unwrap();
    i2c.done();
}

#[tokio::test]
async fn disable_correction_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0010_0001),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0010_0000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.disable_correction_interrupt().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn disable_day_alarm_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_DAY_ALARM, 0b0001_0101),
        i2c_reg_write(PCF8523_DAY_ALARM, 0b1001_0101),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.disable_day_alarm().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn disable_hour_alarm_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_HOUR_ALARM, 0b0001_0101),
        i2c_reg_write(PCF8523_HOUR_ALARM, 0b1001_0101),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.disable_hour_alarm().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn disable_minute_alarm_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_MINUTE_ALARM, 0b0101_1001),
        i2c_reg_write(PCF8523_MINUTE_ALARM, 0b1101_1001),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.disable_minute_alarm().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn disable_weekday_alarm_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_WEEKDAY_ALARM, 0b0001_0101),
        i2c_reg_write(PCF8523_WEEKDAY_ALARM, 0b1001_0101),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.disable_weekday_alarm().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn enable_battery_low_detection_interrupt_internal_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_3, 0b0111_0000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let err = driver
        .enable_battery_low_detection_interrupt()
        .await
        .unwrap_err();
    assert_eq!(err, Internal);
    i2c.done();
}

#[tokio::test]
async fn enable_battery_low_detection_itnerrupt_invalid_state_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_3, 0b1110_0000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let err = driver
        .enable_battery_low_detection_interrupt()
        .await
        .unwrap_err();
    assert_eq!(err, InvalidState);
    i2c.done();
}

#[tokio::test]
async fn enable_battery_low_detection_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_3, 0b0010_0000),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b11_1000),
        i2c_reg_write(PCF8523_CONTROL_3, 0b0010_0001),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver
        .enable_battery_low_detection_interrupt()
        .await
        .unwrap();
    i2c.done();
}

#[tokio::test]
async fn enable_battery_switch_over_interrupt_internal_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_3, 0b1100_0000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let err = driver
        .enable_battery_switch_over_interrupt()
        .await
        .unwrap_err();
    assert_eq!(err, Internal);
    i2c.done();
}

#[tokio::test]
async fn enable_battery_switch_over_interrupt_invalid_state_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_3, 0b0100_0000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let err = driver
        .enable_battery_switch_over_interrupt()
        .await
        .unwrap_err();
    assert_eq!(err, InvalidState);
    i2c.done();
}

#[tokio::test]
async fn enable_battery_switch_over_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_3, 0b1000_0000),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b11_1000),
        i2c_reg_write(PCF8523_CONTROL_3, 0b1000_0010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.enable_battery_switch_over_interrupt().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn enable_correction_interrupt_invalid_state_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_OFFSET, 0b0),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let err = driver.enable_correction_interrupt().await.unwrap_err();
    assert_eq!(err, InvalidState);
    i2c.done();
}

#[tokio::test]
async fn enable_correction_interrupt_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_OFFSET, 0b1),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0010_0100),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b11_1000),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0010_0101),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.enable_correction_interrupt().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn enable_day_alarm_invalid_arg_err() {
    let expectations = [I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec())];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let err = driver.enable_day_alarm(32).await.unwrap_err();
    assert_eq!(err, InvalidArgument);
    i2c.done();
}

#[tokio::test]
async fn enable_weekday_alarm_invalid_arg_err() {
    let expectations = [I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec())];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let err = driver.enable_weekday_alarm(7).await.unwrap_err();
    assert_eq!(err, InvalidArgument);
    i2c.done();
}

#[tokio::test]
async fn enable_day_alarm_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b11_1000),
        i2c_reg_write(PCF8523_DAY_ALARM, 0b0001_0011),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0010_0100),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0010_0110),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.enable_day_alarm(13).await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn enable_weekday_alarm_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b11_1000),
        i2c_reg_write(PCF8523_WEEKDAY_ALARM, 0b0000_0100),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0010_0100),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0010_0110),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.enable_weekday_alarm(4).await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn enable_hour_alarm_invalid_arg_err() {
    let expectations = [I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec())];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let err = driver.enable_hour_alarm(24).await.unwrap_err();
    assert_eq!(err, InvalidArgument);
    i2c.done();
}

#[tokio::test]
async fn enable_hour_alarm_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b11_1000),
        i2c_reg_write(PCF8523_HOUR_ALARM, 0b0001_0111),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0010_0100),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0010_0110),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.enable_hour_alarm(17).await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn enable_minute_alarm_invalid_arg_err() {
    let expectations = [I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec())];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let err = driver.enable_minute_alarm(60).await.unwrap_err();
    assert_eq!(err, InvalidArgument);
    i2c.done();
}

#[tokio::test]
async fn enable_minute_alarm_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b11_1000),
        i2c_reg_write(PCF8523_MINUTE_ALARM, 0b0101_1001),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0010_0100),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0010_0110),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.enable_minute_alarm(59).await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn get_datetime_ok() {
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
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let dt = driver.now().await.unwrap();
    assert_eq!(dt.second, 15);
    assert_eq!(dt.minute, 5);
    assert_eq!(dt.hour, 0);
    assert_eq!(dt.day, 0);
    assert_eq!(dt.month, 1);
    assert_eq!(dt.year, 0);
    i2c.done();
}

#[tokio::test]
async fn lost_power_false() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_SECONDS, 0b0111_1111),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    assert!(!driver.lost_power().await.unwrap());
    i2c.done();
}

#[tokio::test]
async fn lost_power_true() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_SECONDS, 0b1111_1111),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    assert!(driver.lost_power().await.unwrap());
    i2c.done();
}

#[tokio::test]
async fn new_err() {
    let expectations = [I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec())
        .with_error(NoAcknowledge(Address))];
    let mut i2c = I2cMock::new(&expectations);
    match Pcf8523::new(&mut i2c, Pcf8523T {}).await {
        Err(err) => assert_eq!(err, Pcf8523Error::I2C(NoAcknowledge(Address))),
        _ => panic!(),
    }
    i2c.done();
}

#[tokio::test]
async fn read_reg_i2c_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0101_0110).with_error(Other),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let err = driver.read_reg(PCF8523_CONTROL_1).await.unwrap_err();
    assert_eq!(err, Pcf8523Error::I2C(Other));
    i2c.done();
}

#[tokio::test]
async fn read_reg_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0101_0110),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let val = driver.read_reg(PCF8523_CONTROL_1).await.unwrap();
    assert_eq!(val, 0b0101_0110);
    i2c.done();
}

#[tokio::test]
async fn reload_timer_a_watchdog_countdown_arg_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0001_1100),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let timer = TimerA {
        countdown: 0,
        interrupt_mode: Pulsed,
        mode: Countdown,
        source_clock: Frequency1_60Hz,
    };
    driver
        .reload_timer_a_watchdog_countdown(&timer)
        .await
        .unwrap_err();
    i2c.done();
}

#[tokio::test]
async fn reload_timer_a_watchdog_countdown_mode_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0001_1110),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let timer = TimerA {
        countdown: 0,
        interrupt_mode: Pulsed,
        mode: Watchdog,
        source_clock: Frequency1_60Hz,
    };
    driver
        .reload_timer_a_watchdog_countdown(&timer)
        .await
        .unwrap_err();
    i2c.done();
}

#[tokio::test]
async fn reload_timer_a_watchdog_countdown_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0001_1100),
        i2c_reg_write(PCF8523_TMR_A_REG, 0b0100_0001),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let timer = TimerA {
        countdown: 65,
        interrupt_mode: Pulsed,
        mode: Watchdog,
        source_clock: Frequency1_60Hz,
    };
    driver
        .reload_timer_a_watchdog_countdown(&timer)
        .await
        .unwrap();
    i2c.done();
}

#[tokio::test]
async fn reset_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0101_1000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.reset().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn running_false() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0110_1001),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    assert!(!driver.running().await.unwrap());
    i2c.done();
}

#[tokio::test]
async fn running_true() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0100_1001),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    assert!(driver.running().await.unwrap());
    i2c.done();
}

#[tokio::test]
async fn set_clkout_max_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b1100_0000),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b1100_0000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.set_clkout(ClkOut::Frequency32_768Hz).await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn set_clkout_min_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b1100_0000),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b1111_1000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.set_clkout(ClkOut::Frequency0Hz).await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn set_datetime_ok() {
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
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let dt = Pcf8523DateTime {
        second: 59,
        minute: 15,
        hour: 2,
        day: 11,
        month: 10,
        year: 45,
    };
    driver.set_datetime(dt).await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn set_power_management_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_3, 0b0001_1010),
        i2c_reg_write(PCF8523_CONTROL_3, 0b1001_1010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver
        .set_power_management(PowerManagement::SwitchOverStandardOnLowDetectionOff)
        .await
        .unwrap();
    i2c.done();
}

#[tokio::test]
async fn start_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0111_1010),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0101_1010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.start().await.unwrap();
    i2c.done();
}
#[tokio::test]
async fn start_second_timer_pulsed_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0010_0011),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b1011_1011),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0010_0011),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0010_0111),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.start_second_timer(Pulsed).await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn start_second_timer_permanent_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b1010_0011),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b0011_1011),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0010_0011),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0010_0111),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.start_second_timer(PermanentlyActive).await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn start_timer_a_countdown_invalid_timer_countdown_err() {
    let expectations = [I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec())];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let timer = TimerA {
        countdown: 0,
        interrupt_mode: Pulsed,
        mode: Countdown,
        source_clock: Frequency1_60Hz,
    };
    driver.start_timer_a(&timer).await.unwrap_err();
    i2c.done();
}

#[tokio::test]
async fn start_timer_a_countdown_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b110),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b0),
        i2c_reg_write(PCF8523_TMR_A_FREQ_CTRL, 0b11),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b1011_1010),
        i2c_reg_read(PCF8523_CONTROL_2, 0b0),
        i2c_reg_write(PCF8523_CONTROL_2, 0b10),
        i2c_reg_write(PCF8523_TMR_A_REG, 0b1101),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let timer = TimerA {
        countdown: 13,
        interrupt_mode: Pulsed,
        mode: Countdown,
        source_clock: Frequency1_60Hz,
    };
    driver.start_timer_a(&timer).await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn start_timer_a_watchdog_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b110),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b0),
        i2c_reg_write(PCF8523_TMR_A_FREQ_CTRL, 0b10),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b0011_1100),
        i2c_reg_read(PCF8523_CONTROL_2, 0b0),
        i2c_reg_write(PCF8523_CONTROL_2, 0b100),
        i2c_reg_write(PCF8523_TMR_A_REG, 0b1110),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let timer = TimerA {
        countdown: 14,
        interrupt_mode: PermanentlyActive,
        mode: Watchdog,
        source_clock: Frequency1Hz,
    };
    driver.start_timer_a(&timer).await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn start_timer_b_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        // stop timer
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b1),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b0),
        // set frequency, low pulse width
        i2c_reg_read(PCF8523_TMR_B_FREQ_CTRL, 0b0),
        i2c_reg_write(PCF8523_TMR_B_FREQ_CTRL, 0b11_0010),
        // enable interrupt
        i2c_reg_read(PCF8523_CONTROL_2, 0b0),
        i2c_reg_write(PCF8523_CONTROL_2, 0b1),
        // disable clkout, set interrupt mode and enable timer
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b111_1001),
        // set countdown val
        i2c_reg_write(PCF8523_TMR_B_REG, 0b111),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523U {}).await.unwrap();
    let timer_b = TimerB {
        countdown: 7,
        interrupt_mode: TimerBInterruptMode::Pulsed(Width93_750ms),
        source_clock: Frequency1Hz,
    };
    driver.start_timer_b(&timer_b).await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn stop_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b0101_1010),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0111_1010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.stop().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn stop_clkout_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b11_1000),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.stop_clkout().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn stop_second_timer_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_CONTROL_1, 0b100),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.stop_second_timer().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn stop_timer_a_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b110),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b0),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.stop_timer_a().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn stop_timer_b_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_CLKOUT_CTRL, 0b0101_1011),
        i2c_reg_write(PCF8523_TMR_CLKOUT_CTRL, 0b0101_1010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523U {}).await.unwrap();
    driver.stop_timer_b().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn timer_a_counter_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_A_REG, 0b011),
        i2c_reg_read(PCF8523_TMR_A_REG, 0b010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.timer_a_counter().await.unwrap_err();
    i2c.done();
}

#[tokio::test]
async fn timer_a_counter_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_A_REG, 0b011),
        i2c_reg_read(PCF8523_TMR_A_REG, 0b011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver.timer_a_counter().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn timer_b_counter_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_B_REG, 0b011),
        i2c_reg_read(PCF8523_TMR_B_REG, 0b010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523U {}).await.unwrap();
    driver.timer_b_counter().await.unwrap_err();
    i2c.done();
}

#[tokio::test]
async fn timer_b_counter_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_read(PCF8523_TMR_B_REG, 0b011),
        i2c_reg_read(PCF8523_TMR_B_REG, 0b011),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523U {}).await.unwrap();
    driver.timer_b_counter().await.unwrap();
    i2c.done();
}

#[tokio::test]
async fn write_reg_i2c_err() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0101_0110).with_error(Other),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    let err = driver
        .write_reg(PCF8523_CONTROL_1, 0b0101_0110)
        .await
        .unwrap_err();
    assert_eq!(err, Pcf8523Error::I2C(Other));
    i2c.done();
}

#[tokio::test]
async fn write_reg_ok() {
    let expectations = [
        I2cTransaction::read(PCF8523_I2C_ADDRESS, [0b0].to_vec()),
        i2c_reg_write(PCF8523_CONTROL_1, 0b0101_0010),
    ];
    let mut i2c = I2cMock::new(&expectations);
    let mut driver = Pcf8523::new(&mut i2c, Pcf8523T {}).await.unwrap();
    driver
        .write_reg(PCF8523_CONTROL_1, 0b0101_0010)
        .await
        .unwrap();
    i2c.done();
}

fn i2c_reg_read(reg: u8, payload: u8) -> I2cTransaction {
    I2cTransaction::write_read(PCF8523_I2C_ADDRESS, [reg].to_vec(), [payload].to_vec())
}

fn i2c_reg_write(reg: u8, payload: u8) -> I2cTransaction {
    I2cTransaction::write(PCF8523_I2C_ADDRESS, [reg, payload].to_vec())
}
