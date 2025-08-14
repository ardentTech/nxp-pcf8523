#![no_std]

use embedded_hal::i2c::I2c;

const PCF8523_I2C_ADDRESS: u8 = 0x68;
pub const PCF8523_CONTROL_1: u8 = 0x00;
pub const PCF8523_CONTROL_2: u8 = 0x01;
pub const PCF8523_CONTROL_3: u8 = 0x02;
pub const PCF8523_SECONDS: u8 = 0x03;
pub const PCF8523_MINUTES: u8 = 0x04;
pub const PCF8523_HOURS: u8 = 0x05;
pub const PCF8523_DAYS: u8 = 0x06;
pub const PCF8523_WEEKDAYS: u8 = 0x07;
pub const PCF8523_MONTHS: u8 = 0x08;
pub const PCF8523_YEARS: u8 = 0x09;
pub const PCF8523_MINUTE_ALARM: u8 = 0x0a;
pub const PCF8523_HOUR_ALARM: u8 = 0x0b;
pub const PCF8523_DAY_ALARM: u8 = 0x0c;
pub const PCF8523_WEEKDAY_ALARM: u8 = 0x0d;
pub const PCF8523_OFFSET: u8 = 0x0e;
pub const PCF8523_TMR_CLKOUT_CTRL: u8 = 0x0f;
pub const PCF8523_TMR_A_FREQ_CTRL: u8 = 0x10;
pub const PCF8523_TMR_A_REG: u8 = 0x11;
pub const PCF8523_TMR_B_FREQ_CTRL: u8 = 0x12;
pub const PCF8523_TMR_B_REG: u8 = 0x13;

#[derive(Debug, PartialEq)]
pub enum Meridiem {
    AM = 0x0,
    PM = 0x1,
}
// impl TryFrom<u8> for Meridiem {
//     type Error = ();
//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         match value {
//             0 => Ok(Meridiem::AM),
//             1 => Ok(Meridiem::PM),
//             _ => Err(()),
//         }
//     }
// }

#[derive(Debug, PartialEq)]
pub struct Hours {
    hours: u8,
    meridiem: Option<Meridiem>
}
impl Hours {
    pub fn new(hours: u8, meridiem: Option<Meridiem>) -> Self {
        // if meridiem.is_some() {
        //     // TODO validate < 13
        // }
        Self { hours, meridiem }
    }
}

#[derive(Debug, PartialEq)]
pub enum Pcf8523Error<E> {
    I2C(E),
}

#[derive(Debug, PartialEq)]
pub enum Pcf8523Interrupt {
    Alarm,
    BatterySwitchOver,
    CountdownTimerA,
    CountdownTimerB,
    Second,
    WatchdogTimerA,
}

#[derive(Debug, PartialEq)]
pub enum PowerManagement {
    SwitchOverStandardLowDetectionEnabled = 0x0,
    SwitchOverDirectSwitchingLowDetectionEnabled = 0x1,
    SwitchOverDisabledLowDetectionEnabled = 0x2,
    SwitchOverStandardLowDetectionDisabled = 0x4,
    SwitchOverDirectSwitchingLowDetectionDisabled = 0x5,
    SwitchOverDisabledLowDetectionDisabled = 0x7,
}

pub struct Pcf8523<I2C> {
    i2c: I2C
}

