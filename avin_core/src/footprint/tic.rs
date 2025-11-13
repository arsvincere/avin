/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use bitcode::{Decode, Encode};
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use polars::prelude::{DataFrame, DataType, Field, Schema, df};

use avin_utils::AvinError;

use crate::Direction;

/// One buy/sell deal in market data stream.
///
/// # ru
/// Одна конкретная сделка покупки или продажи бумаги в потоке рыночных
/// данных.
///
/// Содержит временную метку, направление сделки, количество лотов, цену,
/// и сумму = количество лотов * количество бумаг в лоте * цену.
///
/// Направление сделки считается направление рыночного ордера,
/// приведшего к сделке.
#[derive(Debug, PartialEq, Encode, Decode, Clone)]
pub struct Tic {
    pub ts: i64,
    pub direction: Direction,
    pub lots: i64,
    pub price: f64,
    pub value: f64,
}
impl Tic {
    pub fn new(
        ts: i64,
        direction: Direction,
        lots: i64,
        price: f64,
        value: f64,
    ) -> Self {
        Tic {
            ts,
            direction,
            lots,
            price,
            value,
        }
    }
    /// Polars dataframe schema for tics.
    ///
    /// # ru
    /// Возвращает polars схему датафрейма для тиков.
    pub fn schema() -> Schema {
        Schema::from_iter(vec![
            Field::new("ts_nanos".into(), DataType::Int64),
            Field::new("direction".into(), DataType::String),
            Field::new("lots".into(), DataType::Int64),
            Field::new("price".into(), DataType::Float64),
            Field::new("value".into(), DataType::Float64),
            Field::new("session".into(), DataType::Int8),
            Field::new("tradeno".into(), DataType::Int64),
        ])
    }
    pub fn from_df(df: &DataFrame) -> Result<Vec<Tic>, AvinError> {
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
                Direction::from(direction.next().unwrap()),
                lots.next().unwrap(),
                price.next().unwrap(),
                value.next().unwrap(),
            );
            tics.push(tic);
        }

        Ok(tics)
    }

    pub fn df(&self) -> DataFrame {
        df!(
            "ts_nanos" => [self.ts],
            "direction" => [self.direction.to_str()],
            "lots" => [self.lots],
            "price" => [self.price],
            "value" => [self.value],
            "session" => [None::<i8>],
            "tradeno" => [None::<i64>],
        )
        .unwrap()
    }
    pub fn dt(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_nanos(self.ts)
    }
    pub fn dt_local(&self) -> NaiveDateTime {
        let utc = DateTime::from_timestamp_nanos(self.ts);
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

    #[test]
    fn schema() {
        // тест проверяет согласованность схемы датафрейма в функцииях
        // Tic::schema() и tic.df()
        // чтобы если изменил в одном месте не забыть изменить в другом

        let ts = 100500;
        let direction = Direction::Buy;
        let lots = 10;
        let price = 320.5;
        let value = 10.0 * 320.5;
        let tic = Tic::new(ts, direction, lots, price, value);
        let df = tic.df();

        let schema = Tic::schema();
        let mut tic_data = DataFrame::empty_with_schema(&schema);
        tic_data.extend(&df).unwrap();
    }
}
