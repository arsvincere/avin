// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;

use avin_core::Time;
use chrono::{DateTime, Datelike, Days, Months, TimeDelta, Timelike, Utc};

use crate::DomainError;

/// Represents a timeframe supported by AVIN.
///
/// Used by charts and footprints.
///
/// `Month` does not have a fixed duration, so [`TimeFrame::timedelta`],
/// [`TimeFrame::seconds`], and [`TimeFrame::nanos`] return `None` for it.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use avin_domain::TimeFrame;
///
/// let timeframe = TimeFrame::M1;
/// assert_eq!(timeframe.to_string(), "1M");
///
/// let timeframe = TimeFrame::from_str("5m").unwrap();
/// assert_eq!(timeframe, TimeFrame::M5);
/// assert_eq!(timeframe.to_string(), "5M");
/// assert_eq!(timeframe.seconds(), Some(300));
///
/// assert_eq!(TimeFrame::Month.seconds(), None);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeFrame {
    S1,
    S5,
    S10,
    S15,

    M1,
    M5,
    M10,
    M15,

    H1,
    H4,

    Day,
    Week,
    Month,
}

impl TimeFrame {
    /// Returns all supported timeframes.
    pub const fn all() -> &'static [Self] {
        &[
            Self::S1,
            Self::S5,
            Self::S10,
            Self::S15,
            Self::M1,
            Self::M5,
            Self::M10,
            Self::M15,
            Self::H1,
            Self::H4,
            Self::Day,
            Self::Week,
            Self::Month,
        ]
    }

    /// Returns a stable machine-readable identifier suitable for persistence
    /// and serialization.
    pub const fn key(&self) -> &'static str {
        match self {
            Self::S1 => "1s",
            Self::S5 => "5s",
            Self::S10 => "10s",
            Self::S15 => "15s",
            Self::M1 => "1m",
            Self::M5 => "5m",
            Self::M10 => "10m",
            Self::M15 => "15m",
            Self::H1 => "1h",
            Self::H4 => "4h",
            Self::Day => "d",
            Self::Week => "w",
            Self::Month => "m",
        }
    }

    /// Returns the fixed duration in nanoseconds.
    ///
    /// Returns `None` if the timeframe has no fixed duration.
    pub fn nanos(&self) -> Option<u64> {
        let seconds = self.seconds()?;

        Some(seconds as u64 * 1_000_000_000)
    }

    /// Returns the fixed duration in seconds.
    ///
    /// Returns `None` if the timeframe has no fixed duration.
    pub fn seconds(&self) -> Option<u32> {
        let timedelta = self.timedelta()?;

        Some(timedelta.num_seconds() as u32)
    }

    /// Returns the fixed duration as a [`TimeDelta`].
    ///
    /// Returns `None` for [`TimeFrame::Month`] because calendar months do not
    /// have a fixed duration.
    pub fn timedelta(&self) -> Option<TimeDelta> {
        match self {
            Self::S1 => Some(TimeDelta::new(1, 0).unwrap()),
            Self::S5 => Some(TimeDelta::new(5, 0).unwrap()),
            Self::S10 => Some(TimeDelta::new(10, 0).unwrap()),
            Self::S15 => Some(TimeDelta::new(15, 0).unwrap()),

            Self::M1 => Some(TimeDelta::new(60, 0).unwrap()),
            Self::M5 => Some(TimeDelta::new(300, 0).unwrap()),
            Self::M10 => Some(TimeDelta::new(600, 0).unwrap()),
            Self::M15 => Some(TimeDelta::new(900, 0).unwrap()),

            Self::H1 => Some(TimeDelta::new(3_600, 0).unwrap()),
            Self::H4 => Some(TimeDelta::new(14_400, 0).unwrap()),

            Self::Day => Some(TimeDelta::new(86_400, 0).unwrap()),
            Self::Week => Some(TimeDelta::new(604_800, 0).unwrap()),
            Self::Month => None,
        }
    }

    /// Returns the inclusive beginning of the frame containing `time`.
    ///
    /// Frame boundaries are aligned in UTC. Weeks begin on Monday and months
    /// begin on the first day of the month.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use avin_core::Time;
    /// use avin_domain::TimeFrame;
    ///
    /// let time = Time::from_str("2026-08-18 10:13:42").unwrap();
    ///
    /// let expected = Time::from_str("2026-08-18 10:10").unwrap();
    ///
    /// assert_eq!(TimeFrame::M10.begin_frame(time), expected);
    /// ```
    pub fn begin_frame(&self, time: Time) -> Time {
        let floor = |value: u32, step: u32| value - value % step;

        let dt = time.dt().with_nanosecond(0).unwrap();

        let floor_dt = match self {
            Self::S1 => dt,

            Self::S5 => dt.with_second(floor(dt.second(), 5)).unwrap(),

            Self::S10 => dt.with_second(floor(dt.second(), 10)).unwrap(),

            Self::S15 => dt.with_second(floor(dt.second(), 15)).unwrap(),

            Self::M1 => dt.with_second(0).unwrap(),

            Self::M5 => dt
                .with_second(0)
                .unwrap()
                .with_minute(floor(dt.minute(), 5))
                .unwrap(),

            Self::M10 => dt
                .with_second(0)
                .unwrap()
                .with_minute(floor(dt.minute(), 10))
                .unwrap(),

            Self::M15 => dt
                .with_second(0)
                .unwrap()
                .with_minute(floor(dt.minute(), 15))
                .unwrap(),

            Self::H1 => dt.with_second(0).unwrap().with_minute(0).unwrap(),

            Self::H4 => dt
                .with_second(0)
                .unwrap()
                .with_minute(0)
                .unwrap()
                .with_hour(floor(dt.hour(), 4))
                .unwrap(),

            Self::Day => dt
                .with_second(0)
                .unwrap()
                .with_minute(0)
                .unwrap()
                .with_hour(0)
                .unwrap(),

            Self::Week => {
                let dt = dt
                    .with_second(0)
                    .unwrap()
                    .with_minute(0)
                    .unwrap()
                    .with_hour(0)
                    .unwrap();
                let past_days = dt.weekday().num_days_from_monday() as u64;
                dt.checked_sub_days(Days::new(past_days)).unwrap()
            }

            Self::Month => dt
                .with_second(0)
                .unwrap()
                .with_minute(0)
                .unwrap()
                .with_hour(0)
                .unwrap()
                .with_day(1)
                .unwrap(),
        };

        Time::try_from(floor_dt).unwrap()
    }

    /// Returns the exclusive end of the frame containing `time`.
    ///
    /// Together with [`TimeFrame::begin_frame`], this defines the frame as
    /// a half-open interval `[begin, end)`.
    pub fn end_frame(&self, time: Time) -> Time {
        match self {
            Self::Month => {
                let dt = time.dt();
                let month_start = next_month_start(dt);

                Time::try_from(month_start).unwrap()
            }
            _ => {
                let begin = self.begin_frame(time);
                let end_ts = begin.ts() + self.nanos().unwrap() as i64;

                Time::new(end_ts)
            }
        }
    }
}

