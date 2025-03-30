use chrono::{NaiveTime, TimeDelta};

// Settings
pub const DEFAULT_BARS_COUNT: i32 = 5000;

// Dir
pub const DATA_DIR: &str = "/home/alex/avin/usr/data";

// Datetime
pub const DT_FMT: &str = "%Y-%m-%d %H:%M:%S";
pub const DAY_BEGIN: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
pub const DAY_END: NaiveTime = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
pub const MSK_TIME_DIF: TimeDelta = TimeDelta::hours(3);
