/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::direction::Direction;
use bitcode::{Decode, Encode};
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use polars::frame::DataFrame;

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
pub struct Tic {
    pub ts_nanos: i64,
    pub direction: Direction,
    pub lots: u32,
    pub price: f64,
    pub value: f64,
}
impl Tic {
    pub fn new(
        ts_nanos: i64,
        direction: Direction,
        lots: u32,
        price: f64,
        value: f64,
    ) -> Self {
        Tic {
            ts_nanos,
            direction,
            lots,
            price,
            value,
        }
    }
    pub fn from_df(df: &DataFrame) -> Result<Vec<Tic>, String> {
        let ts_nanos = df
            .column("ts_nanos")
            .unwrap()
            .i64()
            .unwrap()
            .into_no_null_iter();
        let mut direction = df
            .column("direction")
            .unwrap()
            .str()
            .unwrap()
            .into_no_null_iter();
        let mut lots = df
            .column("lots")
            .unwrap()
            .i64()
            .unwrap()
            .into_no_null_iter();
        let mut price = df
            .column("price")
            .unwrap()
            .f64()
            .unwrap()
            .into_no_null_iter();
        let mut value = df
            .column("value")
            .unwrap()
            .f64()
            .unwrap()
            .into_no_null_iter();

        let mut tics: Vec<Tic> = Vec::with_capacity(df.height());
        for ts in ts_nanos {
            let tic = Tic::new(
                ts,
                Direction::from_str(direction.next().unwrap()).unwrap(),
                lots.next().unwrap() as u32,
                price.next().unwrap(),
                value.next().unwrap(),
            );
            tics.push(tic);
        }

        return Ok(tics);
    }

    pub fn dt(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_nanos(self.ts_nanos)
    }
    pub fn dt_local(&self) -> NaiveDateTime {
        let utc = DateTime::from_timestamp_nanos(self.ts_nanos);
        let local: DateTime<Local> = DateTime::from(utc);

        local.naive_local()
    }
    pub fn is_buy(&self) -> bool {
        self.direction == Direction::Buy
    }
    pub fn is_sell(&self) -> bool {
        self.direction == Direction::Sell
    }
}
impl std::fmt::Display for Tic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Tic: {} {} {}x{}={}",
            self.dt_local(),
            self.direction.to_str(),
            self.lots,
            self.price,
            self.value,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let tic = Tic::new(100500, Direction::Buy, 10, 300.0, 3000.0);
        assert_eq!(tic.direction, Direction::Buy);
        assert_eq!(tic.lots, 10);
        assert_eq!(tic.price, 300.0);
        assert_eq!(tic.value, 3000.0);

        assert!(tic.is_buy());
        assert!(!tic.is_sell());
    }
}
