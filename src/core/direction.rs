/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use bitcode::{Decode, Encode};

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
