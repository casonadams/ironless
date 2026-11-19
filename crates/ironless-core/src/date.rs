//! Calendar days, written `YYYY-MM-DD`: the app's local day, not an instant.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

const WEEKDAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// A calendar day, kept as days since 1970-01-01.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Date {
    days: i64,
}

/// Text that is not a real `YYYY-MM-DD` day.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidDate(pub String);

impl fmt::Display for InvalidDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not a YYYY-MM-DD date: {:?}", self.0)
    }
}

impl std::error::Error for InvalidDate {}

impl Date {
    /// The day `year-month-day`, or `None` for a day the calendar does not have.
    pub fn from_ymd(year: i64, month: u32, day: u32) -> Option<Self> {
        let date = Date {
            days: days_from_civil(year, i64::from(month), i64::from(day)),
        };
        (date.ymd() == (year, month, day)).then_some(date)
    }

    /// The Monday of this day's week.
    pub fn week_start(self) -> Date {
        Date {
            days: self.days - self.weekday_index(),
        }
    }

    /// The day `count` days later (earlier when negative).
    pub fn plus_days(self, count: i64) -> Date {
        Date {
            days: self.days + count,
        }
    }

    /// A short label such as `Tue, Nov 18`.
    pub fn label(self) -> String {
        let (_, month, day) = self.ymd();
        let weekday = WEEKDAYS[usize::try_from(self.weekday_index()).unwrap_or(0)];
        let month = MONTHS[usize::try_from(month - 1).unwrap_or(0)];
        format!("{weekday}, {month} {day}")
    }

    /// 0 for Monday through 6 for Sunday (1970-01-01 was a Thursday).
    fn weekday_index(self) -> i64 {
        (self.days + 3).rem_euclid(7)
    }

    fn ymd(self) -> (i64, u32, u32) {
        civil_from_days(self.days)
    }
}

/// Howard Hinnant's `days_from_civil`: days since 1970-01-01 in the proleptic Gregorian calendar.
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let day_of_year = (153 * ((month + 9) % 12) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// Howard Hinnant's `civil_from_days`, the inverse of [`days_from_civil`].
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let days = days + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    let month = if shifted_month < 10 {
        shifted_month + 3
    } else {
        shifted_month - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    (year, small(month), small(day))
}

fn small(value: i64) -> u32 {
    u32::try_from(value).expect("months and days are small and positive")
}

impl FromStr for Date {
    type Err = InvalidDate;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let invalid = || InvalidDate(text.to_owned());
        let mut parts = text.splitn(3, '-');
        let mut next = || parts.next().and_then(|part| part.parse::<u32>().ok());
        let (year, month, day) = (next(), next(), next());
        let (year, month, day) = (
            year.ok_or_else(invalid)?,
            month.ok_or_else(invalid)?,
            day.ok_or_else(invalid)?,
        );
        Date::from_ymd(i64::from(year), month, day).ok_or_else(invalid)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (year, month, day) = self.ymd();
        write!(f, "{year:04}-{month:02}-{day:02}")
    }
}

impl TryFrom<String> for Date {
    type Error = InvalidDate;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        text.parse()
    }
}

impl From<Date> for String {
    fn from(date: Date) -> Self {
        date.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(text: &str) -> Date {
        text.parse().expect("a valid date")
    }

    #[test]
    fn parses_and_writes_real_days() {
        for text in [
            "1970-01-01",
            "2024-02-29",
            "2026-11-18",
            "1999-12-31",
            "2000-02-29",
        ] {
            assert_eq!(date(text).to_string(), text);
        }
    }

    #[test]
    fn rejects_days_the_calendar_does_not_have() {
        for text in [
            "2025-02-29",
            "2026-13-01",
            "2026-04-31",
            "2026-00-10",
            "18-11",
            "",
            "x-y-z",
        ] {
            assert!(text.parse::<Date>().is_err(), "{text} parsed");
        }
    }

    #[test]
    fn weeks_start_on_monday() {
        let cases = [
            ("2026-11-16", "2026-11-16"),
            ("2026-11-18", "2026-11-16"),
            ("2026-11-22", "2026-11-16"),
            ("2026-11-23", "2026-11-23"),
            ("2026-01-01", "2025-12-29"),
        ];
        for (day, monday) in cases {
            assert_eq!(date(day).week_start(), date(monday), "{day}");
        }
    }

    #[test]
    fn labels_name_the_weekday_and_month() {
        assert_eq!(date("2026-11-18").label(), "Wed, Nov 18");
        assert_eq!(date("2027-01-03").label(), "Sun, Jan 3");
    }

    #[test]
    fn adds_days_across_months() {
        assert_eq!(date("2026-11-28").plus_days(7), date("2026-12-05"));
        assert_eq!(date("2026-03-01").plus_days(-1), date("2026-02-28"));
    }

    #[test]
    fn serializes_as_text() {
        let json = serde_json::to_string(&date("2026-11-18")).expect("serializes");
        assert_eq!(json, "\"2026-11-18\"");
        assert!(serde_json::from_str::<Date>("\"2026-02-30\"").is_err());
    }
}
