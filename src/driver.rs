use embedded_hal::i2c::{I2c, Operation};
use crate::bits::{encode_bcd, get_bits, set_bits};
use crate::datetime::Pcf8523DateTime;
use crate::driver::Pcf8523Error::InvalidArgument;
use crate::registers::*;
use crate::typedefs::{CorrectionMode, PowerManagement};

/// Fixed I2C address of RTC module
pub const PCF8523_I2C_ADDRESS: u8 = 0x68;
const PCF8523_CONTROL_3_DEFAULT: u8 = 0b1110_0000;

#[derive(Debug, PartialEq)]
pub enum Pcf8523Error<E> {
    I2C(E),
    InvalidArgument
}

impl <E> From<E> for Pcf8523Error<E> {
    fn from(e: E) -> Self {
        Pcf8523Error::I2C(e)
    }
}

/// NXP PCF8523 hardware module driver
/// - `I2C` HAL I2C bus interface
#[derive(Debug)]
pub struct Pcf8523<I2C> {
    i2c: I2C
}

impl<I2C: I2c> Pcf8523<I2C> {

    /// Constructs a new instance of the module driver.
    pub fn new(i2c: I2C) -> Result<Self, Pcf8523Error<I2C::Error>> {
        let mut peri = Self { i2c };
        peri.i2c.read(PCF8523_I2C_ADDRESS, &mut [0u8])?;
        Ok(peri)
    }

    /// Calibrate for aging adjustment, temperature compensation and accuracy tuning.
    /// - `mode` controls the frequency of correction application
    /// - `offset` -64..63 (inclusive) correction amount applied via `mode` timing
    pub fn calibrate(&mut self, mode: CorrectionMode, offset: i8) -> Result<(), Pcf8523Error<I2C::Error>> {
        if offset < -64 || offset > 63 {
            return Err(InvalidArgument)
        }
        let mut reg_val = (mode as u8) << 7;
        set_bits(&mut reg_val, offset as u8, 0, 0b111_1111);
        Ok(self.write_reg(PCF8523_OFFSET, reg_val)?)
    }

