/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::DT_FMT;
use bitcode::{Decode, Encode};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct Transaction {
    pub ts_nanos: i64,
    pub quantity: i32,
    pub price: f64,
}
impl Transaction {
    pub fn new(ts_nanos: i64, quantity: i32, price: f64) -> Self {
        Transaction {
            ts_nanos,
            quantity,
            price,
        }
    }
    pub fn from_bin(bytes: &Vec<u8>) -> Self {
        bitcode::decode(bytes).unwrap()
    }
    pub fn from_csv(csv: &str) -> Self {
        let parts: Vec<&str> = csv.split(';').collect();

        let ts_nanos: i64 = parts[0].parse().unwrap();
        let quantity: i32 = parts[1].parse().unwrap();
        let price: f64 = parts[2].parse().unwrap();

        Transaction {
            ts_nanos,
            quantity,
            price,
        }
    }
    pub fn to_bin(&self) -> Vec<u8> {
        bitcode::encode(self)
    }
    pub fn to_csv(&self) -> String {
        format!("{};{};{};", self.ts_nanos, self.quantity, self.price,)
    }
    pub fn to_hash_map(&self) -> HashMap<&str, String> {
        let mut hm = HashMap::new();
        hm.insert("ts_nanos", self.ts_nanos.to_string());
        hm.insert("quantity", self.quantity.to_string());
        hm.insert("price", self.price.to_string());

        hm
    }

    pub fn dt(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_nanos(self.ts_nanos)
    }
    pub fn value(&self) -> f64 {
        self.price * self.quantity as f64
    }
}
impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let formatted = format!("{}", self.dt().format(DT_FMT));
        write!(
            f,
            "Transaction={} {}x{}",
            formatted, self.quantity, self.price
        )
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn new() {
        let dt = Utc::now();
        let ts = dt.timestamp_nanos_opt().unwrap();

        let t = Transaction::new(ts, 10, 325.5);
        assert_eq!(t.dt(), dt);
        assert_eq!(t.quantity, 10);
        assert_eq!(t.price, 325.5);
    }
    #[test]
    fn csv() {
        let dt = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let ts = dt.timestamp_nanos_opt().unwrap();

        let t = Transaction::new(ts, 10, 325.5);
        let csv = t.to_csv();
        assert_eq!(csv, "1735689600000000000;10;325.5;");

        let from_csv = Transaction::from_csv(&csv);
        assert_eq!(t, from_csv);
    }
    #[test]
    fn bin() {
        let dt = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let ts = dt.timestamp_nanos_opt().unwrap();

        let t = Transaction::new(ts, 10, 325.5);
        let bytes = t.to_bin();

        let decoded = Transaction::from_bin(&bytes);
        assert_eq!(t, decoded);
    }
}
