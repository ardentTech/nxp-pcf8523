use crate::bits::{decode_bcd, encode_bcd};

const DAYS_PER_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const SECONDS_FROM_1970_TO_2000: u32 = 946684800;

/// Simple datetime that supports 01.01.2000 to 12.31.2099 (inclusive).
#[derive(Debug)]
pub struct Pcf8523DateTime {
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub year: u8,
}

impl Pcf8523DateTime {
    /// Constructs a new datetime instance.
    /// - `hour` 0..23 (inclusive)
    /// - `minute` 0..59 (inclusive)
    /// - `second` 0..59 (inclusive)
    /// - `month` 1..12 (inclusive)
    /// - `day` 1..31 (inclusive) is validated against month and year
    /// - `year` 0..99 (inclusive) offset from the year 2000
    pub fn new(
        hours: u8,
        minutes: u8,
        seconds: u8,
        month: u8,
        day: u8,
        year: u8,
    ) -> Option<Pcf8523DateTime> {
        if !Self::validate_time(seconds, minutes, hours) || !Self::validate_date(day, month, year) {
            return None;
        }
        Some(Self {
            second: seconds,
            minute: minutes,
            hour: hours,
            day,
            month,
            year,
        })
    }

    pub(crate) fn bcd_decode(&self) -> Self {
        Self {
            second: decode_bcd(self.second),
            minute: decode_bcd(self.minute),
            hour: decode_bcd(self.hour),
            day: decode_bcd(self.day),
            month: decode_bcd(self.month),
            year: decode_bcd(self.year),
        }
    }

    pub(crate) fn encode_bcd(&self) -> Self {
        Self {
            second: encode_bcd(self.second),
            minute: encode_bcd(self.minute),
            hour: encode_bcd(self.hour),
            day: encode_bcd(self.day),
            month: encode_bcd(self.month),
            year: encode_bcd(self.year),
        }
    }

    /// Generates the Unix timestamp representation of the datetime.
    pub fn timestamp(&self) -> u32 {
        SECONDS_FROM_1970_TO_2000
            + ((self.days_since_2000() * 24 + (self.hour as u32)) * 60 + (self.minute as u32)) * 60
            + (self.second as u32)
    }

    fn days_since_2000(&self) -> u32 {
        let mut days: u32 = (0u32..(self.year as u32)).fold((self.day - 1) as u32, |acc, e| {
            acc + if e % 4 != 0 { 365 } else { 366 }
        });
        for m in 0..(self.month - 1) {
            days += DAYS_PER_MONTH[m as usize] as u32;
        }
        if self.month > 1 && self.year % 4 == 0 {
            days += 1;
        }
        days
    }

    const fn validate_date(day: u8, month: u8, year: u8) -> bool {
        if day == 0 || month == 0 || month > 12 || year > 99 {
            false
        } else if month == 2 && (year == 0 || year % 4 == 0) {
            (DAYS_PER_MONTH[month as usize - 1] + 1) >= day
        } else {
            DAYS_PER_MONTH[month as usize - 1] >= day
        }
    }

    const fn validate_time(seconds: u8, minutes: u8, hours: u8) -> bool {
        seconds < 60 && minutes < 60 && hours < 24
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_floor_ok() {
        let dt = Pcf8523DateTime::new(0, 0, 0, 1, 1, 0).unwrap();
        assert_eq!(dt.timestamp(), SECONDS_FROM_1970_TO_2000);
    }

    #[test]
    fn timestamp_one_leap_year_ok() {
        let dt = Pcf8523DateTime::new(0, 0, 0, 3, 1, 1).unwrap();
        assert_eq!(dt.timestamp(), 983404800);
    }

    #[test]
    fn timestamp_multiple_leap_years_ok() {
        let dt = Pcf8523DateTime::new(15, 23, 11, 8, 21, 25).unwrap();
        assert_eq!(dt.timestamp(), 1755789791);
    }

    #[test]
    fn timestamp_year_end_ok() {
        let dt = Pcf8523DateTime::new(23, 59, 59, 12, 31, 24).unwrap();
        assert_eq!(dt.timestamp(), 1735689599);
    }

    #[test]
    fn validate_date_day_above_ceiling() {
        assert!(!Pcf8523DateTime::validate_date(32, 1, 99))
    }

    #[test]
    fn validate_date_day_above_month_ceiling() {
        assert!(!Pcf8523DateTime::validate_date(31, 4, 99))
    }

    #[test]
    fn validate_date_day_below_floor() {
        assert!(!Pcf8523DateTime::validate_date(0, 1, 99))
    }

    #[test]
    fn validate_date_month_above_ceiling() {
        assert!(!Pcf8523DateTime::validate_date(31, 13, 99))
    }

    #[test]
    fn validate_date_month_below_floor() {
        assert!(!Pcf8523DateTime::validate_date(1, 0, 99))
    }

    #[test]
    fn validate_date_year_above_ceiling() {
        assert!(!Pcf8523DateTime::validate_date(1, 1, 100))
    }

    #[test]
    fn validate_date_february_leap_year_invalid() {
        assert!(!Pcf8523DateTime::validate_date(29, 2, 1))
    }

    #[test]
    fn validate_date_february_leap_year_ok() {
        assert!(Pcf8523DateTime::validate_date(29, 2, 0))
    }

    #[test]
    fn validate_date_february_non_leap_year_ok() {
        assert!(Pcf8523DateTime::validate_date(28, 2, 3))
    }

    #[test]
    fn validate_time_seconds_above_ceiling() {
        assert!(!Pcf8523DateTime::validate_time(60, 2, 0))
    }

    #[test]
    fn validate_time_minutes_above_ceiling() {
        assert!(!Pcf8523DateTime::validate_time(30, 60, 0))
    }

    #[test]
    fn validate_time_hours_above_ceiling() {
        assert!(!Pcf8523DateTime::validate_time(30, 42, 24))
    }
}
