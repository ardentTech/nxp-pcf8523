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

pub struct Pcf8523<I2C> {
    i2c: I2C
}
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

    pub fn get_battery_status(&mut self, low: &mut bool) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut val = self.read_reg(PCF8523_CONTROL_3)?;
        *low = (val >> 2) & 0b1 == 1;
        Ok(())
    }

    pub fn get_interrupt_enabled(&mut self, interrupt: Pcf8523Interrupt, enabled: &mut bool) -> Result<(), Pcf8523Error<I2C::Error>> {
        *enabled = match interrupt {
            Pcf8523Interrupt::Alarm => get_bits(self.read_reg(PCF8523_CONTROL_1)?, 1, 1) == 1,
            Pcf8523Interrupt::BatterySwitchOver => get_bits(self.read_reg(PCF8523_CONTROL_3)?, 1, 1) == 1,
            Pcf8523Interrupt::CountdownTimerA => get_bits(self.read_reg(PCF8523_CONTROL_2)?, 1, 1) == 1,
            Pcf8523Interrupt::CountdownTimerB => get_bits(self.read_reg(PCF8523_CONTROL_2)?, 1, 0) == 1,
            Pcf8523Interrupt::Second =>  get_bits(self.read_reg(PCF8523_CONTROL_1)?, 1, 2) == 1,
            Pcf8523Interrupt::WatchdogTimerA => get_bits(self.read_reg(PCF8523_CONTROL_2)?, 1, 2) == 1,
        };
        Ok(())
    }

    pub fn read_reg(&mut self, reg: u8) -> Result<u8, Pcf8523Error<I2C::Error>> {
        let mut buffer = [0u8];
        self.i2c.write_read(PCF8523_I2C_ADDRESS, &[reg], &mut buffer).map_err(Pcf8523Error::I2C)?;
        Ok(buffer[0])
    }

    pub fn reset(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        self.write_reg(PCF8523_CONTROL_1, 0b0101_1000)
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
        let mut low = false;
        driver.get_battery_status(&mut low).unwrap();
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
        let mut low = true;
        driver.get_battery_status(&mut low).unwrap();
        assert!(!low);
        i2c.done();
    }

    #[test]
    fn get_interrupt_enabled_alarm_false() {
        let expectations = [
            i2c_reg_read(PCF8523_CONTROL_1, 0b0),
        ];
        let mut i2c = I2cMock::new(&expectations);
        let mut driver = Pcf8523::new(&mut i2c);
        let mut enabled = true;
        driver.get_interrupt_enabled(Pcf8523Interrupt::Alarm, &mut enabled).unwrap();
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
        let mut enabled = false;
        driver.get_interrupt_enabled(Pcf8523Interrupt::Alarm, &mut enabled).unwrap();
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
        let mut enabled = true;
        driver.get_interrupt_enabled(Pcf8523Interrupt::BatterySwitchOver, &mut enabled).unwrap();
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
        let mut enabled = false;
        driver.get_interrupt_enabled(Pcf8523Interrupt::BatterySwitchOver, &mut enabled).unwrap();
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
        let mut enabled = true;
        driver.get_interrupt_enabled(Pcf8523Interrupt::CountdownTimerA, &mut enabled).unwrap();
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
        let mut enabled = false;
        driver.get_interrupt_enabled(Pcf8523Interrupt::CountdownTimerA, &mut enabled).unwrap();
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
        let mut enabled = true;
        driver.get_interrupt_enabled(Pcf8523Interrupt::CountdownTimerB, &mut enabled).unwrap();
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
        let mut enabled = false;
        driver.get_interrupt_enabled(Pcf8523Interrupt::CountdownTimerB, &mut enabled).unwrap();
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
        let mut enabled = true;
        driver.get_interrupt_enabled(Pcf8523Interrupt::Second, &mut enabled).unwrap();
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
        let mut enabled = false;
        driver.get_interrupt_enabled(Pcf8523Interrupt::Second, &mut enabled).unwrap();
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
        let mut enabled = true;
        driver.get_interrupt_enabled(Pcf8523Interrupt::WatchdogTimerA, &mut enabled).unwrap();
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
        let mut enabled = false;
        driver.get_interrupt_enabled(Pcf8523Interrupt::WatchdogTimerA, &mut enabled).unwrap();
        assert!(enabled);
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
        driver.reset().unwrap();
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
