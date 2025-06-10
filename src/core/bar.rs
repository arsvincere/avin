/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::{core::range::Range, utils};
use bitcode::{Decode, Encode};
use chrono::prelude::*;
use polars::frame::DataFrame;

#[derive(Debug, Clone, Copy, PartialEq, Encode, Decode)]
pub struct Bar {
    pub ts_nanos: i64,
    pub o: f64,
    pub h: f64,
    pub l: f64,
    pub c: f64,
    pub v: u64,
    pub val: Option<f64>,
}
impl Bar {
    pub fn new(
        ts_nanos: i64,
        o: f64,
        h: f64,
        l: f64,
        c: f64,
        v: u64,
        val: Option<f64>,
    ) -> Bar {
        Bar {
            ts_nanos,
            o,
            h,
            l,
            c,
            v,
            val,
        }
    }
    pub fn from_df(df: DataFrame) -> Result<Vec<Bar>, String> {
        let ts = df
            .column("ts_nanos")
            .unwrap()
            .i64()
            .unwrap()
            .into_no_null_iter();
        let mut o = df
            .column("open")
            .unwrap()
            .f64()
            .unwrap()
            .into_no_null_iter();
        let mut h = df
            .column("high")
            .unwrap()
            .f64()
            .unwrap()
            .into_no_null_iter();
        let mut l =
            df.column("low").unwrap().f64().unwrap().into_no_null_iter();
        let mut c = df
            .column("close")
            .unwrap()
            .f64()
            .unwrap()
            .into_no_null_iter();
        let mut v = df
            .column("volume")
            .unwrap()
            .i64()
            .unwrap()
            .into_no_null_iter();
        let mut val = df
            .column("value")
            .unwrap()
            .f64()
            .unwrap()
            .into_no_null_iter();

        let mut bars: Vec<Bar> = Vec::with_capacity(df.height());
        for t in ts {
            let bar = Bar::new(
                t,
                o.next().unwrap(),
                h.next().unwrap(),
                l.next().unwrap(),
                c.next().unwrap(),
                v.next().unwrap() as u64,
                Some(val.next().unwrap()),
            );
            bars.push(bar);
        }

        return Ok(bars);
    }

    pub fn dt(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_nanos(self.ts_nanos)
    }
    pub fn dt_local(&self) -> NaiveDateTime {
        let utc = DateTime::from_timestamp_nanos(self.ts_nanos);
        let local: DateTime<Local> = DateTime::from(utc);

        local.naive_local()
    }
    pub fn is_bear(&self) -> bool {
        self.o > self.c
    }
    pub fn is_bull(&self) -> bool {
        self.o < self.c
    }
    pub fn full(&self) -> Range {
        Range::new(self.l, self.h)
    }
    pub fn body(&self) -> Range {
        Range::new(self.o, self.c)
    }
    pub fn lower(&self) -> Range {
        if self.is_bull() {
            Range::new(self.l, self.o)
        } else {
            Range::new(self.l, self.c)
        }
    }
    pub fn upper(&self) -> Range {
        if self.is_bull() {
            Range::new(self.c, self.h)
        } else {
            Range::new(self.o, self.h)
        }
    }
    pub fn contains(&self, price: f64) -> bool {
        self.l <= price && price <= self.h
    }
    pub fn join(&self, other: Bar) -> Bar {
        Bar {
            ts_nanos: self.ts_nanos,
            o: self.o,
            h: utils::max(self.h, other.h),
            l: utils::min(self.l, other.l),
            c: other.c,
            v: utils::sum(self.v, other.v),
            val: Some(utils::sum(self.val.unwrap(), other.val.unwrap())),
        }
    }
}
impl std::fmt::Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Bar: dt={} o={} h={} l={} c={} v={}",
            self.dt_local(),
            self.o,
            self.h,
            self.l,
            self.c,
            self.v
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ohlcv() {
        let dt = Utc::now();
        let ts = dt.timestamp_nanos_opt().unwrap();
        let b = Bar::new(ts, 10.0, 11.1, 9.9, 10.5, 5000, Some(50_000.0));
        assert_eq!(b.dt(), dt);
        assert_eq!(b.o, 10.0);
        assert_eq!(b.h, 11.1);
        assert_eq!(b.l, 9.9);
        assert_eq!(b.c, 10.5);
        assert_eq!(b.v, 5000);
        assert_eq!(b.val, Some(50_000.0));
    }
    #[test]
    fn bear_bull() {
        let dt = Utc::now();
        let ts = dt.timestamp_nanos_opt().unwrap();
        let b = Bar::new(ts, 10.0, 11.1, 9.9, 10.5, 5000, Some(50_000.0));
        assert!(b.is_bull());
        assert!(!b.is_bear());

        let b = Bar::new(ts, 10.0, 11.1, 9.0, 9.5, 5000, Some(50_000.0));
        assert!(!b.is_bull());
        assert!(b.is_bear());
    }
    #[test]
    fn join() {
        let b1 = Bar::new(1, 100.0, 101.0, 99.0, 100.5, 5000, Some(50_000.0));
        let b2 = Bar::new(2, 100.5, 101.2, 99.7, 100.8, 4000, Some(40_000.0));

        let bar = b1.join(b2);
        assert_eq!(bar.ts_nanos, 1);
        assert_eq!(bar.o, 100.0);
        assert_eq!(bar.h, 101.2);
        assert_eq!(bar.l, 99.0);
        assert_eq!(bar.c, 100.8);
        assert_eq!(bar.v, 9000);
        assert_eq!(bar.val, Some(90_000.0));
    }
}
