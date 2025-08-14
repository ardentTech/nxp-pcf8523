use crate::bits::encode_bcd;

#[derive(Debug, PartialEq)]
pub struct DateTime {
    pub(crate) seconds: u8,
    pub(crate) minutes: u8,
    pub(crate) hours: u8,
    pub(crate) day: u8,
    pub(crate) day_of_week: u8,
    pub(crate) month: u8,
    pub(crate) year: u8,
}
impl DateTime {
    pub fn new(
        seconds: u8,
        minutes: u8,
        hours: u8,
        day: u8,
        day_of_week: u8,
        month: u8,
        year: u8,
    ) -> Option<Self> {
        // TODO validation here
        Some(Self {
            seconds: encode_bcd(seconds),
            minutes: encode_bcd(minutes),
            hours: encode_bcd(hours),
            day: encode_bcd(day),
            day_of_week: encode_bcd(day_of_week),
            month: encode_bcd(month),
            year: encode_bcd(year),
        })
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
    fn test_new_invalid_day_of_week_none() {}

    #[test]
    fn test_new_invalid_month_none() {}

    #[test]
    fn test_new_invalid_year_none() {}

    #[test]
    fn test_new_some() {
        // validate bcd-encoded
    }
}