use embedded_hal::i2c::{I2c, Operation};
use crate::bits::{encode_bcd, get_bits, set_bits};
use crate::datetime::Pcf8523DateTime;
use crate::driver::Pcf8523Error::{InvalidArgument, InvalidTimerCountdown};
use crate::registers::*;
use crate::typedefs::{CorrectionMode, TimerBInterruptMode, PowerManagement, TimerA, TimerB, TimerMode, Variant, Int2};
use crate::typedefs::TimerMode::{Countdown, Watchdog};

/// Fixed I2C address of RTC module
pub const PCF8523_I2C_ADDRESS: u8 = 0x68;
const PCF8523_CONTROL_3_DEFAULT: u8 = 0b1110_0000;

#[derive(Debug, PartialEq)]
pub enum Pcf8523Error<E> {
    I2C(E),
    InvalidArgument,
    InvalidTimerCountdown,
    InconsistentTimerCounter,
}

impl <E> From<E> for Pcf8523Error<E> {
    fn from(e: E) -> Self {
        Pcf8523Error::I2C(e)
    }
}

/// NXP PCF8523 hardware module driver
/// - `I2C` HAL I2C bus interface
#[derive(Debug)]
pub struct Pcf8523<I2C, V> {
    i2c: I2C,
    variant: V
}

impl<I2C: I2c, V: Variant> Pcf8523<I2C, V> {

    /// Constructs a new instance of the module driver and pings the peripheral.
    pub fn new(i2c: I2C, variant: V) -> Result<Self, Pcf8523Error<I2C::Error>> {
        let mut peri = Self { i2c, variant };
        peri.i2c.read(PCF8523_I2C_ADDRESS, &mut [0u8])?;
        Ok(peri)
    }

    /// Calibrate for aging adjustment, temperature compensation and accuracy tuning.
    /// - `mode` specifies the frequency of correction application
    /// - `offset` -64..63 (inclusive) correction amount applied via `mode` timing
    pub fn calibrate(&mut self, mode: CorrectionMode, offset: i8) -> Result<(), Pcf8523Error<I2C::Error>> {
        if offset < -64 || offset > 63 {
            return Err(InvalidArgument)
        }
        let mut reg_val = (mode as u8) << 7;
        set_bits(&mut reg_val, offset as u8, 0, 0b111_1111);
        self.write_reg(PCF8523_OFFSET, reg_val)
    }

