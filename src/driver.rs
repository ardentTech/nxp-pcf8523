use crate::bits::{encode_bcd, get_bits, set_bits};
use crate::datetime::Pcf8523DateTime;
use crate::driver::Pcf8523Error::{Internal, InvalidArgument, InvalidState, InvalidTimerCountdown};
use crate::registers::*;
use crate::typedefs::ClkOut::Frequency0Hz;
use crate::typedefs::TimerMode::{Countdown, Watchdog};
use crate::typedefs::*;

#[cfg(feature = "sync")]
use embedded_hal::i2c::{I2c, Operation};

#[cfg(not(feature = "sync"))]
use embedded_hal_async::i2c::{I2c, Operation};

/// Fixed I2C address of RTC module
pub const PCF8523_I2C_ADDRESS: u8 = 0x68;

#[derive(Debug, PartialEq)]
pub enum Pcf8523Error<E> {
    I2C(E),
    InconsistentTimerCounter,
    Internal,
    InvalidArgument,
    InvalidState,
    InvalidTimerCountdown,
}

impl<E> From<E> for Pcf8523Error<E> {
    fn from(e: E) -> Self {
        Pcf8523Error::I2C(e)
    }
}

/// NXP PCF8523 hardware module driver
/// - `I2C` HAL I2C bus interface
/// - `V` chip variant
#[allow(dead_code)]
#[derive(Debug)]
pub struct Pcf8523<I2C, V> {
    i2c: I2C,
    variant: V,
}

impl<I2C: I2c, V: Variant> Pcf8523<I2C, V> {
    /// Constructs a new instance of the module driver and pings the peripheral.
    #[maybe_async::maybe_async]
    pub async fn new(i2c: I2C, variant: V) -> Result<Self, Pcf8523Error<I2C::Error>> {
        let mut peri = Self { i2c, variant };
        peri.i2c.read(PCF8523_I2C_ADDRESS, &mut [0u8]).await?;
        Ok(peri)
    }

