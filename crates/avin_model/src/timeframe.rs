// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use chrono::{Datelike, Days, TimeDelta, Timelike};

use avin_utils::AvinError;

/// Timeframe.
///
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
/// use avin_model::TimeFrame;
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
    S30,

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
            Self::S30,
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
            Self::S30 => Some(TimeDelta::new(30, 0).unwrap()),

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

    /// Returns the inclusive beginning of the frame containing `ts`.
    ///
    /// `ts` and the returned value are Unix timestamps in nanoseconds.
    ///
    /// Frame boundaries are aligned in UTC. Weeks begin on Monday and months
    /// begin on the first day of the month.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::{TimeZone, Utc};
    ///
    /// use avin_model::TimeFrame;
    ///
    /// let ts = Utc
    ///     .with_ymd_and_hms(2026, 8, 18, 10, 13, 42)
    ///     .unwrap()
    ///     .timestamp_nanos_opt()
    ///     .unwrap();
    ///
    /// let expected = Utc
    ///     .with_ymd_and_hms(2026, 8, 18, 10, 10, 0)
    ///     .unwrap()
    ///     .timestamp_nanos_opt()
    ///     .unwrap();
    ///
    /// assert_eq!(TimeFrame::M10.begin_frame_ts(ts), expected);
    /// ```
    pub fn begin_frame_ts(&self, ts: i64) -> i64 {
        let floor = |value: u32, step: u32| value - value % step;

        let dt = avin_utils::dt(ts).with_nanosecond(0).unwrap();

        let floor_dt = match self {
            Self::S1 => dt,

            Self::S5 => dt.with_second(floor(dt.second(), 5)).unwrap(),

            Self::S10 => dt.with_second(floor(dt.second(), 10)).unwrap(),

            Self::S30 => dt.with_second(floor(dt.second(), 30)).unwrap(),

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

        avin_utils::ts(floor_dt)
    }

    /// Returns the exclusive end of the frame containing `ts`.
    ///
    /// `ts` and the returned value are Unix timestamps in nanoseconds.
    ///
    /// Together with [`TimeFrame::begin_frame_ts`], this defines the frame as
    /// a half-open interval `[begin, end)`.
    pub fn end_frame_ts(&self, ts: i64) -> i64 {
        match self {
            Self::Month => {
                let dt = avin_utils::dt(ts);
                let next_month_start = avin_utils::next_month_start(dt);

                avin_utils::ts(next_month_start)
            }
            _ => {
                let begin_ts = self.begin_frame_ts(ts);

                begin_ts + self.nanos().unwrap() as i64
            }
        }
    }
}