    /// Clears the alarm interrupt.
    pub fn clear_alarm_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_2)?;
        set_bits(&mut reg_val, 0b0_1110, 3, 0b1111_1000);
        self.write_reg(PCF8523_CONTROL_2, reg_val)
    }

    /// Clears the second interrupt.
    pub fn clear_second_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_2)?;
        set_bits(&mut reg_val, 0b0_1101, 3, 0b1111_1000);
        self.write_reg(PCF8523_CONTROL_2, reg_val)
    }

    /// Clears the Timer A interrupt
    /// - `timer` TimerA configuration
    pub fn clear_timer_a_interrupt(&mut self, timer: &TimerA) -> Result<(), Pcf8523Error<I2C::Error>> {
        match timer.mode {
            Countdown => {
                let mut reg_val = self.read_reg(PCF8523_CONTROL_2)?;
                set_bits(&mut reg_val, 0b0_0111, 3, 0b1111_1000);
                self.write_reg(PCF8523_CONTROL_2, reg_val)
            },
            Watchdog => {
                self.read_reg(PCF8523_CONTROL_2)?;
                Ok(())
            }
        }
    }

    /// Disables the alarm interrupt.
    pub fn disable_alarm_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1)?;
        set_bits(&mut reg_val, 0, 1, 0b10);
        self.write_reg(PCF8523_CONTROL_1, reg_val)
    }

    /// Disables battery low detection.
    pub fn disable_battery_low_detection(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_3)?;
        set_bits(&mut reg_val, 0, 0, 0b1);
        self.write_reg(PCF8523_CONTROL_3, reg_val);
        Ok(())
    }

    /// Disables CLKOUT.
    fn disable_clkout(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut tmr_clkout_ctrl = self.read_reg(PCF8523_TMR_CLKOUT_CTRL)?;
        set_bits(&mut tmr_clkout_ctrl, 0b111, 3, 0b11_1000);
        self.write_reg(PCF8523_TMR_CLKOUT_CTRL, tmr_clkout_ctrl)
    }

    /// Disables the correction interrupt.
    pub fn disable_correction_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1)?;
        set_bits(&mut reg_val, 0, 0, 0b1);
        self.write_reg(PCF8523_CONTROL_1, reg_val)
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
        self.write_reg(PCF8523_CONTROL_1, reg_val)
    }

    /// Disables the Timer A interrupt.
    pub fn disable_timer_a_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_2)?;
        set_bits(&mut reg_val, 0, 1, 0b110);
        self.write_reg(PCF8523_CONTROL_2, reg_val)
    }

    /// Disables the weekday alarm while leaving the configured weekday value intact.
    pub fn disable_weekday_alarm(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_WEEKDAY_ALARM)?;
        set_bits(&mut reg_val, 1, 7, 0b1000_0000);
        self.write_reg(PCF8523_WEEKDAY_ALARM, reg_val)?;
        Ok(())
    }

    /// Enables the alarm interrupt.
    fn enable_alarm_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1)?;
        set_bits(&mut reg_val, 1, 1, 0b10);
        self.write_reg(PCF8523_CONTROL_1, reg_val)
    }

    /// Enables battery low detection.
    ///
    /// Generates an interrupt on INT1 (open-drain) when battery voltage drops below 2.5V (typical).
    /// The generated interrupt can only be cleared by replacing the battery.
    pub fn enable_battery_low_detection(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        self.disable_clkout()?;
        let mut reg_val = self.read_reg(PCF8523_CONTROL_3)?;
        set_bits(&mut reg_val, 1, 0, 0b1);
        self.write_reg(PCF8523_CONTROL_3, reg_val);
        Ok(())
    }

    /// Enables the correction interrupt, which pulses on every correction cycle.
    pub fn enable_correction_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1)?;
        set_bits(&mut reg_val, 1, 0, 0b1);
        self.write_reg(PCF8523_CONTROL_1, reg_val)
    }

    /// Enables the day alarm.
    /// - `day` 1..31 (inclusive)
    pub fn enable_day_alarm(&mut self, day: u8) -> Result<(), Pcf8523Error<I2C::Error>> {
        if day == 0 || day > 31 { return Err(InvalidArgument) }
        self.disable_clkout()?;
        self.write_reg(PCF8523_DAY_ALARM, (0 << 7) | encode_bcd(day))?;
        self.enable_alarm_interrupt()?;
        Ok(())
    }

    /// Enables the hour alarm.
    /// - `hour` 0..23 (inclusive)
    pub fn enable_hour_alarm(&mut self, hour: u8) -> Result<(), Pcf8523Error<I2C::Error>> {
        if hour > 23 { return Err(InvalidArgument) }
        self.disable_clkout()?;
        self.write_reg(PCF8523_HOUR_ALARM, (0 << 7) | encode_bcd(hour))?;
        self.enable_alarm_interrupt()?;
        Ok(())
    }

    /// Enables the minute alarm.
    /// - `minute` 0..59 (inclusive)
    pub fn enable_minute_alarm(&mut self, minute: u8) -> Result<(), Pcf8523Error<I2C::Error>> {
        if minute > 59 { return Err(InvalidArgument) }
        self.disable_clkout()?;
        self.write_reg(PCF8523_MINUTE_ALARM, (0 << 7) | encode_bcd(minute))?;
        self.enable_alarm_interrupt()?;
        Ok(())
    }

    /// Enables the second interrupt.
    /// - `pulsed` configures the interrupt as pulsed or permanently active
    pub fn enable_second_interrupt(&mut self, pulsed: bool) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut control_1 = self.read_reg(PCF8523_CONTROL_1)?;
        set_bits(&mut control_1, 1, 2, 0b100);
        self.write_reg(PCF8523_CONTROL_1, control_1)?;

        let mut clkout_ctrl = self.read_reg(PCF8523_TMR_CLKOUT_CTRL)?;
        set_bits(&mut clkout_ctrl, pulsed as u8, 7, 0b1000_0000);
        // disable CLKOUT
        set_bits(&mut clkout_ctrl, 0b111, 3, 0b11_1000);
        self.write_reg(PCF8523_TMR_CLKOUT_CTRL, clkout_ctrl)
    }

    /// Enables the Timer A interrupt and disables the CLKOUT
    /// - `mode` countdown or watchdog
    pub fn enable_timer_a_interrupt(&mut self, mode: TimerMode) -> Result<(), Pcf8523Error<I2C::Error>> {
        self.disable_clkout()?;

        let mut control_2 = self.read_reg(PCF8523_CONTROL_2)?;
        match mode {
            Countdown => set_bits(&mut control_2, 0b01, 1, 0b110),
            Watchdog => set_bits(&mut control_2, 0b10, 1, 0b110),
        }
        self.write_reg(PCF8523_CONTROL_2, control_2)
    }

    /// Enables the weekday alarm.
    /// - `weekday` 0..6 (inclusive)
    pub fn enable_weekday_alarm(&mut self, weekday: u8) -> Result<(), Pcf8523Error<I2C::Error>> {
        if weekday > 6 { return Err(InvalidArgument) }
        self.disable_clkout()?;
        self.write_reg(PCF8523_WEEKDAY_ALARM, (0 << 7) | weekday)?;
        self.enable_alarm_interrupt()?;
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
        let mut second = [0u8];
        let mut minute = [0u8];
        let mut hour = [0u8];
        let mut day = [0u8];
        let mut month = [0u8];
        let mut year = [0u8];
        self.i2c.transaction(PCF8523_I2C_ADDRESS, &mut [
            Operation::Write(&[PCF8523_SECONDS]), Operation::Read(&mut second),
            Operation::Write(&[PCF8523_MINUTES]), Operation::Read(&mut minute),
            Operation::Write(&[PCF8523_HOURS]), Operation::Read(&mut hour),
            Operation::Write(&[PCF8523_DAYS]), Operation::Read(&mut day),
            Operation::Write(&[PCF8523_MONTHS]), Operation::Read(&mut month),
            Operation::Write(&[PCF8523_YEARS]), Operation::Read(&mut year),
        ])?;
        Ok(
            Pcf8523DateTime {
                second: second[0],
                minute: minute[0],
                hour: hour[0],
                day: day[0],
                month: month[0],
                year: year[0],
            }.bcd_decode()
        )
    }

    /// Reads a value from the register.
    /// - `addr` register address to read from
    pub fn read_reg(&mut self, addr: u8) -> Result<u8, Pcf8523Error<I2C::Error>> {
        let mut buffer = [0u8];
        self.i2c.write_read(PCF8523_I2C_ADDRESS, &[addr], &mut buffer)?;
        Ok(buffer[0])
    }

    /// Reloads the Timer A Watchdog counter.
    pub fn reload_timer_a_watchdog_countdown(&mut self, timer: &TimerA) -> Result<(), Pcf8523Error<I2C::Error>> {
        let tmr_clkout_ctrl = self.read_reg(PCF8523_TMR_CLKOUT_CTRL)?;
        if timer.mode == Countdown || get_bits(tmr_clkout_ctrl, 2, 1) != 0b10 {
            return Err(InvalidArgument);
        }
        self.write_reg(PCF8523_TMR_A_REG, timer.countdown)
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

    /// Sets the module datetime in a single I2C transaction to avoid data corruption.
    /// - `datetime` datetime to configure the module for
    pub fn set_datetime(&mut self, datetime: Pcf8523DateTime) -> Result<(), Pcf8523Error<I2C::Error>> {
        let dt = datetime.encode_bcd();
        self.i2c.transaction(PCF8523_I2C_ADDRESS, &mut [
            Operation::Write(&[PCF8523_SECONDS, dt.second]),
            Operation::Write(&[PCF8523_MINUTES, dt.minute]),
            Operation::Write(&[PCF8523_HOURS, dt.hour]),
            Operation::Write(&[PCF8523_DAYS, dt.day]),
            Operation::Write(&[PCF8523_MONTHS, dt.month]),
            Operation::Write(&[PCF8523_YEARS, dt.year]),
        ])?;
        Ok(())
    }

    /// Sets the module's power management functions.
    /// - `power_management` selected power management option for the module
    pub fn set_power_management(&mut self, power_management: PowerManagement) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_3)?;
        set_bits(&mut reg_val, power_management as u8, 5, 0b1110_0000);
        self.write_reg(PCF8523_CONTROL_3, reg_val)?;
        Ok(())
    }

    /// Starts the module, if not already started.
    pub fn start(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1)?;
        if get_bits(reg_val, 1, 5) == 1 {
            set_bits(&mut reg_val, 0, 5, 0b10_0000);
            self.write_reg(PCF8523_CONTROL_1, reg_val)?
        }
        Ok(())
    }

    /// Starts Timer A.
    /// - `timer` TimerA configuration
    pub fn start_timer_a(&mut self, timer: &TimerA) -> Result<(), Pcf8523Error<I2C::Error>> {
        if timer.countdown == 0 { return Err(InvalidTimerCountdown) }

        let mut tmr_clkout_ctrl = self.read_reg(PCF8523_TMR_CLKOUT_CTRL)?;
        let tac = get_bits(tmr_clkout_ctrl, 2, 1);
        // if timer A countdown or watchdog is running, disable it
        if tac == 1 || tac == 2 {
            set_bits(&mut tmr_clkout_ctrl, 00, 1, 0b110);
            self.write_reg(PCF8523_TMR_CLKOUT_CTRL, tmr_clkout_ctrl)?;
        }

        // set interrupt mode
        set_bits(&mut tmr_clkout_ctrl, timer.interrupt_mode.into(), 7, 0b1000_0000);

        // set source clock frequency
        self.write_reg(PCF8523_TMR_A_FREQ_CTRL, timer.source_clock as u8)?;

        // enable interrupt
        self.enable_timer_a_interrupt(timer.mode)?;

        // enable timer mode
        set_bits(&mut tmr_clkout_ctrl, timer.mode.into(), 1, 0b110);
        self.write_reg(PCF8523_TMR_CLKOUT_CTRL, tmr_clkout_ctrl)?;

        // start the timer by setting a countdown value
        self.write_reg(PCF8523_TMR_A_REG, timer.countdown)
    }

    /// Stops the module, if not already stopped.
    pub fn stop(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1)?;
        if get_bits(reg_val, 1, 5) == 0 {
            set_bits(&mut reg_val, 1, 5, 0b10_0000);
            self.write_reg(PCF8523_CONTROL_1, reg_val)?
        }
        Ok(())
    }

    /// Stops Timer A.
    pub fn stop_timer_a(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        self.write_reg(PCF8523_TMR_A_REG, 0)
    }

    /// Gets the Timer A counter. This is the current value, and not the TimerA.countdown value that
    /// was initially configured. In watchdog mode, this value represents the timer period and will
    /// be constant until Timer A is reconfigured. In countdown mode, this value is dynamic in
    /// relation to passing time. Since the timer cannot be frozen during the read, it's read
    /// twice and compared for equality.
    pub fn timer_a_counter(&mut self) -> Result<u8, Pcf8523Error<I2C::Error>> {
        self.timer_counter(PCF8523_TMR_A_REG)
    }

    fn timer_counter(&mut self, reg_addr: u8) -> Result<u8, Pcf8523Error<I2C::Error>> {
        let a = self.read_reg(reg_addr)?;
        let b = self.read_reg(reg_addr)?;
        if a == b { Ok(a) } else { Err(Pcf8523Error::InconsistentTimerCounter) }
    }

    /// Writes a value to a register.
    /// - `reg` register address
    /// - `val` value to write
    pub fn write_reg(&mut self, reg: u8, val: u8) -> Result<(), Pcf8523Error<I2C::Error>> {
        Ok(self.i2c.write(PCF8523_I2C_ADDRESS, &[reg, val])?)
    }
}