impl Display for TimeFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::S1 => f.write_str("1S"),
            Self::S5 => f.write_str("5S"),
            Self::S10 => f.write_str("10S"),
            Self::S15 => f.write_str("15S"),

            Self::M1 => f.write_str("1M"),
            Self::M5 => f.write_str("5M"),
            Self::M10 => f.write_str("10M"),
            Self::M15 => f.write_str("15M"),

            Self::H1 => f.write_str("1H"),
            Self::H4 => f.write_str("4H"),

            Self::Day => f.write_str("D"),
            Self::Week => f.write_str("W"),
            Self::Month => f.write_str("M"),
        }
    }
}

impl std::str::FromStr for TimeFrame {
    type Err = DomainError;

    /// Parses a timeframe.
    ///
    /// Parsing is case-insensitive.
    ///
    /// # Errors
    ///
    /// Returns an error if the timeframe key is unknown.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use avin_domain::TimeFrame;
    ///
    /// assert_eq!(TimeFrame::from_str("1m").unwrap(), TimeFrame::M1);
    /// assert_eq!(TimeFrame::from_str("1M").unwrap(), TimeFrame::M1);
    /// assert_eq!(TimeFrame::from_str("d").unwrap(), TimeFrame::Day);
    /// assert_eq!(TimeFrame::from_str("D").unwrap(), TimeFrame::Day);
    ///
    /// assert!(TimeFrame::from_str("Day").is_err());
    /// assert!(TimeFrame::from_str("M1").is_err());
    /// assert!(TimeFrame::from_str("foo").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(timeframe) = Self::all()
            .iter()
            .find(|tf| tf.key().eq_ignore_ascii_case(s))
        {
            return Ok(*timeframe);
        }

