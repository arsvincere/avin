// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use chrono::{Datelike, Days, TimeDelta, Timelike};

use avin_utils::AvinError;

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

    pub fn nanos(&self) -> Option<u64> {
        let seconds = self.seconds()?;

        Some(seconds as u64 * 1_000_000_000)
    }

    pub fn seconds(&self) -> Option<u32> {
        let timedelta = self.timedelta()?;

        Some(timedelta.num_seconds() as u32)
    }

    pub fn timedelta(&self) -> Option<TimeDelta> {
        match self {
            Self::S1 => Some(TimeDelta::new(1, 0).unwrap()),
            Self::S5 => Some(TimeDelta::new(5, 0).unwrap()),
            Self::S10 => Some(TimeDelta::new(10, 0).unwrap()),
            Self::S30 => Some(TimeDelta::new(30, 0).unwrap()),

            Self::M1 => Some(TimeDelta::new(60, 0).unwrap()),
            Self::M5 => Some(TimeDelta::new(5 * 60, 0).unwrap()),
            Self::M10 => Some(TimeDelta::new(10 * 60, 0).unwrap()),
            Self::M15 => Some(TimeDelta::new(15 * 60, 0).unwrap()),

            Self::H1 => Some(TimeDelta::new(60 * 60, 0).unwrap()),
            Self::H4 => Some(TimeDelta::new(4 * 60 * 60, 0).unwrap()),

            Self::Day => Some(TimeDelta::new(24 * 60 * 60, 0).unwrap()),
            Self::Week => Some(TimeDelta::new(7 * 24 * 60 * 60, 0).unwrap()),
            Self::Month => None,
        }
    }

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

impl std::str::FromStr for TimeFrame {
    type Err = AvinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::all()
            .iter()
            .copied()
            .find(|timeframe| timeframe.to_string().eq_ignore_ascii_case(s))
            .ok_or_else(|| {
                let available = Self::all()
                    .iter()
                    .map(Self::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");

                let msg = format!(
                    "unknown timeframe '{}', available=[{}]",
                    s, available,
                );

                AvinError::InvalidValue(msg)
            })
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