    /// Clears the second interrupt.
    pub fn clear_second_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_2)?;
        set_bits(&mut reg_val, 0, 4, 0b1_0000);
        Ok(self.write_reg(PCF8523_CONTROL_2, reg_val)?)
    }

    /// Disables the alarm interrupt.
    pub fn disable_alarm_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1)?;
        set_bits(&mut reg_val, 0, 1, 0b10);
        Ok(self.write_reg(PCF8523_CONTROL_1, reg_val)?)
    }

    /// Disables the correction interrupt.
    pub fn disable_correction_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1)?;
        set_bits(&mut reg_val, 0, 0, 0b1);
        Ok(self.write_reg(PCF8523_CONTROL_1, reg_val)?)
    }

    /// Disables the day alarm while leaving the configured day value intact.
    pub fn disable_day_alarm(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_DAY_ALARM)?;
        set_bits(&mut reg_val, 1, 7, 0b1000_0000);
        self.write_reg(PCF8523_DAY_ALARM, reg_val)?;
        Ok(())
    }

    /// Disables the hour alarm while leaving the configured hour value intact.
    pub fn disable_hour_alarm(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_HOUR_ALARM)?;
        set_bits(&mut reg_val, 1, 7, 0b1000_0000);
        self.write_reg(PCF8523_HOUR_ALARM, reg_val)?;
        Ok(())
    }

    /// Disables the minute alarm while leaving the configured minute value intact.
    pub fn disable_minute_alarm(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_MINUTE_ALARM)?;
        set_bits(&mut reg_val, 1, 7, 0b1000_0000);
        self.write_reg(PCF8523_MINUTE_ALARM, reg_val)?;
        Ok(())
    }

    /// Disables the second interrupt.
    pub fn disable_second_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1)?;
        set_bits(&mut reg_val, 0, 2, 0b100);
        Ok(self.write_reg(PCF8523_CONTROL_1, reg_val)?)
    }

    /// Disables the weekday alarm while leaving the configured weekday value intact.
    pub fn disable_weekday_alarm(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_WEEKDAY_ALARM)?;
        set_bits(&mut reg_val, 1, 7, 0b1000_0000);
        self.write_reg(PCF8523_WEEKDAY_ALARM, reg_val)?;
        Ok(())
    }

    /// Enables the alarm interrupt.
    pub fn enable_alarm_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1)?;
        set_bits(&mut reg_val, 1, 1, 0b10);
        Ok(self.write_reg(PCF8523_CONTROL_1, reg_val)?)
    }

    /// Enables the correction interrupt, which pulses on every correction cycle.
    pub fn enable_correction_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1)?;
        set_bits(&mut reg_val, 1, 0, 0b1);
        Ok(self.write_reg(PCF8523_CONTROL_1, reg_val)?)
    }

    /// Enables the day alarm.
    /// - `day` 1..31 (inclusive)
    pub fn enable_day_alarm(&mut self, day: u8) -> Result<(), Pcf8523Error<I2C::Error>> {
        if day == 0 || day > 31 { return Err(InvalidArgument) }
        self.write_reg(PCF8523_DAY_ALARM, (0 << 7) | encode_bcd(day))?;
        Ok(())
    }

    /// Enables the hour alarm.
    /// - `hour` 0..23 (inclusive)
    pub fn enable_hour_alarm(&mut self, hour: u8) -> Result<(), Pcf8523Error<I2C::Error>> {
        if hour > 23 { return Err(InvalidArgument) }
        self.write_reg(PCF8523_HOUR_ALARM, (0 << 7) | encode_bcd(hour))?;
        Ok(())
    }

    /// Enables the minute alarm.
    /// - `minute` 0..59 (inclusive)
    pub fn enable_minute_alarm(&mut self, minute: u8) -> Result<(), Pcf8523Error<I2C::Error>> {
        if minute > 59 { return Err(InvalidArgument) }
        self.write_reg(PCF8523_MINUTE_ALARM, (0 << 7) | encode_bcd(minute))?;
        Ok(())
    }

    /// Enables the second interrupt.
    pub fn enable_second_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1)?;
        set_bits(&mut reg_val, 1, 2, 0b100);
        Ok(self.write_reg(PCF8523_CONTROL_1, reg_val)?)
    }

    /// Enables the weekday alarm.
    /// - `weekday` 0..6 (inclusive)
    pub fn enable_weekday_alarm(&mut self, weekday: u8) -> Result<(), Pcf8523Error<I2C::Error>> {
        if weekday > 6 { return Err(InvalidArgument) }
        self.write_reg(PCF8523_WEEKDAY_ALARM, (0 << 7) | weekday)?;
        Ok(())
    }

    /// Determines if the module was initialized.
    pub fn initialized(&mut self) -> Result<bool, Pcf8523Error<I2C::Error>> {
        Ok((self.read_reg(PCF8523_CONTROL_3)? & PCF8523_CONTROL_3_DEFAULT) != PCF8523_CONTROL_3_DEFAULT)
    }

    /// Determines if the module lost power.
    pub fn lost_power(&mut self) -> Result<bool, Pcf8523Error<I2C::Error>> {
        let reg_val = self.read_reg(PCF8523_SECONDS)?;
        Ok((reg_val >> 7) == 1)
    }

    /// Gets the current datetime for the module.
    pub fn now(&mut self) -> Result<Pcf8523DateTime, Pcf8523Error<I2C::Error>> {
        let mut seconds = [0u8];
        let mut minutes = [0u8];
        let mut hours = [0u8];
        let mut day = [0u8];
        let mut month = [0u8];
        let mut year = [0u8];
        self.i2c.transaction(PCF8523_I2C_ADDRESS, &mut [
            Operation::Write(&[PCF8523_SECONDS]), Operation::Read(&mut seconds),
            Operation::Write(&[PCF8523_MINUTES]), Operation::Read(&mut minutes),
            Operation::Write(&[PCF8523_HOURS]), Operation::Read(&mut hours),
            Operation::Write(&[PCF8523_DAYS]), Operation::Read(&mut day),
            Operation::Write(&[PCF8523_MONTHS]), Operation::Read(&mut month),
            Operation::Write(&[PCF8523_YEARS]), Operation::Read(&mut year),
        ])?;
        Ok(
            Pcf8523DateTime {
                seconds: seconds[0],
                minutes: minutes[0],
                hours: hours[0],
                day: day[0],
                month: month[0],
                year: year[0],
            }.bcd_decode()
        )
    }

    /// Reads a value from the register.
    pub fn read_reg(&mut self, reg: u8) -> Result<u8, Pcf8523Error<I2C::Error>> {
        let mut buffer = [0u8];
        self.i2c.write_read(PCF8523_I2C_ADDRESS, &[reg], &mut buffer)?;
        Ok(buffer[0])
    }

    /// Performs a software reset.
    pub fn reset(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        self.write_reg(PCF8523_CONTROL_1, 0b101_1000)
    }

    /// Determines if the on-board oscillator is running.
    pub fn running(&mut self) -> Result<bool, Pcf8523Error<I2C::Error>> {
        let reg_val = self.read_reg(PCF8523_CONTROL_1)?;
        Ok(get_bits(reg_val, 1, 5) == 0)
    }

    /// Sets the module datetime in a single I2C transaction.
    /// - `datetime` initial module datetime
    pub fn set_datetime(
        &mut self,
        datetime: Pcf8523DateTime,
    ) -> Result<(), Pcf8523Error<I2C::Error>> {
        let dt = datetime.encode_bcd();
        // execute all writes in a single transaction to avoid data corruption
        self.i2c.transaction(PCF8523_I2C_ADDRESS, &mut [
            Operation::Write(&[PCF8523_SECONDS, dt.seconds]),
            Operation::Write(&[PCF8523_MINUTES, dt.minutes]),
            Operation::Write(&[PCF8523_HOURS, dt.hours]),
            Operation::Write(&[PCF8523_DAYS, dt.day]),
            Operation::Write(&[PCF8523_MONTHS, dt.month]),
            Operation::Write(&[PCF8523_YEARS, dt.year]),
        ])?;
        Ok(())
    }

    /// Sets the module's power management functions.
    pub fn set_power_management(&mut self, power_management: PowerManagement) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_3)?;
        set_bits(&mut reg_val, power_management as u8, 5, 0b1110_0000);
        self.write_reg(PCF8523_CONTROL_3, reg_val)?;
        Ok(())
    }

    /// Starts the module (if not already started).
    pub fn start(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1)?;
        // check and toggle OS flag if necessary
        if get_bits(reg_val, 1, 5) == 1 {
            set_bits(&mut reg_val, 0, 5, 0b10_0000);
            self.write_reg(PCF8523_CONTROL_1, reg_val)?
        }
        Ok(())
    }

    /// Stops the module (if not already stopped).
    pub fn stop(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1)?;
        if get_bits(reg_val, 1, 5) == 0 {
            set_bits(&mut reg_val, 1, 5, 0b10_0000);
            self.write_reg(PCF8523_CONTROL_1, reg_val)?
        }
        Ok(())
    }

    /// Writes a value to a register.
    /// - `reg` register address
    /// - `val` value to write
    pub fn write_reg(&mut self, reg: u8, val: u8) -> Result<(), Pcf8523Error<I2C::Error>> {
        Ok(self.i2c.write(PCF8523_I2C_ADDRESS, &[reg, val])?)
    }
}