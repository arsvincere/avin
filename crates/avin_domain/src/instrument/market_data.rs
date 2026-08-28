// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;
use std::str::FromStr;

use avin_utils::AvinError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarketData {
    Bar1M,
    Bar5M,
    Bar10M,
    Bar15M,
    Bar1H,
    Bar4H,
    BarDay,
    BarWeek,
    BarMonth,

    Tick,
    OrderBook,
}

impl MarketData {
    /// Returns all supported market data types.
    pub const fn all() -> &'static [Self] {
        &[
            Self::Bar1M,
            Self::Bar5M,
            Self::Bar10M,
            Self::Bar15M,
            Self::Bar1H,
            Self::Bar4H,
            Self::BarDay,
            Self::BarWeek,
            Self::BarMonth,
            Self::Tick,
            Self::OrderBook,
        ]
    }
}

impl Display for MarketData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bar1M => f.write_str("bar_1m"),
            Self::Bar5M => f.write_str("bar_5m"),
            Self::Bar10M => f.write_str("bar_10m"),
            Self::Bar15M => f.write_str("bar_15m"),
            Self::Bar1H => f.write_str("bar_1h"),
            Self::Bar4H => f.write_str("bar_4h"),
            Self::BarDay => f.write_str("bar_day"),
            Self::BarWeek => f.write_str("bar_week"),
            Self::BarMonth => f.write_str("bar_month"),
            Self::Tick => f.write_str("tick"),
            Self::OrderBook => f.write_str("order_book"),
        }
    }
}

impl FromStr for MarketData {
    type Err = AvinError;

    /// Parses an market data name.
    ///
    /// Parsing is case-insensitive.
    ///
    /// # Errors
    ///
    /// Returns an error if the market data name is unknown.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use avin_domain::MarketData;
    ///
    /// assert_eq!(MarketData::from_str("BAR_1M").unwrap(), MarketData::Bar1M);
    /// assert!(MarketData::from_str("foo").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "bar_1m" => Ok(Self::Bar1M),
            "bar_5m" => Ok(Self::Bar1M),
            "bar_10m" => Ok(Self::Bar1M),
            "bar_15m" => Ok(Self::Bar1M),
            "bar_1h" => Ok(Self::Bar1M),
            "bar_4h" => Ok(Self::Bar1M),
            "bar_day" => Ok(Self::Bar1M),
            "bar_week" => Ok(Self::Bar1M),
            "bar_month" => Ok(Self::Bar1M),
            "tick" => Ok(Self::Bar1M),
            "order_book" => Ok(Self::Bar1M),
            _ => {
                let available = Self::all()
                    .iter()
                    .map(Self::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");

                let msg = format!(
                    "unknown market data '{}', available=[{}]",
                    s, available
                );

                Err(AvinError::Value(msg))
            }
        }
    }
}
