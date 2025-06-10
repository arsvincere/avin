/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use chrono::{
    DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc,
};
use polars::{frame::DataFrame, prelude::NamedFrom, series::Series};

use crate::conf::DT_FMT;

static POW_VEC: &'static [f64] = &[
    0.0,
    10.0,
    100.0,
    1000.0,
    10000.0,
    100000.0,
    1000000.0,
    10000000.0,
    100000000.0,
    1000000000.0,
];

/// Return UTC datetime from user local datetime
pub fn datetime(dt: &str) -> DateTime<Utc> {
    let dt = NaiveDateTime::parse_from_str(dt, DT_FMT).unwrap();
    let dt = Local.from_local_datetime(&dt).unwrap();

    dt.to_utc()
}
/// Return UTC datetime from user local date
pub fn date(d: &str) -> DateTime<Utc> {
    let dt = NaiveDate::parse_from_str(d, "%Y-%m-%d")
        .unwrap()
        .and_time(NaiveTime::MIN);
    let dt = Local.from_local_datetime(&dt).unwrap();

    dt.to_utc()
}
/// Convert datetime UTC -> timestamp nanos
pub fn ts(dt: &DateTime<Utc>) -> i64 {
    dt.timestamp_nanos_opt().unwrap()
}
/// Convert timestamp nanos -> datetime UTC
pub fn dt(ts_nanos: i64) -> DateTime<Utc> {
    DateTime::from_timestamp_nanos(ts_nanos)
}
/// Replace column 'ts_nanos' to 'dt' (naive UTC datetime)
pub fn replace_ts(mut df: DataFrame) -> DataFrame {
    // get iter of column 'ts_nanos'
    let timestamps = df
        .column("ts_nanos")
        .unwrap()
        .i64()
        .unwrap()
        .into_no_null_iter();

    // collect dt values
    let mut datetimes = Vec::new();
    for ts in timestamps {
        let dt = dt(ts);
        datetimes.push(dt.naive_utc());
    }

    // create dt series & replace
    let datetimes = Series::new("dt".into(), datetimes);
    df.replace("ts_nanos", datetimes).unwrap();
    df.rename("ts_nanos", "dt".into()).unwrap();

    df
}

pub fn round(num: f64, precision: u8) -> f64 {
    assert!(precision <= 9);

    if precision == 0 {
        num.round()
    } else {
        let multiplier = POW_VEC[precision as usize];
        let tmp_value = (num * multiplier).round().abs() as u64;

        (tmp_value as f64 / multiplier) * num.signum()
    }
}
pub fn round_price(price: f64, step: f64) -> f64 {
    let price = (price * POW_VEC[9]).round() as u64;
    let step = (step * POW_VEC[9]).round() as u64;
    let frac = price % step;

    // если дробная часть меньше половины шага цены -> trunc
    let tmp = if frac < step / 2 {
        price - price % step
    }
    // если дробная часть больше половины шага цены -> trunc + step
    else {
        price - price % step + step
    };

    tmp as f64 / POW_VEC[9]
}

pub fn max<T: PartialOrd>(left: T, right: T) -> T {
    if left > right { left } else { right }
}
pub fn min<T: PartialOrd>(left: T, right: T) -> T {
    if left < right { left } else { right }
}
pub fn sum<T: std::ops::Add>(
    left: T,
    right: T,
) -> <T as std::ops::Add<T>>::Output {
    left + right
}

