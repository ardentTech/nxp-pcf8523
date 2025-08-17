use crate::bits::{decode_bcd, encode_bcd};

const DAYS_PER_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

pub struct Pcf8523DateTime {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
    pub day: u8,
    pub month: u8,
    pub year: u8,
}

impl Pcf8523DateTime {
    pub fn new(seconds: u8, minutes: u8, hours: u8, day: u8, month: u8, year: u8) -> Option<Pcf8523DateTime> {
        if !Self::validate_time(seconds, minutes, hours) || !Self::validate_date(day, month, year) {
            return None;
        }
        Some(Self { seconds, minutes, hours, day, month, year })
    }

    pub(crate) fn decode(&self) -> Self {
        Self {
            seconds: decode_bcd(self.seconds),
            minutes: decode_bcd(self.minutes),
            hours: decode_bcd(self.hours),
            day: decode_bcd(self.day),
            month: decode_bcd(self.month),
            year: decode_bcd(self.year),
        }
    }

    pub(crate) fn encode(&self) -> Self {
        Self {
            seconds: encode_bcd(self.seconds),
            minutes: encode_bcd(self.minutes),
            hours: encode_bcd(self.hours),
            day: encode_bcd(self.day),
            month: encode_bcd(self.month),
            year: encode_bcd(self.year),
        }
    }

    fn validate_date(day: u8, month: u8, year: u8) -> bool {
        if day == 0 || month == 0 || month > 12 || year > 99 {
            false
        } else if month == 2 && (year == 0 || year % 4 == 0) {
            (DAYS_PER_MONTH[month as usize - 1] + 1) >= day
        } else {
            DAYS_PER_MONTH[month as usize - 1] >= day
        }
    }

    fn validate_time(seconds: u8, minutes: u8, hours: u8) -> bool {
        seconds < 60 && minutes < 60 && hours < 24
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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