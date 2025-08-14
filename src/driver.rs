use embedded_hal::i2c::I2c;
use crate::typedefs::{Pcf8523Error, Pcf8523Interrupt, PowerManagement};
use crate::bits::{decode_bcd, get_bits, set_bits};
use crate::registers::{PCF8523_CONTROL_1, PCF8523_CONTROL_2, PCF8523_CONTROL_3, PCF8523_DAYS, PCF8523_MINUTES, PCF8523_MONTHS, PCF8523_SECONDS, PCF8523_WEEKDAYS, PCF8523_YEARS};

pub const PCF8523_I2C_ADDRESS: u8 = 0x68;

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