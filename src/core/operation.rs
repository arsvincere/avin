/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::conf::DT_FMT;
use crate::core::transaction::Transaction;
use bitcode::{Decode, Encode};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
pub struct Operation {
    pub ts_nanos: i64,
    pub quantity: i32,
    pub value: f64,
    pub commission: f64,
}
impl Operation {
    pub fn new(
        ts_nanos: i64,
        quantity: i32,
        value: f64,
        commission: f64,
    ) -> Self {
        Self {
            ts_nanos,
            quantity,
            value,
            commission,
        }
    }
    pub fn from(
        ts_nanos: i64,
        transactions: &Vec<Transaction>,
        commission: f64,
    ) -> Self {
        if transactions.is_empty() {
            panic!("Empty transactions list! Fail to create operation!");
        }

        let mut quantity: i32 = 0;
        let mut value: f64 = 0.0;
        for i in transactions.iter() {
            quantity += i.quantity;
            value += i.quantity as f64 * i.price;
        }

        Self {
            ts_nanos,
            quantity,
            value,
            commission,
        }
    }
    pub fn from_bin(bytes: &Vec<u8>) -> Self {
        bitcode::decode(bytes).unwrap()
    }
    pub fn from_csv(csv: &str) -> Self {
        let parts: Vec<&str> = csv.split(';').collect();

        let ts_nanos: i64 = parts[0].parse().unwrap();
        let quantity: i32 = parts[1].parse().unwrap();
        let value: f64 = parts[2].parse().unwrap();
        let commission: f64 = parts[3].parse().unwrap();

        Operation {
            ts_nanos,
            quantity,
            value,
            commission,
        }
    }
    pub fn to_bin(&self) -> Vec<u8> {
        bitcode::encode(self)
    }
    pub fn to_csv(&self) -> String {
        format!(
            "{};{};{};{};",
            self.ts_nanos, self.quantity, self.value, self.commission
        )
    }
    pub fn to_hash_map(&self) -> HashMap<&str, String> {
        let mut info = HashMap::new();
        info.insert("ts_nanos", self.ts_nanos.to_string());
        info.insert("quantity", self.quantity.to_string());
        info.insert("value", self.value.to_string());
        info.insert("commission", self.commission.to_string());

        info
    }

    pub fn dt(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_nanos(self.ts_nanos)
    }
    pub fn avg_price(&self) -> f64 {
        self.value / self.quantity as f64
    }
}
impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let dt = format!("{}", self.dt().format(DT_FMT));
        write!(
            f,
            "Operation={} {}={}+{}",
            dt, self.quantity, self.value, self.commission
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn new() {
        let ts = 100500;
        let t1 = Transaction::new(10, 320.0);
        let t2 = Transaction::new(10, 330.0);

        let op = Operation::from(ts, &vec![t1, t2], 6500.0 * 0.001);
        assert_eq!(op.ts_nanos, ts);
        assert_eq!(op.quantity, 20);
        assert_eq!(op.value, 6500.0);
        assert_eq!(op.commission, 6.5);
        assert_eq!(op.avg_price(), 325.0);
    }
    #[test]
    fn csv() {
        let t1 = Transaction::new(10, 320.0);

        let dt = Utc.with_ymd_and_hms(2025, 4, 6, 12, 19, 0).unwrap();
        let ts = dt.timestamp_nanos_opt().unwrap();
        let op = Operation::from(ts, &vec![t1], 320.0 * 10.0 * 0.0005);

        let csv = op.to_csv();
        assert_eq!(csv, "1743941940000000000;10;3200;1.6;");

        let from_csv = Operation::from_csv(&csv);
        assert_eq!(op, from_csv);
    }
    #[test]
    fn bin() {
        let t1 = Transaction::new(10, 320.0);

        let dt = Utc.with_ymd_and_hms(2025, 4, 6, 12, 19, 0).unwrap();
        let ts = dt.timestamp_nanos_opt().unwrap();
        let op = Operation::from(ts, &vec![t1], 320.0 * 10.0 * 0.0005);

        let bytes = op.to_bin();
        let decoded = Operation::from_bin(&bytes);
        assert_eq!(op, decoded);
    }
}