pub fn bisect_left<T, U>(list: &[T], x: U, key: fn(&T) -> U) -> Option<usize>
where
    U: PartialOrd + Ord,
{
    // NOTE:
    // если пустой вектор -> None
    // если меньше первого -> None
    // если больше последнего -> ПОСЛЕДНИЙ
    // если есть == x вернет индекс самого ПЕРВОГО вхождения
    // если x между то вернет индекс СЛЕВА от x

    // начальные проверки
    if list.len() == 0 {
        // пустой вектор
        return None;
    } else if x < key(&list[0]) {
        // искомый меньше всех в векторе
        return None;
    } else if x > key(&list[list.len() - 1]) {
        // искомый больше всех в векторе
        return Some(list.len() - 1);
    }

    let result = list.binary_search_by_key(&x, key);
    match result {
        Ok(mut i) => {
            // back to first entry
            while i > 1 {
                if key(&list[i - 1]) == x {
                    i -= 1;
                } else {
                    break;
                }
            }
            return Some(i);
        }
        Err(i) => Some(i - 1),
    }
}
pub fn bisect_right<T, U>(list: &[T], x: U, key: fn(&T) -> U) -> Option<usize>
where
    U: PartialOrd + Ord,
{
    // NOTE:
    // если пустой вектор -> None
    // если меньше первого -> 0
    // если больше последнего -> None
    // если есть == x вернет индекс самого ПЕРВОГО вхождения
    // если x между то вернет индекс СПРАВА от x

    // начальные проверки
    if list.len() == 0 {
        // пустой вектор
        return None;
    } else if x < key(&list[0]) {
        // искомый меньше всех в векторе
        return Some(0);
    } else if x > key(&list[list.len() - 1]) {
        // искомый больше всех в векторе
        return None;
    }

    let result = list.binary_search_by_key(&x, key);
    match result {
        Ok(mut i) => {
            // back to first entry
            while i > 1 {
                if key(&list[i - 1]) == x {
                    i -= 1;
                } else {
                    break;
                }
            }
            return Some(i);
        }
        Err(i) => Some(i),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_datetime() {
        let dt = datetime("2025-01-01 10:00:00");
        let utc_dt = Utc.with_ymd_and_hms(2025, 1, 1, 7, 0, 0).unwrap();
        assert_eq!(utc_dt, dt);
    }
    #[test]
    fn local_date() {
        let dt = date("2025-01-01");
        let utc_dt = Utc.with_ymd_and_hms(2024, 12, 31, 21, 0, 0).unwrap();
        assert_eq!(utc_dt, dt);
    }
    #[test]
    fn g_round() {
        let x: f64 = 123.456789;

        assert_eq!(round(x, 1), 123.5);
        assert_eq!(round(x, 2), 123.46);
        assert_eq!(round(x, 3), 123.457);
        assert_eq!(round(x, 4), 123.4568);
        assert_eq!(round(x, 5), 123.45679);
        assert_eq!(round(x, 6), 123.456789);

        assert_eq!(round(123.9_f64, 0), 124.0);
    }
    #[test]
    fn rounding() {
        let x: f64 = 123.111111111;

        assert_eq!(round(x, 0), 123.0);
        assert_eq!(round(x, 1), 123.1);
        assert_eq!(round(x, 2), 123.11);
        assert_eq!(round(x, 3), 123.111);
        assert_eq!(round(x, 4), 123.1111);
        assert_eq!(round(x, 5), 123.11111);
        assert_eq!(round(x, 6), 123.111111);
        assert_eq!(round(x, 7), 123.1111111);
        assert_eq!(round(x, 8), 123.11111111);
        assert_eq!(round(x, 9), 123.111111111);
    }
    #[test]
    fn rounding_prices() {
        let price = 88.0;

        // округление цены до шага цены 0.01
        let step = 0.01;

        let buy = price * 0.999;
        let rounded = round_price(buy, step);
        assert_eq!(buy, 87.912);
        assert_eq!(rounded, 87.91);

        let sell = price * 1.001;
        let rounded = round_price(sell, step);
        assert_eq!(sell, 88.088);
        assert_eq!(rounded, 88.09);

        // округление цены до шага цены 0.5
        let step = 0.5;

        let buy = price * 0.99;
        let rounded = round_price(buy, step);
        assert_eq!(buy, 87.12);
        assert_eq!(rounded, 87.0);

        let sell = price * 1.01;
        let rounded = round_price(sell, step);
        assert_eq!(sell, 88.88);
        assert_eq!(rounded, 89.0);
    }
    #[test]
    fn min_max_sum() {
        assert_eq!(8, max(2, 8));
        assert_eq!(8.0, max(2.0, 8.0));

        assert_eq!(2, min(2, 8));
        assert_eq!(2.0, min(2.0, 8.0));

        assert_eq!(10, sum(2, 8));
        assert_eq!(10.0, sum(2.0, 8.0));
    }
    #[test]
    fn bisect_l() {
        //                  0              5              10
        let s: [i32; 12] = [2, 3, 3, 3, 3, 3, 5, 8, 8, 8, 13, 15];
        let key = |x: &i32| *x;

        assert_eq!(bisect_left(&s, 0, key), None);
        assert_eq!(bisect_left(&s, 1, key), None);
        assert_eq!(bisect_left(&s, 2, key), Some(0));
        assert_eq!(bisect_left(&s, 3, key), Some(1));
        assert_eq!(bisect_left(&s, 4, key), Some(5));
        assert_eq!(bisect_left(&s, 5, key), Some(6));
        assert_eq!(bisect_left(&s, 6, key), Some(6));
        assert_eq!(bisect_left(&s, 7, key), Some(6));
        assert_eq!(bisect_left(&s, 8, key), Some(7));
        assert_eq!(bisect_left(&s, 9, key), Some(9));
        assert_eq!(bisect_left(&s, 10, key), Some(9));
        assert_eq!(bisect_left(&s, 11, key), Some(9));
        assert_eq!(bisect_left(&s, 12, key), Some(9));
        assert_eq!(bisect_left(&s, 13, key), Some(10));
        assert_eq!(bisect_left(&s, 14, key), Some(10));
        assert_eq!(bisect_left(&s, 15, key), Some(11));
        assert_eq!(bisect_left(&s, 16, key), Some(11));
        assert_eq!(bisect_left(&s, 100500, key), Some(11));
    }
    #[test]
    fn bisect_r() {
        //                  0              5              10
        let s: [i32; 12] = [2, 3, 3, 3, 3, 3, 5, 8, 8, 8, 13, 15];
        let key = |x: &i32| *x;

        assert_eq!(bisect_right(&s, 0, key), Some(0));
        assert_eq!(bisect_right(&s, 1, key), Some(0));
        assert_eq!(bisect_right(&s, 2, key), Some(0));
        assert_eq!(bisect_right(&s, 3, key), Some(1));
        assert_eq!(bisect_right(&s, 4, key), Some(6));
        assert_eq!(bisect_right(&s, 5, key), Some(6));
        assert_eq!(bisect_right(&s, 6, key), Some(7));
        assert_eq!(bisect_right(&s, 7, key), Some(7));
        assert_eq!(bisect_right(&s, 8, key), Some(7));
        assert_eq!(bisect_right(&s, 9, key), Some(10));
        assert_eq!(bisect_right(&s, 10, key), Some(10));
        assert_eq!(bisect_right(&s, 11, key), Some(10));
        assert_eq!(bisect_right(&s, 12, key), Some(10));
        assert_eq!(bisect_right(&s, 13, key), Some(10));
        assert_eq!(bisect_right(&s, 14, key), Some(11));
        assert_eq!(bisect_right(&s, 15, key), Some(11));
        assert_eq!(bisect_right(&s, 16, key), None);
        assert_eq!(bisect_right(&s, 100500, key), None);
    }
}