// TODO CONTROL_1 bit 0
impl<I2C: I2c> Pcf8523<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn clear_interrupt(&mut self, interrupt: Pcf8523Interrupt) -> Result<(), Pcf8523Error<I2C::Error>> {
        match interrupt {
            Pcf8523Interrupt::Alarm => {
                self.clear_alarm_interrupt()
            }
            Pcf8523Interrupt::BatterySwitchOver => {
                self.clear_bsf_interrupt()
            }
            Pcf8523Interrupt::CountdownTimerA => {
                self.clear_countdown_timer_a_interrupt()
            }
            Pcf8523Interrupt::CountdownTimerB => {
                self.clear_countdown_timer_b_interrupt()
            }
            Pcf8523Interrupt::Second => {
                self.clear_second_interrupt()
            }
            Pcf8523Interrupt::WatchdogTimerA => {
                self.clear_watchdog_timer_a_interrupt()
            }
        }
    }

    fn clear_alarm_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut val = self.read_reg(PCF8523_CONTROL_2)?;
        set_bits(&mut val, 0, 3, 0b1000);
        self.write_reg(PCF8523_CONTROL_2, val)?;
        Ok(())
    }

    fn clear_bsf_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut val = self.read_reg(PCF8523_CONTROL_3)?;
        set_bits(&mut val, 0, 3, 0b100);
        self.write_reg(PCF8523_CONTROL_3, val)?;
        Ok(())
    }

    fn clear_countdown_timer_a_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut val = self.read_reg(PCF8523_CONTROL_2)?;
        set_bits(&mut val, 0, 6, 0b100_0000);
        self.write_reg(PCF8523_CONTROL_2, val)?;
        Ok(())
    }

    fn clear_countdown_timer_b_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut val = self.read_reg(PCF8523_CONTROL_2)?;
        set_bits(&mut val, 0, 5, 0b10_0000);
        self.write_reg(PCF8523_CONTROL_2, val)?;
        Ok(())
    }

    fn clear_second_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut val = self.read_reg(PCF8523_CONTROL_2)?;
        set_bits(&mut val, 0, 4, 0b1_0000);
        self.write_reg(PCF8523_CONTROL_2, val)?;
        Ok(())
    }

    fn clear_watchdog_timer_a_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        self.read_reg(PCF8523_CONTROL_2)?;
        Ok(())
    }

    // TODO test
    pub fn freeze_rtc_time_circuits(&mut self, freeze: bool) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut val = self.read_reg(PCF8523_CONTROL_1)?;
        set_bits(&mut val, 1, 5, 0b10_0000);
        self.write_reg(PCF8523_CONTROL_1, val)?;
        Ok(())
    }

    pub fn get_battery_status(&mut self) -> Result<bool, Pcf8523Error<I2C::Error>> {
        Ok((self.read_reg(PCF8523_CONTROL_3)? >> 2) & 0b1 == 1)
    }

    // fn get_clock_integrity_guaranteed(&mut self) -> Result<bool, Pcf8523Error<I2C::Error>> {
    //     TODO 8.6.1
    // }

    pub fn get_day_of_month(&mut self) -> Result<u8, Pcf8523Error<I2C::Error>> {
        Ok(decode_bcd(self.read_reg(PCF8523_DAYS)? & 0b11_1111))
    }

    pub fn get_day_of_week(&mut self) -> Result<u8, Pcf8523Error<I2C::Error>> {
        Ok(decode_bcd(self.read_reg(PCF8523_WEEKDAYS)? & 0b111))
    }

    pub fn get_interrupt_enabled(&mut self, interrupt: Pcf8523Interrupt) -> Result<bool, Pcf8523Error<I2C::Error>> {
        Ok(
            match interrupt {
                Pcf8523Interrupt::Alarm => get_bits(self.read_reg(PCF8523_CONTROL_1)?, 1, 1) == 1,
                Pcf8523Interrupt::BatterySwitchOver => get_bits(self.read_reg(PCF8523_CONTROL_3)?, 1, 1) == 1,
                Pcf8523Interrupt::CountdownTimerA => get_bits(self.read_reg(PCF8523_CONTROL_2)?, 1, 1) == 1,
                Pcf8523Interrupt::CountdownTimerB => get_bits(self.read_reg(PCF8523_CONTROL_2)?, 1, 0) == 1,
                Pcf8523Interrupt::Second =>  get_bits(self.read_reg(PCF8523_CONTROL_1)?, 1, 2) == 1,
                Pcf8523Interrupt::WatchdogTimerA => get_bits(self.read_reg(PCF8523_CONTROL_2)?, 1, 2) == 1,
            }
        )
    }

    // TODO test
    // fn get_hours(&mut self) -> Result<Hours, Pcf8523Error<I2C::Error>> {
    //     let reg_val = self.read_reg(PCF8523_HOURS)?;
    //     let hours = if get_bits(self.read_reg(PCF8523_CONTROL_1)?, 1, 0b1000) == 0 {
    //         Hours::new(decode_bcd(get_bits(reg_val, 6, 0)), None)
    //     } else {
    //         Hours::new(decode_bcd(get_bits(reg_val, 5, 0)), get_bits(reg_val, 1, 5).try_into().unwrap())
    //     };
    //     Ok(hours)
    // }

    pub fn get_minutes(&mut self) -> Result<u8, Pcf8523Error<I2C::Error>> {
        Ok(decode_bcd(self.read_reg(PCF8523_MINUTES)? & 0b111_1111))
    }

    pub fn get_month(&mut self) -> Result<u8, Pcf8523Error<I2C::Error>> {
        Ok(decode_bcd(self.read_reg(PCF8523_MONTHS)? & 0b1_1111))
    }

    pub fn get_seconds(&mut self) -> Result<u8, Pcf8523Error<I2C::Error>> {
        Ok(decode_bcd(self.read_reg(PCF8523_SECONDS)? & 0b111_1111))
    }

    pub fn get_year(&mut self) -> Result<u8, Pcf8523Error<I2C::Error>> {
        Ok(decode_bcd(self.read_reg(PCF8523_YEARS)? & 0b1111_1111))
    }

    pub fn read_reg(&mut self, reg: u8) -> Result<u8, Pcf8523Error<I2C::Error>> {
        let mut buffer = [0u8];
        self.i2c.write_read(PCF8523_I2C_ADDRESS, &[reg], &mut buffer).map_err(Pcf8523Error::I2C)?;
        Ok(buffer[0])
    }

    // TODO test
    pub fn select_hour_mode(&mut self, mode_12hr: bool) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut val = self.read_reg(PCF8523_CONTROL_1)?;
        set_bits(&mut val, mode_12hr as u8, 3, 0b1000);
        self.write_reg(PCF8523_CONTROL_1, val)?;
        Ok(())
    }

    // TODO test
    pub fn select_oscillator_capacitor(&mut self, cap12_5pf: bool) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut val = self.read_reg(PCF8523_CONTROL_1)?;
        set_bits(&mut val, cap12_5pf as u8, 7, 0b1000_0000);
        self.write_reg(PCF8523_CONTROL_1, val)?;
        Ok(())
    }

    // TODO test
    pub fn select_power_management(&mut self, power_management: PowerManagement) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut val = self.read_reg(PCF8523_CONTROL_3)?;
        set_bits(&mut val, power_management as u8, 5, 0b1110_0000);
        self.write_reg(PCF8523_CONTROL_3, val)?;
        Ok(())
    }

    pub fn software_reset(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        self.write_reg(PCF8523_CONTROL_1, 0b101_1000)
    }

    pub fn write_reg(&mut self, reg: u8, val: u8) -> Result<(), Pcf8523Error<I2C::Error>> {
        Ok(self.i2c.write(PCF8523_I2C_ADDRESS, &[reg, val]).map_err(Pcf8523Error::I2C)?)
    }
}

