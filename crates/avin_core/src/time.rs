// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;
use std::str::FromStr;

use chrono::{DateTime, NaiveDate, NaiveDateTime, Timelike, Utc};

use crate::CoreError;

/// Canonical UTC time representation used across AVIN.
///
/// Internally, it stores a Unix timestamp in nanoseconds as an `i64`.
///
/// Supported times range from `1677-09-21 00:12:43` to `2262-04-11 23:47:16`.
///
/// **IMPORTANT!!!**
/// Human-readable string parsing always interprets input as UTC.
/// Local time zones are not inferred or applied automatically.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use chrono::{Utc, TimeZone};
///
/// use avin_core::Time;
///
/// let time = Time::from_str("2026-01-01 12:55:03").unwrap();
/// assert_eq!(time.to_string(), "2026-01-01 12:55:03");
///
/// let dt = Utc.with_ymd_and_hms(2026, 01, 01, 12, 55, 3).unwrap();
/// assert_eq!(time.dt(), dt);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time(i64);

impl Time {
    /// Creates a new Time from a Unix timestamp in nanoseconds.
    pub fn new(timestamp_nanos: i64) -> Self {
        Time(timestamp_nanos)
    }

    /// Returns the current UTC time.
    pub fn now() -> Time {
        Time::new(Utc::now().timestamp_nanos_opt().unwrap())
    }

    /// Returns the time as a UTC [`DateTime`].
    pub fn dt(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_nanos(self.0)
    }
    /// Returns the Unix timestamp in nanoseconds.
    pub fn ts(&self) -> i64 {
        self.0
    }
}

impl Display for Time {
    /// Formats the time in a compact human-readable UTC representation.
    ///
    /// Subsecond precision is omitted. Zero-valued time components are omitted
    /// when they are not needed to represent the value.
    ///
    /// Formatting examples:
    /// 2026-01-01 15:05:01.85468   -> "2026-01-01 15:05:01"
    /// 2026-01-01 15:05:01         -> "2026-01-01 15:05:01"
    /// 2026-01-01 15:05:00         -> "2026-01-01 15:05"
    /// 2026-01-01 15:00:00         -> "2026-01-01 15:00"
    /// 2026-01-01 00:00:00         -> "2026-01-01"
    /// 2026-01-01 00:00:00.0001    -> "2026-01-01 00:00:00"
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt = self.dt();

        // has nanoseconds or seconds
        if dt.nanosecond() != 0 || dt.second() != 0 {
            write!(f, "{}", dt.format("%Y-%m-%d %H:%M:%S"))
        }
        // has hours or minutes only
        else if dt.hour() != 0 || dt.minute() != 0 {
            write!(f, "{}", dt.format("%Y-%m-%d %H:%M"))
        }
        // has date only
        else {
            write!(f, "{}", dt.format("%Y-%m-%d"))
        }
    }
}

impl FromStr for Time {
    type Err = CoreError;

    /// Parses a UTC time from a human-readable string.
    ///
    /// Supported formats:
    ///
    /// ```text
    /// 2026-01-01 12:55:00
    /// 2026-01-01 12:55
    /// 2026-01-01
    /// ```
    ///
    /// **IMPORTANT!!!**
    /// All parsed values are interpreted as UTC.
    ///
    /// # Errors
    ///
    /// Returns an error if the string has an unsupported format or if the
    /// resulting time is outside the nanosecond range supported by [`Time`].
    ///
    /// Supported times range from `1677-09-21 00:12:43` to
    /// `2262-04-11 23:47:16`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use avin_core::Time;
    ///
    /// assert!(Time::from_str("2026-01-01 12:55:00").is_ok());
    /// assert!(Time::from_str("2026-01-01 12:55").is_ok());
    /// assert!(Time::from_str("2026-01-01").is_ok());
    ///
    /// assert!(Time::from_str("2026/01/01").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dt = parse_dt_str(s)?;

        let ts = dt.timestamp_nanos_opt().ok_or_else(|| {
            let begin = Time::new(i64::MIN);
            let end = Time::new(i64::MAX);
            CoreError::Time(format!(
                "time {s} is outside supported range [{begin}, {end}]"
            ))
        })?;

