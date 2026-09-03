// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use chrono::{DateTime, Datelike, Months, Timelike, Utc};

// TODO: ?
// перенести в Time?
/// Returns the start of the next calendar month in UTC.
///
/// The returned datetime is always the first day of the next month at
/// `00:00:00`.
///
/// # Panics
///
/// Panics if the resulting datetime is outside the range supported by
/// `chrono`.
#[inline]
pub fn next_month_start(dt: DateTime<Utc>) -> DateTime<Utc> {
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

// TODO: ?
// перенести в Time?
/// Returns the start of the previous calendar month in UTC.
///
/// The returned datetime is always the first day of the previous month at
/// `00:00:00`.
///
/// # Panics
///
/// Panics if the resulting datetime is outside the range supported by
/// `chrono`.
#[inline]
pub fn prev_month_start(dt: DateTime<Utc>) -> DateTime<Utc> {
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
        .checked_sub_months(Months::new(1))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

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

    #[test]
    fn test_prev_month_start() {
        let dt = Utc
            .with_ymd_and_hms(2023, 8, 2, 10, 7, 15)
            .unwrap()
            .with_nanosecond(123_456_789)
            .unwrap();
        let prev = prev_month_start(dt);
        assert_eq!(prev, Utc.with_ymd_and_hms(2023, 7, 1, 0, 0, 0).unwrap());

        let dt = Utc.with_ymd_and_hms(2023, 12, 31, 23, 59, 59).unwrap();
        let prev = prev_month_start(dt);
        assert_eq!(prev, Utc.with_ymd_and_hms(2023, 11, 1, 0, 0, 0).unwrap());

        let dt = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let prev = prev_month_start(dt);
        assert_eq!(prev, Utc.with_ymd_and_hms(2022, 12, 1, 0, 0, 0).unwrap());
    }
}