        let available = Self::all()
            .iter()
            .map(|tf| tf.key())
            .collect::<Vec<_>>()
            .join(", ");

        let msg = format!(
            "unknown timeframe key '{}', available=[{}]",
            s, available
        );

        Err(DomainError::TimeFrame(msg))
    }
}

// Returns the start of the next calendar month in UTC.
//
// The returned datetime is always the first day of the next month at
// `00:00:00`.
//
// # Panics
//
// Panics if the resulting datetime is outside the range supported by
// `chrono`.
fn next_month_start(dt: DateTime<Utc>) -> DateTime<Utc> {
    dt.with_day(1)
        .unwrap()
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap()
        .checked_add_months(Months::new(1))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::{TimeZone, Utc};

    use super::*;

    #[test]
    fn all() {
        let expected = [
            TimeFrame::S1,
            TimeFrame::S5,
            TimeFrame::S10,
            TimeFrame::S15,
            TimeFrame::M1,
            TimeFrame::M5,
            TimeFrame::M10,
            TimeFrame::M15,
            TimeFrame::H1,
            TimeFrame::H4,
            TimeFrame::Day,
            TimeFrame::Week,
            TimeFrame::Month,
        ];

        assert_eq!(TimeFrame::all(), expected);
    }

    #[test]
    fn key() {
        assert_eq!(TimeFrame::S1.key(), "1s");
        assert_eq!(TimeFrame::S5.key(), "5s");
        assert_eq!(TimeFrame::S10.key(), "10s");
        assert_eq!(TimeFrame::S15.key(), "15s");
        assert_eq!(TimeFrame::M1.key(), "1m");
        assert_eq!(TimeFrame::M5.key(), "5m");
        assert_eq!(TimeFrame::M10.key(), "10m");
        assert_eq!(TimeFrame::M15.key(), "15m");
        assert_eq!(TimeFrame::H1.key(), "1h");
        assert_eq!(TimeFrame::H4.key(), "4h");
        assert_eq!(TimeFrame::Day.key(), "d");
        assert_eq!(TimeFrame::Week.key(), "w");
        assert_eq!(TimeFrame::Month.key(), "m");
    }

    #[test]
    fn duration() {
        let cases = [
            (TimeFrame::S1, 1),
            (TimeFrame::S5, 5),
            (TimeFrame::S10, 10),
            (TimeFrame::S15, 15),
            (TimeFrame::M1, 60),
            (TimeFrame::M5, 5 * 60),
            (TimeFrame::M10, 10 * 60),
            (TimeFrame::M15, 15 * 60),
            (TimeFrame::H1, 60 * 60),
            (TimeFrame::H4, 4 * 60 * 60),
            (TimeFrame::Day, 24 * 60 * 60),
            (TimeFrame::Week, 7 * 24 * 60 * 60),
        ];

        for (timeframe, seconds) in cases {
            assert_eq!(timeframe.seconds(), Some(seconds));
            assert_eq!(
                timeframe.nanos(),
                Some(seconds as u64 * 1_000_000_000),
            );
            assert_eq!(
                timeframe.timedelta().unwrap().num_seconds(),
                seconds as i64,
            );
        }

        assert_eq!(TimeFrame::Month.nanos(), None);
        assert_eq!(TimeFrame::Month.seconds(), None);
        assert_eq!(TimeFrame::Month.timedelta(), None);
    }

    #[test]
    fn begin_frame() {
        let dt = Utc
            .with_ymd_and_hms(2023, 8, 2, 10, 13, 42)
            .unwrap()
            .with_nanosecond(123_456_789)
            .unwrap();

        let input = Time::try_from(dt).unwrap();

        let cases = [
            (TimeFrame::S1, Time::from_str("2023-08-02 10:13:42")),
            (TimeFrame::S5, Time::from_str("2023-08-02 10:13:40")),
            (TimeFrame::S10, Time::from_str("2023-08-02 10:13:40")),
            (TimeFrame::S15, Time::from_str("2023-08-02 10:13:30")),
            (TimeFrame::M1, Time::from_str("2023-08-02 10:13:00")),
            (TimeFrame::M5, Time::from_str("2023-08-02 10:10:00")),
            (TimeFrame::M10, Time::from_str("2023-08-02 10:10:00")),
            (TimeFrame::M15, Time::from_str("2023-08-02 10:00:00")),
            (TimeFrame::H1, Time::from_str("2023-08-02 10:00:00")),
            (TimeFrame::H4, Time::from_str("2023-08-02 08:00:00")),
            (TimeFrame::Day, Time::from_str("2023-08-02 00:00:00")),
            (TimeFrame::Week, Time::from_str("2023-07-31 00:00:00")),
            (TimeFrame::Month, Time::from_str("2023-08-01 00:00:00")),
        ];

        for (timeframe, expected) in cases {
            assert_eq!(timeframe.begin_frame(input), expected.unwrap());
        }
    }

    #[test]
    fn end_frame() {
        let dt = Utc
            .with_ymd_and_hms(2023, 8, 2, 10, 13, 42)
            .unwrap()
            .with_nanosecond(123_456_789)
            .unwrap();

        let input = Time::try_from(dt).unwrap();

        let cases = [
            (TimeFrame::S1, Time::from_str("2023-08-02 10:13:43")),
            (TimeFrame::S5, Time::from_str("2023-08-02 10:13:45")),
            (TimeFrame::S10, Time::from_str("2023-08-02 10:13:50")),
            (TimeFrame::S15, Time::from_str("2023-08-02 10:13:45")),
            (TimeFrame::M1, Time::from_str("2023-08-02 10:14:00")),
            (TimeFrame::M5, Time::from_str("2023-08-02 10:15:00")),
            (TimeFrame::M10, Time::from_str("2023-08-02 10:20:00")),
            (TimeFrame::M15, Time::from_str("2023-08-02 10:15:00")),
            (TimeFrame::H1, Time::from_str("2023-08-02 11:00:00")),
            (TimeFrame::H4, Time::from_str("2023-08-02 12:00:00")),
            (TimeFrame::Day, Time::from_str("2023-08-03 00:00:00")),
            (TimeFrame::Week, Time::from_str("2023-08-07 00:00:00")),
            (TimeFrame::Month, Time::from_str("2023-09-01 00:00:00")),
        ];

        for (timeframe, expected) in cases {
            assert_eq!(timeframe.end_frame(input), expected.unwrap());
        }
    }

    #[test]
    fn display() {
        let cases = [
            (TimeFrame::S1, "1S"),
            (TimeFrame::S5, "5S"),
            (TimeFrame::S10, "10S"),
            (TimeFrame::S15, "15S"),
            (TimeFrame::M1, "1M"),
            (TimeFrame::M5, "5M"),
            (TimeFrame::M10, "10M"),
            (TimeFrame::M15, "15M"),
            (TimeFrame::H1, "1H"),
            (TimeFrame::H4, "4H"),
            (TimeFrame::Day, "D"),
            (TimeFrame::Week, "W"),
            (TimeFrame::Month, "M"),
        ];

        for (timeframe, expected) in cases {
            assert_eq!(timeframe.to_string(), expected);
        }
    }

    #[test]
    fn from_str() {
        for timeframe in TimeFrame::all().iter() {
            let key = timeframe.key();
            assert_eq!(TimeFrame::from_str(key).unwrap(), *timeframe);
        }

        assert_eq!(TimeFrame::from_str("4H").unwrap(), TimeFrame::H4);
        assert_eq!(TimeFrame::from_str("D").unwrap(), TimeFrame::Day);

        assert!(matches!(
            TimeFrame::from_str("M1").unwrap_err(),
            DomainError::TimeFrame(_)
        ));
        assert!(matches!(
            TimeFrame::from_str("Day").unwrap_err(),
            DomainError::TimeFrame(_)
        ));
        assert!(matches!(
            TimeFrame::from_str("foo").unwrap_err(),
            DomainError::TimeFrame(_)
        ));
    }

    #[test]
    fn test_next_month_start() {
        let dt = Utc
            .with_ymd_and_hms(2023, 8, 2, 10, 7, 15)
            .unwrap()
            .with_nanosecond(123_456_789)
            .unwrap();
        let next = next_month_start(dt);
        assert_eq!(next, Utc.with_ymd_and_hms(2023, 9, 1, 0, 0, 0).unwrap());

        let dt = Utc.with_ymd_and_hms(2023, 12, 31, 23, 59, 59).unwrap();
        let next = next_month_start(dt);
        assert_eq!(next, Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());

        let dt = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let next = next_month_start(dt);
        assert_eq!(next, Utc.with_ymd_and_hms(2023, 2, 1, 0, 0, 0).unwrap());
    }
}
