use embedded_hal::i2c::{ErrorType, I2c, Operation};
use crate::datetime::Pcf8523DateTime;
use crate::registers::*;

pub const PCF8523_I2C_ADDRESS: u8 = 0x68;

#[derive(Debug, PartialEq)]
pub enum Pcf8523Error<E> {
    I2C(E)
}
impl <E> From<E> for Pcf8523Error<E> {
    fn from(e: E) -> Self {
        Pcf8523Error::I2C(e)
    }
}

#[derive(Debug)]
pub struct Pcf8523<I2C> {
    i2c: I2C
}

impl<I2C: I2c> Pcf8523<I2C> {
    pub fn new(i2c: I2C) -> Result<Self, Pcf8523Error<I2C::Error>> {
        let mut peri = Self { i2c };
        peri.i2c.read(PCF8523_I2C_ADDRESS, &mut [0u8])?;
        Ok(peri)
    }

    pub fn set_datetime(
        &mut self,
        datetime: Pcf8523DateTime,
    ) -> Result<(), Pcf8523Error<I2C::Error>> {
        let dt = datetime.encode();
        self.i2c.transaction(PCF8523_I2C_ADDRESS, &mut [
            Operation::Write(&[PCF8523_SECONDS, dt.seconds]),
            Operation::Write(&[PCF8523_MINUTES, dt.minutes]),
            Operation::Write(&[PCF8523_HOURS, dt.hours]),
            Operation::Write(&[PCF8523_DAYS, dt.day]),
            // Operation::Write(&[PCF8523_WEEKDAYS, dt.day_of_week]), // TODO
            Operation::Write(&[PCF8523_MONTHS, dt.month]),
            Operation::Write(&[PCF8523_YEARS, dt.year]),
        ])?;
        Ok(())
    }

    pub fn now(&mut self) -> Result<Pcf8523DateTime, Pcf8523Error<I2C::Error>> {
        let mut seconds = [0u8];
        let mut minutes = [0u8];
        let mut hours = [0u8];
        let mut day = [0u8];
        //let mut day_of_week = [0u8];
        let mut month = [0u8];
        let mut year = [0u8];
        self.i2c.transaction(PCF8523_I2C_ADDRESS, &mut [
            Operation::Write(&[PCF8523_SECONDS]), Operation::Read(&mut seconds),
            Operation::Write(&[PCF8523_MINUTES]), Operation::Read(&mut minutes),
            Operation::Write(&[PCF8523_HOURS]), Operation::Read(&mut hours),
            Operation::Write(&[PCF8523_DAYS]), Operation::Read(&mut day),
            //Operation::Write(&[PCF8523_WEEKDAYS]), Operation::Read(&mut day_of_week),
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
            }.decode()
        )
    }

    pub fn read_reg(&mut self, reg: u8) -> Result<u8, Pcf8523Error<I2C::Error>> {
        let mut buffer = [0u8];
        self.i2c.write_read(PCF8523_I2C_ADDRESS, &[reg], &mut buffer)?;
        Ok(buffer[0])
    }

    pub fn reset(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        self.write_reg(PCF8523_CONTROL_1, 0b101_1000)
    }

    pub fn write_reg(&mut self, reg: u8, val: u8) -> Result<(), Pcf8523Error<I2C::Error>> {
        Ok(self.i2c.write(PCF8523_I2C_ADDRESS, &[reg, val])?)
    }
}