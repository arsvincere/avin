/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use bitcode::{Decode, Encode};

/// Order and transaction direction.
///
/// # ru
/// Направление сделки, перечисление используется в ордерах [`crate::Order`]
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub enum Direction {
    Buy,
    Sell,
}
impl Direction {
    pub fn to_str(&self) -> &'static str {
        match self {
            Direction::Buy => "b",
            Direction::Sell => "s",
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Buy => write!(f, "Buy"),
            Self::Sell => write!(f, "Sell"),
        }
    }
}
impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "b" => Direction::Buy,
            "s" => Direction::Sell,
            "B" => Direction::Buy,
            "S" => Direction::Sell,
            _ => panic!("Invalid value for direction: {value}"),
        }
    }
}