    /// Clears the alarm interrupt.
    #[maybe_async::maybe_async]
    pub async fn clear_alarm_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_2).await?;
        set_bits(&mut reg_val, 0b0_1110, 3, 0b1111_1000);
        self.write_reg(PCF8523_CONTROL_2, reg_val).await
    }

    /// Clears the battery switch-over interrupt.
    #[maybe_async::maybe_async]
    pub async fn clear_battery_switch_over_interrupt(
        &mut self,
    ) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_3).await?;
        set_bits(&mut reg_val, 0, 3, 0b1000);
        self.write_reg(PCF8523_CONTROL_3, reg_val).await
    }

    /// Clears the second interrupt.
    #[maybe_async::maybe_async]
    pub async fn clear_second_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_2).await?;
        set_bits(&mut reg_val, 0b0_1101, 3, 0b1111_1000);
        self.write_reg(PCF8523_CONTROL_2, reg_val).await
    }

    /// Clears the Timer A interrupt
    /// - `timer` TimerA configuration
    #[maybe_async::maybe_async]
    pub async fn clear_timer_a_interrupt(
        &mut self,
        timer: &TimerA,
    ) -> Result<(), Pcf8523Error<I2C::Error>> {
        match timer.mode {
            Countdown => {
                let mut reg_val = self.read_reg(PCF8523_CONTROL_2).await?;
                set_bits(&mut reg_val, 0b0_0111, 3, 0b1111_1000);
                self.write_reg(PCF8523_CONTROL_2, reg_val).await
            }
            Watchdog => {
                self.read_reg(PCF8523_CONTROL_2).await?;
                Ok(())
            }
        }
    }

    /// Disables the alarm interrupt.
    #[maybe_async::maybe_async]
    pub async fn disable_alarm_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1).await?;
        set_bits(&mut reg_val, 0, 1, 0b10);
        self.write_reg(PCF8523_CONTROL_1, reg_val).await
    }

    /// Disables battery low detection.
    #[maybe_async::maybe_async]
    pub async fn disable_battery_low_detection_interrupt(
        &mut self,
    ) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_3).await?;
        set_bits(&mut reg_val, 0, 0, 0b1);
        self.write_reg(PCF8523_CONTROL_3, reg_val).await
    }

    /// Disables the battery switch-over interrupt.
    #[maybe_async::maybe_async]
    pub async fn disable_battery_switch_over_interrupt(
        &mut self,
    ) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_3).await?;
        set_bits(&mut reg_val, 0, 1, 0b10);
        self.write_reg(PCF8523_CONTROL_3, reg_val).await
    }

    /// Disables the correction interrupt.
    #[maybe_async::maybe_async]
    pub async fn disable_correction_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1).await?;
        set_bits(&mut reg_val, 0, 0, 0b1);
        self.write_reg(PCF8523_CONTROL_1, reg_val).await
    }

    /// Disables the day alarm while leaving the configured day value intact.
    #[maybe_async::maybe_async]
    pub async fn disable_day_alarm(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_DAY_ALARM).await?;
        set_bits(&mut reg_val, 1, 7, 0b1000_0000);
        self.write_reg(PCF8523_DAY_ALARM, reg_val).await
    }

    /// Disables the hour alarm while leaving the configured hour value intact.
    #[maybe_async::maybe_async]
    pub async fn disable_hour_alarm(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_HOUR_ALARM).await?;
        set_bits(&mut reg_val, 1, 7, 0b1000_0000);
        self.write_reg(PCF8523_HOUR_ALARM, reg_val).await
    }

    /// Disables the minute alarm while leaving the configured minute value intact.
    #[maybe_async::maybe_async]
    pub async fn disable_minute_alarm(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_MINUTE_ALARM).await?;
        set_bits(&mut reg_val, 1, 7, 0b1000_0000);
        self.write_reg(PCF8523_MINUTE_ALARM, reg_val).await
    }

    /// Disables the weekday alarm while leaving the configured weekday value intact.
    #[maybe_async::maybe_async]
    pub async fn disable_weekday_alarm(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_WEEKDAY_ALARM).await?;
        set_bits(&mut reg_val, 1, 7, 0b1000_0000);
        self.write_reg(PCF8523_WEEKDAY_ALARM, reg_val).await
    }

    /// Enables the alarm interrupt.
    #[maybe_async::maybe_async]
    async fn enable_alarm_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1).await?;
        set_bits(&mut reg_val, 1, 1, 0b10);
        self.write_reg(PCF8523_CONTROL_1, reg_val).await
    }

    /// Enables battery low detection interrupt.
    ///
    /// Generates an interrupt on INT1 (open-drain) when battery voltage drops below 2.5V (typical).
    /// The generated interrupt can only be cleared by replacing the battery.
    /// Power management must first be configured for battery low detection.
    #[maybe_async::maybe_async]
    pub async fn enable_battery_low_detection_interrupt(
        &mut self,
    ) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_3).await?;
        let pm_bits = get_bits(reg_val, 3, 5);
        match PowerManagement::try_from(pm_bits) {
            Ok(pm) => match pm {
                PowerManagement::SwitchOverStandardOnLowDetectionOn
                | PowerManagement::SwitchOverDirectOnLowDetectionOn
                | PowerManagement::SwitchOverOffLowDetectionOn => {
                    self.set_clkout(ClkOut::Frequency0Hz).await?;
                    set_bits(&mut reg_val, 1, 0, 0b1);
                    self.write_reg(PCF8523_CONTROL_3, reg_val).await
                }
                _ => Err(InvalidState),
            },
            Err(_) => Err(Internal),
        }
    }

    /// Enables battery switch-over interrupt.
    ///
    /// Power management must first be configured for battery switch-over.
    #[maybe_async::maybe_async]
    pub async fn enable_battery_switch_over_interrupt(
        &mut self,
    ) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_3).await?;
        let pm_bits = get_bits(reg_val, 3, 5);
        match PowerManagement::try_from(pm_bits) {
            Ok(pm) => match pm {
                PowerManagement::SwitchOverStandardOnLowDetectionOn
                | PowerManagement::SwitchOverDirectOnLowDetectionOn
                | PowerManagement::SwitchOverStandardOnLowDetectionOff
                | PowerManagement::SwitchOverDirectOnLowDetectionOff => {
                    self.set_clkout(ClkOut::Frequency0Hz).await?;
                    set_bits(&mut reg_val, 1, 1, 0b10);
                    self.write_reg(PCF8523_CONTROL_3, reg_val).await
                }
                _ => Err(InvalidState),
            },
            Err(_) => Err(Internal),
        }
    }

    /// Enables the correction interrupt, which pulses on every correction cycle.
    ///
    /// If no offset is configured, the driver state is considered invalid.
    #[maybe_async::maybe_async]
    pub async fn enable_correction_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        if self.read_reg(PCF8523_OFFSET).await? != 0 {
            let mut reg_val = self.read_reg(PCF8523_CONTROL_1).await?;
            set_bits(&mut reg_val, 1, 0, 0b1);
            self.set_clkout(ClkOut::Frequency0Hz).await?;
            self.write_reg(PCF8523_CONTROL_1, reg_val).await
        } else {
            Err(InvalidState)
        }
    }

    /// Enables the day alarm.
    /// - `day` 1..31 (inclusive)
    #[maybe_async::maybe_async]
    pub async fn enable_day_alarm(&mut self, day: u8) -> Result<(), Pcf8523Error<I2C::Error>> {
        if day == 0 || day > 31 {
            return Err(InvalidArgument);
        }
        self.set_clkout(ClkOut::Frequency0Hz).await?;
        self.write_reg(PCF8523_DAY_ALARM, (0 << 7) | encode_bcd(day))
            .await?;
        self.enable_alarm_interrupt().await
    }

    /// Enables the hour alarm.
    /// - `hour` 0..23 (inclusive)
    #[maybe_async::maybe_async]
    pub async fn enable_hour_alarm(&mut self, hour: u8) -> Result<(), Pcf8523Error<I2C::Error>> {
        if hour > 23 {
            return Err(InvalidArgument);
        }
        self.set_clkout(ClkOut::Frequency0Hz).await?;
        self.write_reg(PCF8523_HOUR_ALARM, (0 << 7) | encode_bcd(hour))
            .await?;
        self.enable_alarm_interrupt().await
    }

    /// Enables the minute alarm.
    /// - `minute` 0..59 (inclusive)
    #[maybe_async::maybe_async]
    pub async fn enable_minute_alarm(
        &mut self,
        minute: u8,
    ) -> Result<(), Pcf8523Error<I2C::Error>> {
        if minute > 59 {
            return Err(InvalidArgument);
        }
        self.set_clkout(ClkOut::Frequency0Hz).await?;
        self.write_reg(PCF8523_MINUTE_ALARM, (0 << 7) | encode_bcd(minute))
            .await?;
        self.enable_alarm_interrupt().await
    }

    /// Enables the weekday alarm.
    /// - `weekday` 0..6 (inclusive)
    #[maybe_async::maybe_async]
    pub async fn enable_weekday_alarm(
        &mut self,
        weekday: u8,
    ) -> Result<(), Pcf8523Error<I2C::Error>> {
        if weekday > 6 {
            return Err(InvalidArgument);
        }
        self.set_clkout(ClkOut::Frequency0Hz).await?;
        self.write_reg(PCF8523_WEEKDAY_ALARM, (0 << 7) | weekday)
            .await?;
        self.enable_alarm_interrupt().await
    }

    /// Determines if the module lost power.
    #[maybe_async::maybe_async]
    pub async fn lost_power(&mut self) -> Result<bool, Pcf8523Error<I2C::Error>> {
        let reg_val = self.read_reg(PCF8523_SECONDS).await?;
        Ok((reg_val >> 7) == 1)
    }

    /// Gets the current datetime for the module.
    #[maybe_async::maybe_async]
    pub async fn now(&mut self) -> Result<Pcf8523DateTime, Pcf8523Error<I2C::Error>> {
        let mut second = [0u8];
        let mut minute = [0u8];
        let mut hour = [0u8];
        let mut day = [0u8];
        let mut month = [0u8];
        let mut year = [0u8];
        self.i2c
            .transaction(
                PCF8523_I2C_ADDRESS,
                &mut [
                    Operation::Write(&[PCF8523_SECONDS]),
                    Operation::Read(&mut second),
                    Operation::Write(&[PCF8523_MINUTES]),
                    Operation::Read(&mut minute),
                    Operation::Write(&[PCF8523_HOURS]),
                    Operation::Read(&mut hour),
                    Operation::Write(&[PCF8523_DAYS]),
                    Operation::Read(&mut day),
                    Operation::Write(&[PCF8523_MONTHS]),
                    Operation::Read(&mut month),
                    Operation::Write(&[PCF8523_YEARS]),
                    Operation::Read(&mut year),
                ],
            )
            .await?;
        Ok(Pcf8523DateTime {
            second: second[0],
            minute: minute[0],
            hour: hour[0],
            day: day[0],
            month: month[0],
            year: year[0],
        }
        .bcd_decode())
    }

    /// Reads a value from the register.
    /// - `addr` register address to read from
    #[maybe_async::maybe_async]
    pub async fn read_reg(&mut self, addr: u8) -> Result<u8, Pcf8523Error<I2C::Error>> {
        let mut buffer = [0u8];
        self.i2c
            .write_read(PCF8523_I2C_ADDRESS, &[addr], &mut buffer)
            .await?;
        Ok(buffer[0])
    }

    /// Reloads the Timer A Watchdog counter.
    #[maybe_async::maybe_async]
    pub async fn reload_timer_a_watchdog_countdown(
        &mut self,
        timer: &TimerA,
    ) -> Result<(), Pcf8523Error<I2C::Error>> {
        let tmr_clkout_ctrl = self.read_reg(PCF8523_TMR_CLKOUT_CTRL).await?;
        if timer.mode == Countdown || get_bits(tmr_clkout_ctrl, 2, 1) != 0b10 {
            return Err(InvalidArgument);
        }
        self.write_reg(PCF8523_TMR_A_REG, timer.countdown).await
    }

    /// Performs a software reset.
    #[maybe_async::maybe_async]
    pub async fn reset(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        self.write_reg(PCF8523_CONTROL_1, 0b101_1000).await
    }

    /// Determines if the on-board oscillator is running.
    #[maybe_async::maybe_async]
    pub async fn running(&mut self) -> Result<bool, Pcf8523Error<I2C::Error>> {
        let reg_val = self.read_reg(PCF8523_CONTROL_1).await?;
        Ok(get_bits(reg_val, 1, 5) == 0)
    }

    /// Sets the CLKOUT pin frequency.
    /// - `frequency` Hz to run CLKOUT at
    #[maybe_async::maybe_async]
    pub async fn set_clkout(&mut self, frequency: ClkOut) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut tmr_clkout_ctrl = self.read_reg(PCF8523_TMR_CLKOUT_CTRL).await?;
        set_bits(&mut tmr_clkout_ctrl, frequency as u8, 3, 0b11_1000);
        self.write_reg(PCF8523_TMR_CLKOUT_CTRL, tmr_clkout_ctrl)
            .await
    }

    /// Sets the module datetime in a single I2C transaction to avoid data corruption.
    /// - `datetime` datetime to configure the module for
    #[maybe_async::maybe_async]
    pub async fn set_datetime(
        &mut self,
        datetime: Pcf8523DateTime,
    ) -> Result<(), Pcf8523Error<I2C::Error>> {
        let dt = datetime.encode_bcd();
        self.i2c
            .transaction(
                PCF8523_I2C_ADDRESS,
                &mut [
                    Operation::Write(&[PCF8523_SECONDS, dt.second]),
                    Operation::Write(&[PCF8523_MINUTES, dt.minute]),
                    Operation::Write(&[PCF8523_HOURS, dt.hour]),
                    Operation::Write(&[PCF8523_DAYS, dt.day]),
                    Operation::Write(&[PCF8523_MONTHS, dt.month]),
                    Operation::Write(&[PCF8523_YEARS, dt.year]),
                ],
            )
            .await?;
        Ok(())
    }

    /// Sets the module's power management functions.
    /// - `power_management` selected power management option for the module
    #[maybe_async::maybe_async]
    pub async fn set_power_management(
        &mut self,
        power_management: PowerManagement,
    ) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_3).await?;
        set_bits(&mut reg_val, power_management as u8, 5, 0b1110_0000);
        self.write_reg(PCF8523_CONTROL_3, reg_val).await
    }

    /// Calibrate for aging adjustment, temperature compensation and accuracy tuning.
    /// - `mode` specifies the frequency of correction application
    /// - `offset` -64..63 (inclusive) correction amount applied via `mode` timing
    #[maybe_async::maybe_async]
    pub async fn set_offset(
        &mut self,
        mode: CorrectionMode,
        offset: i8,
    ) -> Result<(), Pcf8523Error<I2C::Error>> {
        if offset < -64 || offset > 63 {
            return Err(InvalidArgument);
        }
        let mut reg_val = (mode as u8) << 7;
        set_bits(&mut reg_val, offset as u8, 0, 0b111_1111);
        self.write_reg(PCF8523_OFFSET, reg_val).await
    }

    /// Starts the RTC time circuits.
    #[maybe_async::maybe_async]
    pub async fn start(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1).await?;
        set_bits(&mut reg_val, 0, 5, 0b10_0000);
        self.write_reg(PCF8523_CONTROL_1, reg_val).await
    }

    /// Starts the second timer and enables the interrupt.
    /// - `interrupt_mode` configures the interrupt as pulsed or permanently active
    #[maybe_async::maybe_async]
    pub async fn start_second_timer(
        &mut self,
        interrupt_mode: TimerInterruptMode,
    ) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut clkout_ctrl = self.read_reg(PCF8523_TMR_CLKOUT_CTRL).await?;
        // set interrupt mode
        set_bits(&mut clkout_ctrl, interrupt_mode as u8, 7, 0b1000_0000);
        // disable CLKOUT
        set_bits(&mut clkout_ctrl, 0b111, 3, 0b11_1000);
        self.write_reg(PCF8523_TMR_CLKOUT_CTRL, clkout_ctrl).await?;

        // enable interrupt
        let mut control_1 = self.read_reg(PCF8523_CONTROL_1).await?;
        set_bits(&mut control_1, 1, 2, 0b100); // SIE
        self.write_reg(PCF8523_CONTROL_1, control_1).await
    }

    /// Starts Timer A and enables the interrupt.
    /// - `timer` TimerA configuration
    #[maybe_async::maybe_async]
    pub async fn start_timer_a(&mut self, timer: &TimerA) -> Result<(), Pcf8523Error<I2C::Error>> {
        if timer.countdown == 0 {
            return Err(InvalidTimerCountdown);
        }

        self.stop_timer_a().await?;

        // set source clock frequency
        self.write_reg(PCF8523_TMR_A_FREQ_CTRL, timer.source_clock as u8)
            .await?;

        let mut tmr_clkout_ctrl = self.read_reg(PCF8523_TMR_CLKOUT_CTRL).await?;
        // disable CLKOUT
        set_bits(&mut tmr_clkout_ctrl, Frequency0Hz as u8, 3, 0b11_1000);

        // set interrupt mode
        set_bits(
            &mut tmr_clkout_ctrl,
            timer.interrupt_mode.into(),
            7,
            0b1000_0000,
        );

        // enable timer mode
        set_bits(&mut tmr_clkout_ctrl, timer.mode.into(), 1, 0b110);
        self.write_reg(PCF8523_TMR_CLKOUT_CTRL, tmr_clkout_ctrl)
            .await?;

        // enable interrupt for timer mode
        let mut control_2 = self.read_reg(PCF8523_CONTROL_2).await?;
        match timer.mode {
            Countdown => set_bits(&mut control_2, 0b01, 1, 0b110),
            Watchdog => set_bits(&mut control_2, 0b10, 1, 0b110),
        }
        self.write_reg(PCF8523_CONTROL_2, control_2).await?;

        // start the timer by setting a countdown value
        self.write_reg(PCF8523_TMR_A_REG, timer.countdown).await
    }

    /// Stops the RTC time circuits.
    #[maybe_async::maybe_async]
    pub async fn stop(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_1).await?;
        set_bits(&mut reg_val, 1, 5, 0b10_0000);
        self.write_reg(PCF8523_CONTROL_1, reg_val).await
    }

    /// Stops CLKOUT.
    #[maybe_async::maybe_async]
    pub async fn stop_clkout(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_TMR_CLKOUT_CTRL).await?;
        set_bits(&mut reg_val, 0b111, 3, 0b11_1000); // COF
        self.write_reg(PCF8523_TMR_CLKOUT_CTRL, reg_val).await
    }

    /// Stops the second timer.
    #[maybe_async::maybe_async]
    pub async fn stop_second_timer(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut control_1 = self.read_reg(PCF8523_CONTROL_1).await?;
        set_bits(&mut control_1, 0, 2, 0b100); // SIE
        self.write_reg(PCF8523_CONTROL_1, control_1).await
    }

    /// Stops Timer A.
    #[maybe_async::maybe_async]
    pub async fn stop_timer_a(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut tmr_clkout_ctrl = self.read_reg(PCF8523_TMR_CLKOUT_CTRL).await?;
        set_bits(&mut tmr_clkout_ctrl, 00, 1, 0b110);
        self.write_reg(PCF8523_TMR_CLKOUT_CTRL, tmr_clkout_ctrl)
            .await
    }

    /// Gets the Timer A counter. This is the current value, and not the TimerA.countdown value that
    /// was initially configured. In watchdog mode, this value represents the timer period and will
    /// be constant until Timer A is reconfigured. In countdown mode, this value is dynamic in
    /// relation to passing time. Since the timer cannot be frozen during the read, it's read
    /// twice and compared for equality.
    #[maybe_async::maybe_async]
    pub async fn timer_a_counter(&mut self) -> Result<u8, Pcf8523Error<I2C::Error>> {
        self.timer_counter(PCF8523_TMR_A_REG).await
    }

    #[maybe_async::maybe_async]
    async fn timer_counter(&mut self, reg_addr: u8) -> Result<u8, Pcf8523Error<I2C::Error>> {
        let a = self.read_reg(reg_addr).await?;
        let b = self.read_reg(reg_addr).await?;
        if a == b {
            Ok(a)
        } else {
            Err(Pcf8523Error::InconsistentTimerCounter)
        }
    }

    /// Writes a value to a register.
    /// - `reg` register address
    /// - `val` value to write
    #[maybe_async::maybe_async]
    pub async fn write_reg(&mut self, reg: u8, val: u8) -> Result<(), Pcf8523Error<I2C::Error>> {
        Ok(self.i2c.write(PCF8523_I2C_ADDRESS, &[reg, val]).await?)
    }
}

