//! Calendar date parsing for the `merged` field, without a date-time dependency.

use std::fmt;

const MONTH_ABBREVIATIONS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// A calendar date. Field order makes the derived `Ord` chronological.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Date {
    /// Parses a strict `YYYY-MM-DD` date and rejects days that do not exist.
    pub fn parse_iso(text: &str) -> Result<Self, String> {
        let invalid = || format!("expected a YYYY-MM-DD date, got {text:?}");
        let mut parts = text.split('-');
        let (Some(year), Some(month), Some(day), None) =
            (parts.next(), parts.next(), parts.next(), parts.next())
        else {
            return Err(invalid());
        };
        if year.len() != 4 || month.len() != 2 || day.len() != 2 {
            return Err(invalid());
        }
        let all_digits = |part: &str| part.bytes().all(|byte| byte.is_ascii_digit());
        if !(all_digits(year) && all_digits(month) && all_digits(day)) {
            return Err(invalid());
        }
        let year: u16 = year.parse().map_err(|_| invalid())?;
        let month: u8 = month.parse().map_err(|_| invalid())?;
        let day: u8 = day.parse().map_err(|_| invalid())?;

        if !(1..=12).contains(&month) {
            return Err(format!("month {month} is out of range in {text:?}"));
        }
        if day == 0 || day > days_in_month(year, month) {
            return Err(format!("day {day} does not exist in {text:?}"));
        }
        Ok(Self { year, month, day })
    }

    /// Human-readable form, e.g. `Sep 25, 2026`.
    pub fn display_long(&self) -> String {
        let month = MONTH_ABBREVIATIONS[usize::from(self.month) - 1];
        format!("{month} {}, {}", self.day, self.year)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

fn is_leap_year(year: u16) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_two_digit_day() {
        let date = Date::parse_iso("2026-09-25").unwrap();
        assert_eq!(date.display_long(), "Sep 25, 2026");
    }

    #[test]
    fn formats_single_digit_day_without_leading_zero() {
        let date = Date::parse_iso("2026-09-07").unwrap();
        assert_eq!(date.display_long(), "Sep 7, 2026");
    }

    #[test]
    fn display_round_trips_the_iso_form() {
        assert_eq!(
            Date::parse_iso("2026-01-02").unwrap().to_string(),
            "2026-01-02"
        );
    }

    #[test]
    fn rejects_text_that_is_not_a_date() {
        for bad in [
            "not-a-date",
            "",
            "2026-1-02",
            "2026-01-2",
            "20260102",
            "2026-01-02-03",
            "+026-01-02",
        ] {
            assert!(Date::parse_iso(bad).is_err(), "{bad:?} should be rejected");
        }
    }

    #[test]
    fn rejects_out_of_range_month() {
        assert!(Date::parse_iso("2026-13-01").is_err());
        assert!(Date::parse_iso("2026-00-01").is_err());
    }

    #[test]
    fn rejects_days_that_do_not_exist() {
        assert!(Date::parse_iso("2026-02-30").is_err());
        assert!(Date::parse_iso("2026-04-31").is_err());
        assert!(Date::parse_iso("2026-01-00").is_err());
    }

    #[test]
    fn leap_day_is_valid_only_in_leap_years() {
        assert!(Date::parse_iso("2024-02-29").is_ok());
        assert!(Date::parse_iso("2026-02-29").is_err());
        assert!(Date::parse_iso("1900-02-29").is_err());
        assert!(Date::parse_iso("2000-02-29").is_ok());
    }

    #[test]
    fn orders_chronologically() {
        let earlier = Date::parse_iso("2025-12-31").unwrap();
        let later = Date::parse_iso("2026-01-01").unwrap();
        assert!(earlier < later);
    }
}
