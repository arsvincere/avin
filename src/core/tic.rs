/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use crate::core::direction::Direction;
use bitcode::{Decode, Encode};
use chrono::{DateTime, Utc};

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

    pub fn dt(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_nanos(self.ts_nanos)
    }
}

impl std::fmt::Display for Tic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Tic: {} {} {}x{}={}",
            self.dt(),
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
    }
}
