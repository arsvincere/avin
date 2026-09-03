// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;
use std::str::FromStr;

use crate::DomainError;

/// Identifies a market data type supported by AVIN.
///
/// `MarketData` represents the kind and granularity of market data, such as
/// bars, ticks, or order book data.
///
/// Each market data type has a stable machine-readable key returned by
/// [`MarketData::key`]. Keys can be parsed case-insensitively using
/// [`FromStr`] and are intended for persistence and configuration.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use avin_domain::MarketData;
///
/// let market_data = MarketData::from_str("bar_5m").unwrap();
///
/// assert_eq!(market_data, MarketData::Bar5M);
/// assert_eq!(market_data.key(), "bar_5m");
/// assert_eq!(market_data.to_string(), "bar 5M");
/// ```
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

    /// Returns a stable machine-readable identifier suitable for persistence
    /// and serialization.
    pub const fn key(&self) -> &'static str {
        match self {
            Self::Bar1M => "bar_1m",
            Self::Bar5M => "bar_5m",
            Self::Bar10M => "bar_10m",
            Self::Bar15M => "bar_15m",
            Self::Bar1H => "bar_1h",
            Self::Bar4H => "bar_4h",
            Self::BarDay => "bar_day",
            Self::BarWeek => "bar_week",
            Self::BarMonth => "bar_month",
            Self::Tick => "tick",
            Self::OrderBook => "order_book",
        }
    }
}

impl Display for MarketData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bar1M => f.write_str("bar 1M"),
            Self::Bar5M => f.write_str("bar 5M"),
            Self::Bar10M => f.write_str("bar 10M"),
            Self::Bar15M => f.write_str("bar 15M"),
            Self::Bar1H => f.write_str("bar 1H"),
            Self::Bar4H => f.write_str("bar 4H"),
            Self::BarDay => f.write_str("bar D"),
            Self::BarWeek => f.write_str("bar W"),
            Self::BarMonth => f.write_str("bar M"),
            Self::Tick => f.write_str("ticks"),
            Self::OrderBook => f.write_str("order book"),
        }
    }
}

impl FromStr for MarketData {
    type Err = DomainError;

    /// Parses a market data key.
    ///
    /// Parsing is case-insensitive.
    ///
    /// # Errors
    ///
    /// Returns an error if the market data key is unknown.
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
            "bar_5m" => Ok(Self::Bar5M),
            "bar_10m" => Ok(Self::Bar10M),
            "bar_15m" => Ok(Self::Bar15M),
            "bar_1h" => Ok(Self::Bar1H),
            "bar_4h" => Ok(Self::Bar4H),
            "bar_day" => Ok(Self::BarDay),
            "bar_week" => Ok(Self::BarWeek),
            "bar_month" => Ok(Self::BarMonth),
            "tick" => Ok(Self::Tick),
            "order_book" => Ok(Self::OrderBook),
            _ => {
                let available = Self::all()
                    .iter()
                    .map(|md| md.key())
                    .collect::<Vec<_>>()
                    .join(", ");

                let msg = format!(
                    "unknown market data key '{}', available=[{}]",
                    s, available
                );

                Err(DomainError::Value(msg))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        assert_eq!(
            MarketData::all(),
            &[
                MarketData::Bar1M,
                MarketData::Bar5M,
                MarketData::Bar10M,
                MarketData::Bar15M,
                MarketData::Bar1H,
                MarketData::Bar4H,
                MarketData::BarDay,
                MarketData::BarWeek,
                MarketData::BarMonth,
                MarketData::Tick,
                MarketData::OrderBook,
            ]
        );
    }

    #[test]
    fn key() {
        assert_eq!(MarketData::Bar1M.key(), "bar_1m");
        assert_eq!(MarketData::Bar5M.key(), "bar_5m");
        assert_eq!(MarketData::Bar10M.key(), "bar_10m");
        assert_eq!(MarketData::Bar15M.key(), "bar_15m");
        assert_eq!(MarketData::Bar1H.key(), "bar_1h");
        assert_eq!(MarketData::Bar4H.key(), "bar_4h");
        assert_eq!(MarketData::BarDay.key(), "bar_day");
        assert_eq!(MarketData::BarWeek.key(), "bar_week");
        assert_eq!(MarketData::BarMonth.key(), "bar_month");
        assert_eq!(MarketData::Tick.key(), "tick");
        assert_eq!(MarketData::OrderBook.key(), "order_book");
    }

    #[test]
    fn display() {
        assert_eq!(MarketData::Bar1M.to_string(), "bar 1M");
        assert_eq!(MarketData::Bar5M.to_string(), "bar 5M");
        assert_eq!(MarketData::Bar10M.to_string(), "bar 10M");
        assert_eq!(MarketData::Bar15M.to_string(), "bar 15M");
        assert_eq!(MarketData::Bar1H.to_string(), "bar 1H");
        assert_eq!(MarketData::Bar4H.to_string(), "bar 4H");
        assert_eq!(MarketData::BarDay.to_string(), "bar D");
        assert_eq!(MarketData::BarWeek.to_string(), "bar W");
        assert_eq!(MarketData::BarMonth.to_string(), "bar M");
        assert_eq!(MarketData::Tick.to_string(), "ticks");
        assert_eq!(MarketData::OrderBook.to_string(), "order book");
    }

    #[test]
    fn from_str() {
        for market_data in MarketData::all().iter() {
            assert_eq!(
                MarketData::from_str(market_data.key()).unwrap(),
                *market_data
            );
        }

        assert_eq!(
            MarketData::from_str("bAr_1m").unwrap(),
            MarketData::Bar1M
        );

        assert!(matches!(
            MarketData::from_str("foo").unwrap_err(),
            DomainError::Value(_)
        ));
    }
}