        Ok(Time::new(ts))
    }
}

fn parse_dt_str(s: &str) -> Result<DateTime<Utc>, CoreError> {
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
        return Ok(dt.and_utc());
    }

    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M") {
        return Ok(dt.and_utc());
    }

    if let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        let dt = NaiveDateTime::from(date);
        return Ok(dt.and_utc());
    }

    Err(CoreError::Time(format!("invalid time format: {s}")))
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn new() {
        let time = Time::new(123456789);

        assert_eq!(time.ts(), 123456789);
    }

    #[test]
    fn now() {
        let before = Utc::now().timestamp_nanos_opt().unwrap();
        let now = Time::now().ts();
        let after = Utc::now().timestamp_nanos_opt().unwrap();

        assert!(before <= now);
        assert!(now <= after);
    }

    #[test]
    fn datetime() {
        let time = Time::from_str("2026-01-01 12:55:00").unwrap();
        let expected = Utc.with_ymd_and_hms(2026, 1, 1, 12, 55, 0).unwrap();

        assert_eq!(time.dt(), expected);
    }

    #[test]
    fn from_str_seconds() {
        let time = Time::from_str("2026-01-01 12:55:01").unwrap();
        let expected = Utc.with_ymd_and_hms(2026, 1, 1, 12, 55, 1).unwrap();

        assert_eq!(time.dt(), expected);
    }

    #[test]
    fn from_str_minutes() {
        let time = Time::from_str("2026-01-01 12:55").unwrap();
        let expected = Utc.with_ymd_and_hms(2026, 1, 1, 12, 55, 0).unwrap();

        assert_eq!(time.dt(), expected);
    }

    #[test]
    fn from_str_date() {
        let time = Time::from_str("2026-01-01").unwrap();
        let expected = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();

        assert_eq!(time.dt(), expected);
    }

    #[test]
    fn from_str_invalid_format() {
        assert!(matches!(
            Time::from_str("2026/01/01").unwrap_err(),
            CoreError::Time(_)
        ));
    }

    #[test]
    fn from_str_invalid_date() {
        assert!(matches!(
            Time::from_str("2026-02-30").unwrap_err(),
            CoreError::Time(_)
        ));
    }

    #[test]
    fn from_str_outside_supported_range() {
        assert!(matches!(
            Time::from_str("1600-01-01").unwrap_err(),
            CoreError::Time(_)
        ));

        assert!(matches!(
            Time::from_str("2300-01-01").unwrap_err(),
            CoreError::Time(_)
        ));
    }

    #[test]
    fn display_seconds() {
        let time = Time::from_str("2026-01-01 15:05:01").unwrap();

        assert_eq!(time.to_string(), "2026-01-01 15:05:01");
    }

    #[test]
    fn display_minutes() {
        let time = Time::from_str("2026-01-01 15:05:00").unwrap();

        assert_eq!(time.to_string(), "2026-01-01 15:05");

        let time = Time::from_str("2026-01-01 15:00:00").unwrap();

        assert_eq!(time.to_string(), "2026-01-01 15:00");
    }

    #[test]
    fn display_date() {
        let time = Time::from_str("2026-01-01 00:00:00").unwrap();

        assert_eq!(time.to_string(), "2026-01-01");
    }

    #[test]
    fn display_subseconds() {
        // when subseconds == 0
        let time = Time::from_str("2026-01-01 15:00:00").unwrap();

        assert_eq!(time.to_string(), "2026-01-01 15:00");

        // show seconds when subseconds != 0
        let time = Time::new(time.ts() + 854_680_000);

        assert_eq!(time.to_string(), "2026-01-01 15:00:00");

        // date only
        let time = Time::from_str("2026-01-01").unwrap();

        assert_eq!(time.to_string(), "2026-01-01");

        // show HH:MM:SS when subseconds != 0
        let time = Time::new(time.ts() + 100_000);

        assert_eq!(time.to_string(), "2026-01-01 00:00:00");
    }
}
