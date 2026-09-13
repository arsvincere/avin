// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: docs, tests

use std::fmt::Display;
use std::str::FromStr;

use crate::CoreError;

pub enum Direction {
    Buy,
    Sell,
}

impl Direction {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Buy => "b",
            Self::Sell => "s",
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Buy => f.write_str("buy"),
            Self::Sell => f.write_str("sell"),
        }
    }
}

impl FromStr for Direction {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "b" => Ok(Self::Buy),
            "s" => Ok(Self::Sell),
            _ => {
                let msg = format!(
                    "unknown direction key '{}', available=[{}, {}]",
                    s,
                    Self::Buy.key(),
                    Self::Sell.key()
                );

                Err(CoreError::Direction(msg))
            }
        }
    }
}