impl std::fmt::Display for TimeFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::S1 => f.write_str("1S"),
            Self::S5 => f.write_str("5S"),
            Self::S10 => f.write_str("10S"),
            Self::S30 => f.write_str("30S"),

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
    type Err = AvinError;

    /// Parses a timeframe from its canonical textual representation.
    ///
    /// Parsing is case-insensitive. Accepted values correspond to the
    /// representation produced by [`std::fmt::Display`], such as `"1S"`,
    /// `"15M"`, `"4H"`, `"D"`, `"W"`, and `"M"`.
    ///
    /// # Errors
    ///
    /// Returns an error if the timeframe is unknown.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use avin_model::TimeFrame;
    ///
    /// assert_eq!(TimeFrame::from_str("1m").unwrap(), TimeFrame::M1);
    /// assert_eq!(TimeFrame::from_str("1M").unwrap(), TimeFrame::M1);
    /// assert_eq!(TimeFrame::from_str("4H").unwrap(), TimeFrame::H4);
    /// assert_eq!(TimeFrame::from_str("D").unwrap(), TimeFrame::Day);
    ///
    /// assert!(TimeFrame::from_str("M1").is_err());
    /// assert!(TimeFrame::from_str("foo").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(timeframe) = Self::all()
            .iter()
            .copied()
            .find(|tf| tf.to_string().eq_ignore_ascii_case(s))
        {
            return Ok(timeframe);
        }

        let all = Self::all()
            .iter()
            .map(Self::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        let msg = format!("unknown timeframe '{}', available=[{}]", s, all);

        Err(AvinError::InvalidValue(msg))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::{TimeZone, Utc};

    use super::*;

    // helper - returns timestamp nanos of datetime
    fn ts(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        nanos: u32,
    ) -> i64 {
        let dt = Utc
            .with_ymd_and_hms(year, month, day, hour, minute, second)
            .unwrap()
            .with_nanosecond(nanos)
            .unwrap();

        avin_utils::ts(dt)
    }

    #[test]
    fn all() {
        let expected = [
            TimeFrame::S1,
            TimeFrame::S5,
            TimeFrame::S10,
            TimeFrame::S30,
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
    fn duration() {
        let cases = [
            (TimeFrame::S1, 1),
            (TimeFrame::S5, 5),
            (TimeFrame::S10, 10),
            (TimeFrame::S30, 30),
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
    fn begin_frame_ts() {
        let input = ts(2023, 8, 2, 10, 13, 42, 123_456_789);

        let cases = [
            (TimeFrame::S1, ts(2023, 8, 2, 10, 13, 42, 0)),
            (TimeFrame::S5, ts(2023, 8, 2, 10, 13, 40, 0)),
            (TimeFrame::S10, ts(2023, 8, 2, 10, 13, 40, 0)),
            (TimeFrame::S30, ts(2023, 8, 2, 10, 13, 30, 0)),
            (TimeFrame::M1, ts(2023, 8, 2, 10, 13, 0, 0)),
            (TimeFrame::M5, ts(2023, 8, 2, 10, 10, 0, 0)),
            (TimeFrame::M10, ts(2023, 8, 2, 10, 10, 0, 0)),
            (TimeFrame::M15, ts(2023, 8, 2, 10, 0, 0, 0)),
            (TimeFrame::H1, ts(2023, 8, 2, 10, 0, 0, 0)),
            (TimeFrame::H4, ts(2023, 8, 2, 8, 0, 0, 0)),
            (TimeFrame::Day, ts(2023, 8, 2, 0, 0, 0, 0)),
            (TimeFrame::Week, ts(2023, 7, 31, 0, 0, 0, 0)),
            (TimeFrame::Month, ts(2023, 8, 1, 0, 0, 0, 0)),
        ];

        for (timeframe, expected) in cases {
            assert_eq!(timeframe.begin_frame_ts(input), expected);
        }
    }

    #[test]
    fn end_frame_ts() {
        let input = ts(2023, 8, 2, 10, 13, 42, 123_456_789);

        let cases = [
            (TimeFrame::S1, ts(2023, 8, 2, 10, 13, 43, 0)),
            (TimeFrame::S5, ts(2023, 8, 2, 10, 13, 45, 0)),
            (TimeFrame::S10, ts(2023, 8, 2, 10, 13, 50, 0)),
            (TimeFrame::S30, ts(2023, 8, 2, 10, 14, 0, 0)),
            (TimeFrame::M1, ts(2023, 8, 2, 10, 14, 0, 0)),
            (TimeFrame::M5, ts(2023, 8, 2, 10, 15, 0, 0)),
            (TimeFrame::M10, ts(2023, 8, 2, 10, 20, 0, 0)),
            (TimeFrame::M15, ts(2023, 8, 2, 10, 15, 0, 0)),
            (TimeFrame::H1, ts(2023, 8, 2, 11, 0, 0, 0)),
            (TimeFrame::H4, ts(2023, 8, 2, 12, 0, 0, 0)),
            (TimeFrame::Day, ts(2023, 8, 3, 0, 0, 0, 0)),
            (TimeFrame::Week, ts(2023, 8, 7, 0, 0, 0, 0)),
            (TimeFrame::Month, ts(2023, 9, 1, 0, 0, 0, 0)),
        ];

        for (timeframe, expected) in cases {
            assert_eq!(timeframe.end_frame_ts(input), expected);
        }
    }

    #[test]
    fn display() {
        let cases = [
            (TimeFrame::S1, "1S"),
            (TimeFrame::S5, "5S"),
            (TimeFrame::S10, "10S"),
            (TimeFrame::S30, "30S"),
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
        for timeframe in TimeFrame::all() {
            let canonical = timeframe.to_string();
            assert_eq!(TimeFrame::from_str(&canonical).unwrap(), *timeframe);

            let lower_case = canonical.to_ascii_lowercase();
            assert_eq!(TimeFrame::from_str(&lower_case).unwrap(), *timeframe);
        }

        assert!(TimeFrame::from_str("M1").is_err());
        assert!(TimeFrame::from_str("Day").is_err());
        assert!(TimeFrame::from_str("foo").is_err());
    }
}
