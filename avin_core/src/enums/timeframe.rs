/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{DateTime, TimeDelta, Timelike};
use strum::EnumIter;
use time_unit::TimeUnit;

use crate::MarketData;

/// List for selecting the timeframe.
///
/// # ru
/// Перечисление для выбора таймфрейма. Используется в
/// [`crate::Chart`] и [`crate::Footprint`]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    bitcode::Encode,
    bitcode::Decode,
    EnumIter,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum TimeFrame {
    M1,
    M10,
    H1,
    Day,
    Week,
    Month,
}
impl TimeFrame {
    // const ALL: &'static [Self] =
    //     &[Self::A, Self::B, Self::C, Self::D, Self::E];
    // pub const ALL: &'static [Self] = &[
    //     Self::M1,
    //     Self::M10,
    //     Self::H1,
    //     Self::Day,
    //     Self::Week,
    //     Self::Month,
    // ];

    pub fn all() -> Vec<TimeFrame> {
        vec![
            TimeFrame::M1,
            TimeFrame::M10,
            TimeFrame::H1,
            TimeFrame::Day,
            TimeFrame::Week,
            TimeFrame::Month,
        ]
    }
    pub fn next_ts(&self, ts: i64) -> i64 {
        let dt = DateTime::from_timestamp_nanos(ts);
        let dt = dt.with_nanosecond(0).unwrap();
        let dt = dt.with_second(0).unwrap();

        match self {
            TimeFrame::M1 => {
                let ts = dt.timestamp_nanos_opt().unwrap();
                ts + TimeUnit::Minutes.get_unit_nanoseconds() as i64
            }
            // "5M" => {
            //     let need_minutes = 5 - dt.minute() % 5;
            //     let need_nano = need_minutes as i64
            //         * TimeUnit::Minutes.get_unit_nanoseconds() as i64;
            //     let ts = dt.timestamp_nanos_opt().unwrap();
            //     ts + need_nano
            // }
            TimeFrame::M10 => {
                let need_minutes = 10 - dt.minute() % 10;
                let need_nano = need_minutes as i64
                    * TimeUnit::Minutes.get_unit_nanoseconds() as i64;
                let ts = dt.timestamp_nanos_opt().unwrap();
                ts + need_nano
            }
            TimeFrame::H1 => {
                let dt = dt.with_minute(0).unwrap();
                let ts = dt.timestamp_nanos_opt().unwrap();
                ts + TimeUnit::Hours.get_unit_nanoseconds() as i64
            }
            TimeFrame::Day => {
                let dt = dt.with_minute(0).unwrap();
                let dt = dt.with_hour(0).unwrap();
                let ts = dt.timestamp_nanos_opt().unwrap();
                ts + TimeUnit::Days.get_unit_nanoseconds() as i64
            }
            other => todo!("TimeFrame::next_ts({}, {})", ts, other),
        }
    }
    pub fn prev_ts(&self, ts: i64) -> i64 {
        let dt = DateTime::from_timestamp_nanos(ts);
        let dt = dt.with_nanosecond(0).unwrap();
        let dt = dt.with_second(0).unwrap();

        match self {
            TimeFrame::M1 => dt.timestamp_nanos_opt().unwrap(),
            TimeFrame::M10 => {
                let past_minutes = dt.minute() % 10;
                let past_nano = past_minutes as i64
                    * TimeUnit::Minutes.get_unit_nanoseconds() as i64;
                let ts = dt.timestamp_nanos_opt().unwrap();
                ts - past_nano
            }
            TimeFrame::H1 => {
                let dt = dt.with_minute(0).unwrap();
                dt.timestamp_nanos_opt().unwrap()
            }
            TimeFrame::Day => {
                let dt = dt.with_minute(0).unwrap().with_hour(0).unwrap();
                dt.timestamp_nanos_opt().unwrap()
            }
            other => todo!("TimeFrame::next_ts({}, {})", ts, other),
        }
    }