impl<I2C: I2c, V: Variant + Int2> Pcf8523<I2C, V> {

    /// Clears the Timer B interrupt.
    pub fn clear_timer_b_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_2)?;
        set_bits(&mut reg_val, 0b0_1011, 3, 0b1111_1000);
        self.write_reg(PCF8523_CONTROL_2, reg_val)
    }

    /// Disables the Timer B interrupt.
    pub fn disable_timer_b_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_2)?;
        set_bits(&mut reg_val, 0, 0, 0b1);
        self.write_reg(PCF8523_CONTROL_2, reg_val)
    }

    /// Enables the Timer B interrupt.
    pub fn enable_timer_b_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        self.disable_clkout()?;
        let mut reg_val = self.read_reg(PCF8523_CONTROL_2)?;
        set_bits(&mut reg_val, 1, 0, 0b1);
        self.write_reg(PCF8523_CONTROL_2, reg_val)
    }

    /// Starts Timer B, which only supports countdown timer mode.
    /// - `timer` TimerB configuration
    pub fn start_timer_b(&mut self, timer: &TimerB) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut tmr_clkout_ctrl = self.read_reg(PCF8523_TMR_CLKOUT_CTRL)?;
        // disable timer (if enabled)
        if get_bits(tmr_clkout_ctrl, 1, 0) == 1 {
            set_bits(&mut tmr_clkout_ctrl, 0, 0, 0b1);
            self.write_reg(PCF8523_TMR_CLKOUT_CTRL, tmr_clkout_ctrl)?;
        }

        let mut tmr_b_freq_ctrl = self.read_reg(PCF8523_TMR_B_FREQ_CTRL)?;

        // set timer frequency
        set_bits(&mut tmr_b_freq_ctrl, timer.source_clock as u8, 0, 0b111);

        match timer.interrupt_mode {
            // if pulsed interrupt, set low pulse width
            TimerBInterruptMode::Pulsed(width) => {
                set_bits(&mut tmr_b_freq_ctrl, width as u8, 4, 0b111_0000);
            },
            _ => {}
        }
        self.write_reg(PCF8523_TMR_B_FREQ_CTRL, tmr_b_freq_ctrl)?;

        self.enable_timer_b_interrupt()?;

        // set interrupt mode
        set_bits(&mut tmr_clkout_ctrl, timer.interrupt_mode.into(), 6, 0b100_0000);
        // set countdown val
        self.write_reg(PCF8523_TMR_B_REG, timer.countdown)?;

        // enable timer
        set_bits(&mut tmr_clkout_ctrl, 1, 0, 0b1);
        self.write_reg(PCF8523_TMR_CLKOUT_CTRL, tmr_clkout_ctrl)
    }

    /// Stops Timer B.
    pub fn stop_timer_b(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_TMR_CLKOUT_CTRL)?;
        set_bits(&mut reg_val, 0, 0, 0b1);
        self.write_reg(PCF8523_TMR_CLKOUT_CTRL, reg_val)
    }

    /// Gets the Timer B counter. This is the current value, and not the TimerB.countdown value that
    /// was initially configured. This value is dynamic in relation to passing timeSince the timer
    /// cannot be frozen during the read, it's read twice and compared for equality.
    pub fn timer_b_counter(&mut self) -> Result<u8, Pcf8523Error<I2C::Error>> {
        self.timer_counter(PCF8523_TMR_B_REG)
    }

}