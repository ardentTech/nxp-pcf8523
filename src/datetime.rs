use crate::bits::{decode_bcd, encode_bcd};

#[derive(Debug, PartialEq)]
pub enum DayOfWeek {
    Sunday = 0x0,
    Monday = 0x1,
    Tuesday = 0x2,
    Wednesday = 0x3,
    Thursday = 0x4,
    Friday = 0x5,
    Saturday = 0x6,
}
impl TryFrom<u8> for DayOfWeek {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(DayOfWeek::Sunday),
            0x1 => Ok(DayOfWeek::Monday),
            0x2 => Ok(DayOfWeek::Tuesday),
            0x3 => Ok(DayOfWeek::Wednesday),
            0x4 => Ok(DayOfWeek::Thursday),
            0x5 => Ok(DayOfWeek::Friday),
            0x6 => Ok(DayOfWeek::Saturday),
            _ => Err(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Month {
    January = 0x1,
    February = 0x2,
    March = 0x3,
    April = 0x4,
    May = 0x5,
    June = 0x6,
    July = 0x7,
    August = 0x8,
    September = 0x9,
    October = 0xa,
    November = 0xb,
    December = 0xc,
}
impl TryFrom<u8> for Month {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Month::January),
            0x2 => Ok(Month::February),
            0x3 => Ok(Month::March),
            0x4 => Ok(Month::April),
            0x5 => Ok(Month::May),
            0x6 => Ok(Month::June),
            0x7 => Ok(Month::July),
            0x8 => Ok(Month::August),
            0x9 => Ok(Month::September),
            0xa => Ok(Month::October),
            0xb => Ok(Month::November),
            0xc => Ok(Month::December),
            _ => Err(())
        }
    }
}

// TODO might want an explicit BCD-encoded version?
#[derive(Debug)]
pub struct DateTime {
    pub seconds: u8,
    pub(crate) minutes: u8,
    pub(crate) hours: u8,
    pub(crate) day: u8,
    pub(crate) day_of_week: u8,
    pub(crate) month: u8,
    pub(crate) year: u8,
}
impl Default for DateTime {
    fn default() -> Self {
        Self {
            seconds: 0,
            minutes: 0,
            hours: 0,
            day: 0,
            day_of_week: 0,
            month: 1,
            year: 0
        }
    }
}

impl DateTime {
    pub fn new(
        seconds: u8,
        minutes: u8,
        hours: u8,
        day: u8,
        day_of_week: DayOfWeek,
        month: Month,
        year: u8,
    ) -> Option<Self> {
        // TODO more useful errors in Result<> instead of Option<>?
        if seconds > 59 { return None; }
        if minutes > 59 { return None; }
        if hours > 23 { return None; }
        if day > 31 { return None; }
        if year > 99 { return None; }

        Some(Self {
            seconds,
            minutes,
            hours,
            day,
            day_of_week: day_of_week as u8,
            month: month as u8,
            year,
        })
    }

    pub(crate) fn decode(&self) -> Self {
        Self {
            seconds: decode_bcd(self.seconds),
            minutes: decode_bcd(self.minutes),
            hours: decode_bcd(self.hours),
            day: decode_bcd(self.day),
            day_of_week: decode_bcd(self.day_of_week),
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
            day_of_week: encode_bcd(self.day_of_week),
            month: encode_bcd(self.month),
            year: encode_bcd(self.year),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_invalid_seconds_none() {}

    #[test]
    fn test_new_invalid_minutes_none() {}

    #[test]
    fn test_new_invalid_hours_none() {}

    #[test]
    fn test_new_invalid_day_none() {}

    #[test]
    fn test_new_invalid_year_none() {}

    #[test]
    fn test_new_some() {
        // validate bcd-encoded
    }
}