    pub fn timedelta(&self) -> TimeDelta {
        match self {
            Self::M1 => TimeDelta::new(60, 0).unwrap(),
            // "5M" => TimeDelta::new(5 * 60, 0).unwrap(),
            Self::M10 => TimeDelta::new(10 * 60, 0).unwrap(),
            Self::H1 => TimeDelta::new(60 * 60, 0).unwrap(),
            Self::Day => TimeDelta::new(24 * 60 * 60, 0).unwrap(),
            Self::Week => TimeDelta::new(7 * 24 * 60 * 60, 0).unwrap(),
            Self::Month => TimeDelta::new(31 * 24 * 60 * 60, 0).unwrap(),
        }
    }
    pub fn nanos(&self) -> i64 {
        let nanos = match self {
            Self::M1 => TimeUnit::Minutes.get_unit_nanoseconds(),
            // "5M" => TimeUnit::Minutes.get_unit_nanoseconds() * 5,
            Self::M10 => TimeUnit::Minutes.get_unit_nanoseconds() * 10,
            Self::H1 => TimeUnit::Hours.get_unit_nanoseconds(),
            Self::Day => TimeUnit::Days.get_unit_nanoseconds(),
            Self::Week => TimeUnit::Weeks.get_unit_nanoseconds(),
            Self::Month => TimeUnit::Days.get_unit_nanoseconds() * 31,
        };

        nanos as i64
    }
    pub fn market_data(&self) -> MarketData {
        match self {
            Self::M1 => MarketData::BAR_1M,
            // "5M" => MarketData::BAR_5M,
            Self::M10 => MarketData::BAR_10M,
            Self::H1 => MarketData::BAR_1H,
            Self::Day => MarketData::BAR_D,
            Self::Week => MarketData::BAR_W,
            Self::Month => MarketData::BAR_M,
        }
    }
}
impl std::fmt::Display for TimeFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::M1 => write!(f, "1M"),
            // Self::M5 => write!(f, "5M"),
            Self::M10 => write!(f, "10M"),
            Self::H1 => write!(f, "1H"),
            Self::Day => write!(f, "D"),
            Self::Week => write!(f, "W"),
            Self::Month => write!(f, "M"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, TimeZone, Utc};

    #[test]
    fn to_market_data() {
        assert_eq!(TimeFrame::M1.market_data(), MarketData::BAR_1M);
        assert_eq!(TimeFrame::M10.market_data(), MarketData::BAR_10M);
        assert_eq!(TimeFrame::H1.market_data(), MarketData::BAR_1H);
        assert_eq!(TimeFrame::Day.market_data(), MarketData::BAR_D);
        assert_eq!(TimeFrame::Week.market_data(), MarketData::BAR_W);
        assert_eq!(TimeFrame::Month.market_data(), MarketData::BAR_M);
    }
    #[test]
    fn next_ts() {
        let dt = Utc.with_ymd_and_hms(2023, 8, 1, 10, 0, 5).unwrap();
        let ts = dt.timestamp_nanos_opt().unwrap();

        let next_ts = TimeFrame::M1.next_ts(ts);
        let next_dt = DateTime::from_timestamp_nanos(next_ts);
        assert_eq!(
            next_dt,
            Utc.with_ymd_and_hms(2023, 8, 1, 10, 1, 0).unwrap()
        );

        // let next_ts = TimeFrame::M5(ts);
        // let next_dt = DateTime::from_timestamp_nanos(next_ts);
        // assert_eq!(
        //     next_dt,
        //     Utc.with_ymd_and_hms(2023, 8, 1, 10, 5, 0).unwrap()
        // );

        let next_ts = TimeFrame::M10.next_ts(ts);
        let next_dt = DateTime::from_timestamp_nanos(next_ts);
        assert_eq!(
            next_dt,
            Utc.with_ymd_and_hms(2023, 8, 1, 10, 10, 0).unwrap()
        );

        let next_ts = TimeFrame::H1.next_ts(ts);
        let next_dt = DateTime::from_timestamp_nanos(next_ts);
        assert_eq!(
            next_dt,
            Utc.with_ymd_and_hms(2023, 8, 1, 11, 0, 0).unwrap()
        );

        let next_ts = TimeFrame::Day.next_ts(ts);
        let next_dt = DateTime::from_timestamp_nanos(next_ts);
        assert_eq!(
            next_dt,
            Utc.with_ymd_and_hms(2023, 8, 2, 0, 0, 0).unwrap()
        );
    }
    #[test]
    fn prev_ts() {
        let dt = Utc.with_ymd_and_hms(2023, 8, 1, 10, 3, 5).unwrap();
        let ts = dt.timestamp_nanos_opt().unwrap();

        let prev_ts = TimeFrame::M1.prev_ts(ts);
        let prev_dt = DateTime::from_timestamp_nanos(prev_ts);
        assert_eq!(
            prev_dt,
            Utc.with_ymd_and_hms(2023, 8, 1, 10, 3, 0).unwrap()
        );

        // let prev_ts = TimeFrame::M5(ts);
        // let prev_dt = DateTime::from_timestamp_nanos(prev_ts);
        // assert_eq!(
        //     prev_dt,
        //     Utc.with_ymd_and_hms(2023, 8, 1, 10, 5, 0).unwrap()
        // );

        let prev_ts = TimeFrame::M10.prev_ts(ts);
        let prev_dt = DateTime::from_timestamp_nanos(prev_ts);
        assert_eq!(
            prev_dt,
            Utc.with_ymd_and_hms(2023, 8, 1, 10, 0, 0).unwrap()
        );

        let prev_ts = TimeFrame::H1.prev_ts(ts);
        let prev_dt = DateTime::from_timestamp_nanos(prev_ts);
        assert_eq!(
            prev_dt,
            Utc.with_ymd_and_hms(2023, 8, 1, 10, 0, 0).unwrap()
        );

        let prev_ts = TimeFrame::Day.prev_ts(ts);
        let prev_dt = DateTime::from_timestamp_nanos(prev_ts);
        assert_eq!(
            prev_dt,
            Utc.with_ymd_and_hms(2023, 8, 1, 0, 0, 0).unwrap()
        );
    }
}
