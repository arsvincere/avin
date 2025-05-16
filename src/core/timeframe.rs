/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use bitcode::{Decode, Encode};
use chrono::{DateTime, TimeDelta, Timelike};
use time_unit::TimeUnit;

use crate::data::MarketData;

#[derive(Debug, Eq, PartialEq, Clone, Encode, Decode)]
pub struct TimeFrame {
    name: &'static str,
}
impl TimeFrame {
    pub fn new(name: &str) -> Self {
        match name {
            "1M" => Self { name: "1M" },
            "5M" => Self { name: "5M" },
            "10M" => Self { name: "10M" },
            "1H" => Self { name: "1H" },
            "D" => Self { name: "D" },
            "W" => Self { name: "W" },
            "M" => Self { name: "M" },
            _ => panic!("Invalid TimeFrame: {}", name),
        }
    }
    pub fn all() -> Vec<TimeFrame> {
        vec![
            TimeFrame::new("1M"),
            TimeFrame::new("5M"),
            TimeFrame::new("10M"),
            TimeFrame::new("1H"),
            TimeFrame::new("D"),
            TimeFrame::new("W"),
            TimeFrame::new("M"),
        ]
    }

    // TODO: delete it? use only Display?
    pub fn name(&self) -> &str {
        self.name
    }
    pub fn timedelta(&self) -> TimeDelta {
        match self.name {
            "1M" => TimeDelta::new(60, 0).unwrap(),
            "5M" => TimeDelta::new(5 * 60, 0).unwrap(),
            "10M" => TimeDelta::new(10 * 60, 0).unwrap(),
            "1H" => TimeDelta::new(60 * 60, 0).unwrap(),
            "D" => TimeDelta::new(24 * 60 * 60, 0).unwrap(),
            "W" => TimeDelta::new(7 * 24 * 60 * 60, 0).unwrap(),
            "M" => TimeDelta::new(31 * 24 * 60 * 60, 0).unwrap(),
            _ => panic!("Invalid TimeFrame: {}", self.name),
        }
    }
    pub fn to_market_data(&self) -> MarketData {
        match self.name {
            "1M" => MarketData::BAR_1M,
            "5M" => MarketData::BAR_5M,
            "10M" => MarketData::BAR_10M,
            "1H" => MarketData::BAR_1H,
            "D" => MarketData::BAR_D,
            "W" => MarketData::BAR_W,
            "M" => MarketData::BAR_M,
            _ => panic!("Invalid TimeFrame: {}", self.name),
        }
    }

    pub fn next_ts(ts: i64, tf: &str) -> i64 {
        let dt = DateTime::from_timestamp_nanos(ts);
        let dt = dt.with_nanosecond(0).unwrap();
        let dt = dt.with_second(0).unwrap();

        match tf {
            "1M" => {
                let ts = dt.timestamp_nanos_opt().unwrap();
                ts + TimeUnit::Minutes.get_unit_nanoseconds() as i64
            }
            "5M" => {
                let need_minutes = 5 - dt.minute() % 5;
                let need_nano = need_minutes as i64
                    * TimeUnit::Minutes.get_unit_nanoseconds() as i64;
                let ts = dt.timestamp_nanos_opt().unwrap();
                ts + need_nano
            }
            "10M" => {
                let need_minutes = 10 - dt.minute() % 10;
                let need_nano = need_minutes as i64
                    * TimeUnit::Minutes.get_unit_nanoseconds() as i64;
                let ts = dt.timestamp_nanos_opt().unwrap();
                ts + need_nano
            }
            "1H" => {
                let dt = dt.with_minute(0).unwrap();
                let ts = dt.timestamp_nanos_opt().unwrap();
                ts + TimeUnit::Hours.get_unit_nanoseconds() as i64
            }
            "D" => {
                let dt = dt.with_minute(0).unwrap();
                let dt = dt.with_hour(0).unwrap();
                let ts = dt.timestamp_nanos_opt().unwrap();
                ts + TimeUnit::Days.get_unit_nanoseconds() as i64
            }
            other => todo!("TimeFrame::next_ts({}, {})", ts, other),
        }
    }
}
impl std::fmt::Display for TimeFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl std::hash::Hash for TimeFrame {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, TimeZone, Utc};

    #[test]
    #[should_panic]
    fn invalid_timeframe() {
        TimeFrame::new("7M");
    }
    #[test]
    fn to_market_data() {
        assert_eq!(TimeFrame::new("1M").to_market_data(), MarketData::BAR_1M);
        assert_eq!(TimeFrame::new("5M").to_market_data(), MarketData::BAR_5M);
        assert_eq!(
            TimeFrame::new("10M").to_market_data(),
            MarketData::BAR_10M
        );
        assert_eq!(TimeFrame::new("1H").to_market_data(), MarketData::BAR_1H);
        assert_eq!(TimeFrame::new("D").to_market_data(), MarketData::BAR_D);
        assert_eq!(TimeFrame::new("W").to_market_data(), MarketData::BAR_W);
        assert_eq!(TimeFrame::new("M").to_market_data(), MarketData::BAR_M);
    }
    #[test]
    fn next_ts() {
        let dt = Utc.with_ymd_and_hms(2023, 8, 1, 10, 0, 5).unwrap();
        let ts = dt.timestamp_nanos_opt().unwrap();

        let next_ts = TimeFrame::next_ts(ts, "1M");
        let next_dt = DateTime::from_timestamp_nanos(next_ts);
        assert_eq!(
            next_dt,
            Utc.with_ymd_and_hms(2023, 8, 1, 10, 1, 0).unwrap()
        );

        let next_ts = TimeFrame::next_ts(ts, "5M");
        let next_dt = DateTime::from_timestamp_nanos(next_ts);
        assert_eq!(
            next_dt,
            Utc.with_ymd_and_hms(2023, 8, 1, 10, 5, 0).unwrap()
        );

        let next_ts = TimeFrame::next_ts(ts, "10M");
        let next_dt = DateTime::from_timestamp_nanos(next_ts);
        assert_eq!(
            next_dt,
            Utc.with_ymd_and_hms(2023, 8, 1, 10, 10, 0).unwrap()
        );

        let next_ts = TimeFrame::next_ts(ts, "1H");
        let next_dt = DateTime::from_timestamp_nanos(next_ts);
        assert_eq!(
            next_dt,
            Utc.with_ymd_and_hms(2023, 8, 1, 11, 0, 0).unwrap()
        );

        let next_ts = TimeFrame::next_ts(ts, "D");
        let next_dt = DateTime::from_timestamp_nanos(next_ts);
        assert_eq!(
            next_dt,
            Utc.with_ymd_and_hms(2023, 8, 2, 0, 0, 0).unwrap()
        );
    }
}