fn decode_bcd(a: u8) -> u8 {
    (((a >> 4) & 0xf) * 10) + (a & 0xf)
}

fn encode_bcd(a: u8) -> u8 {
    if a >= 100 { panic!("Cannot BCD encode value {} as u8", a); }
    a % 10 | (a / 10 << 4)
}

fn get_bits(byte: u8, bits: u8, lsb_offset: u8) -> u8 {
    (byte >> lsb_offset) & ((1 << (bits)) - 1)
}

fn set_bits(byte: &mut u8, data: u8, lsb_offset: u8, mask: u8) {
    *byte &= !mask;
    *byte |= (data << lsb_offset) & mask
}

#[cfg(test)]
mod tests {
    use embedded_hal::i2c::ErrorKind::Other;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
    use super::*;

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
    fn decode_bcd_floor() {
        assert_eq!(decode_bcd(0b0), 0u8);
    }

    #[test]
    fn decode_bcd_ceiling() {
        assert_eq!(decode_bcd(0b1001_1001), 99u8);
    }

    #[test]
    fn encode_bcd_floor() {
        assert_eq!(encode_bcd(0u8), 0b0);
    }

    #[test]
    fn encode_bcd_ceiling() {
        assert_eq!(encode_bcd(99u8), 0b1001_1001);
    }

    #[test]
    #[should_panic]
    fn encode_bcd_out_of_bounds() {
        encode_bcd(100u8);
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
}
