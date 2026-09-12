// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum CoreError {
    Time(String),
    TimeRange(String),
    Year(String),
    Price(String),
    PriceRange(String),
    Direction(String),
    Quantity(String),
}

impl Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Time(msg) => f.write_str(msg),
            Self::TimeRange(msg) => f.write_str(msg),
            Self::Year(msg) => f.write_str(msg),
            Self::Price(msg) => f.write_str(msg),
            Self::PriceRange(msg) => f.write_str(msg),
            Self::Direction(msg) => f.write_str(msg),
            Self::Quantity(msg) => f.write_str(msg),
        }
    }
}

impl Error for CoreError {}
