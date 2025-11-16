/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{NaiveTime, TimeDelta};

pub const DAY_BEGIN: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
pub const DAY_END: NaiveTime = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
pub const MSK_OFFSET: TimeDelta = TimeDelta::hours(3);
pub const MINUTES_IN_DAY: i32 = 24 * 60 * 60;

pub const ONE_SECOND: TimeDelta = TimeDelta::seconds(1);
pub const ONE_MINUTE: TimeDelta = TimeDelta::minutes(1);
pub const ONE_HOUR: TimeDelta = TimeDelta::hours(1);
pub const ONE_DAY: TimeDelta = TimeDelta::days(1);
pub const ONE_WEEK: TimeDelta = TimeDelta::weeks(1);
pub const ONE_MONTH: TimeDelta = TimeDelta::days(30);
pub const ONE_YEAR: TimeDelta = TimeDelta::days(365);