impl<I2C: I2c, V: Variant + Int2Pin> Pcf8523<I2C, V> {
    /// Clears the Timer B interrupt.
    #[maybe_async::maybe_async]
    pub async fn clear_timer_b_interrupt(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_CONTROL_2).await?;
        set_bits(&mut reg_val, 0b0_1011, 3, 0b1111_1000);
        self.write_reg(PCF8523_CONTROL_2, reg_val).await
    }

    /// Starts Timer B and enables the interrupt.
    /// - `timer` TimerB configuration
    #[maybe_async::maybe_async]
    pub async fn start_timer_b(&mut self, timer: &TimerB) -> Result<(), Pcf8523Error<I2C::Error>> {
        self.stop_timer_b().await?;

        let mut tmr_b_freq_ctrl = self.read_reg(PCF8523_TMR_B_FREQ_CTRL).await?;
        // set timer frequency
        set_bits(&mut tmr_b_freq_ctrl, timer.source_clock as u8, 0, 0b111);

        match timer.interrupt_mode {
            // if pulsed interrupt, set low pulse width
            TimerBInterruptMode::Pulsed(width) => {
                set_bits(&mut tmr_b_freq_ctrl, width as u8, 4, 0b111_0000);
            }
            _ => {}
        }
        self.write_reg(PCF8523_TMR_B_FREQ_CTRL, tmr_b_freq_ctrl)
            .await?;

        let mut control_2 = self.read_reg(PCF8523_CONTROL_2).await?;
        // enable interrupt
        set_bits(&mut control_2, 1, 0, 0b1);
        self.write_reg(PCF8523_CONTROL_2, control_2).await?;

        let mut tmr_clkout_ctrl = self.read_reg(PCF8523_TMR_CLKOUT_CTRL).await?;
        // disable CLKOUT
        set_bits(&mut tmr_clkout_ctrl, Frequency0Hz as u8, 3, 0b11_1000);
        // set interrupt mode
        set_bits(
            &mut tmr_clkout_ctrl,
            timer.interrupt_mode.into(),
            6,
            0b100_0000,
        );

        // enable timer
        set_bits(&mut tmr_clkout_ctrl, 1, 0, 0b1);
        self.write_reg(PCF8523_TMR_CLKOUT_CTRL, tmr_clkout_ctrl)
            .await?;

        // start the timer by setting a countdown value
        self.write_reg(PCF8523_TMR_B_REG, timer.countdown).await
    }

    /// Stops Timer B.
    #[maybe_async::maybe_async]
    pub async fn stop_timer_b(&mut self) -> Result<(), Pcf8523Error<I2C::Error>> {
        let mut reg_val = self.read_reg(PCF8523_TMR_CLKOUT_CTRL).await?;
        set_bits(&mut reg_val, 0, 0, 0b1);
        self.write_reg(PCF8523_TMR_CLKOUT_CTRL, reg_val).await
    }

    /// Gets the Timer B counter. This is the current value, and not the TimerB.countdown value that
    /// was initially configured. This value is dynamic in relation to passing timeSince the timer
    /// cannot be frozen during the read, it's read twice and compared for equality.
    #[maybe_async::maybe_async]
    pub async fn timer_b_counter(&mut self) -> Result<u8, Pcf8523Error<I2C::Error>> {
        self.timer_counter(PCF8523_TMR_B_REG).await
    }
}
