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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time(i64);

impl Time {
    pub fn now() -> Time {
        Time::new(Utc::now().timestamp_nanos_opt().unwrap())
    }

    pub fn new(timestamp_nanos: i64) -> Self {
        Time(timestamp_nanos)
    }

    pub fn dt(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_nanos(self.0)
    }

    pub fn ts(&self) -> i64 {
        self.0
    }
}

impl Display for Time {
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

    // Всегда интерпретируется как UTC
    //
    // supported formats:
    // 2026-01-01 12:55:00
    // 2026-01-01 12:55
    // 2026-01-01
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
