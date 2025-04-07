/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::range::Range;
use bitcode::{Decode, Encode};
use chrono::prelude::*;
use polars::frame::DataFrame;
use std::error::Error;

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
pub struct Bar {
    pub ts_nanos: i64,
    pub o: f64,
    pub h: f64,
    pub l: f64,
    pub c: f64,
    pub v: u64,
}
impl Bar {
    pub fn display(&self) -> String {
        format!(
            "Bar: dt={} o={} h={} l={} c={} v={}",
            self.dt(),
            self.o,
            self.h,
            self.l,
            self.c,
            self.v
        )
    }

    pub fn dt(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_nanos(self.ts_nanos)
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

    pub fn new(
        ts_nanos: i64,
        o: f64,
        h: f64,
        l: f64,
        c: f64,
        v: u64,
    ) -> Result<Bar, Box<dyn Error>> {
        let bar = Bar {
            ts_nanos,
            o,
            h,
            l,
            c,
            v,
        };
        Ok(bar)
    }
    pub fn from_df(df: DataFrame) -> Result<Vec<Bar>, Box<dyn Error>> {
        let timestamp = df
            .column("ts_nanos")
            .unwrap()
            .i64()
            .unwrap()
            .into_no_null_iter();
        let mut open = df
            .column("open")
            .unwrap()
            .f64()
            .unwrap()
            .into_no_null_iter();
        let mut high = df
            .column("high")
            .unwrap()
            .f64()
            .unwrap()
            .into_no_null_iter();
        let mut low =
            df.column("low").unwrap().f64().unwrap().into_no_null_iter();
        let mut close = df
            .column("close")
            .unwrap()
            .f64()
            .unwrap()
            .into_no_null_iter();
        let mut volume = df
            .column("volume")
            .unwrap()
            .u64()
            .unwrap()
            .into_no_null_iter();

        let mut bars: Vec<Bar> = Vec::new();
        for ts in timestamp {
            let bar = Bar::new(
                ts,
                open.next().unwrap(),
                high.next().unwrap(),
                low.next().unwrap(),
                close.next().unwrap(),
                volume.next().unwrap(),
            )
            .unwrap();
            bars.push(bar);
        }

        return Ok(bars);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ohlcv() {
        let dt = Utc::now();
        let ts = dt.timestamp_nanos_opt().unwrap();
        let b = Bar::new(ts, 10.0, 11.1, 9.9, 10.5, 5000).unwrap();
        assert_eq!(b.dt(), dt);
        assert_eq!(b.o, 10.0);
        assert_eq!(b.h, 11.1);
        assert_eq!(b.l, 9.9);
        assert_eq!(b.c, 10.5);
        assert_eq!(b.v, 5000);
    }
    #[test]
    fn bear_bull() {
        let dt = Utc::now();
        let ts = dt.timestamp_nanos_opt().unwrap();
        let b = Bar::new(ts, 10.0, 11.1, 9.9, 10.5, 5000).unwrap();
        assert!(b.is_bull());
        assert!(!b.is_bear());

        let b = Bar::new(ts, 10.0, 11.1, 9.0, 9.5, 5000).unwrap();
        assert!(!b.is_bull());
        assert!(b.is_bear());
    }
}
