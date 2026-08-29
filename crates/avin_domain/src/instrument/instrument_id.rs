// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;
use std::str::FromStr;

use avin_utils::AvinError;

use crate::{Category, Exchange, Ticker};

/// Canonical instrument identifier used by AVIN.
///
/// An `InstrumentId` combines an exchange, category, and ticker into
/// a compact, human-readable form such as `MOEX.SHARE.SBER`.
///
/// Unlike external identifiers such as FIGI, ISIN, or provider-specific UIDs,
/// it can be interpreted directly by a person.
///
/// Its canonical text representation is `EXCHANGE.CATEGORY.TICKER`.
/// The ticker may contain dots because only the first two dots separate
/// the identifier components.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use avin_domain::{Exchange, InstrumentId, Category, Ticker};
///
/// let iid = InstrumentId::new(
///     Exchange::Moex,
///     Category::Share,
///     Ticker::new("SBER").unwrap(),
/// );
///
/// assert_eq!(iid.to_string(), "MOEX.SHARE.SBER");
///
/// let iid = InstrumentId::from_str("MOEX.SHARE.SBER").unwrap();
/// assert_eq!(iid.exchange(), Exchange::Moex);
/// assert_eq!(iid.category(), Category::Share);
/// assert_eq!(iid.ticker(), &Ticker::new("SBER").unwrap());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstrumentId {
    exchange: Exchange,
    category: Category,
    ticker: Ticker,
}

impl InstrumentId {
    /// Creates an instrument ID from its components.
    pub fn new(
        exchange: Exchange,
        category: Category,
        ticker: Ticker,
    ) -> Self {
        Self {
            exchange,
            category,
            ticker,
        }
    }

    /// Returns the exchange.
    pub fn exchange(&self) -> Exchange {
        self.exchange
    }

    /// Returns the category.
    pub fn category(&self) -> Category {
        self.category
    }

    /// Returns the instrument ticker.
    pub fn ticker(&self) -> &Ticker {
        &self.ticker
    }
}

impl Display for InstrumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.exchange, self.category, self.ticker)
    }
}

impl FromStr for InstrumentId {
    type Err = AvinError;

    /// Parses an instrument ID from `EXCHANGE.CATEGORY.TICKER`.
    ///
    /// Exchange and category parsing is case-insensitive.
    ///
    /// # Errors
    ///
    /// Returns an error if the input does not contain all three components
    /// or if any component is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use avin_domain::{Exchange, InstrumentId, Category};
    ///
    /// let iid = InstrumentId::from_str("moex.SHARE.SBER").unwrap();
    ///
    /// assert_eq!(iid.exchange(), Exchange::Moex);
    /// assert_eq!(iid.category(), Category::Share);
    /// assert_eq!(iid.ticker().to_string(), "SBER");
    ///
    /// assert!(InstrumentId::from_str("MOEX.SHARE").is_err());
    /// assert!(InstrumentId::from_str("foo.SHARE.SBER").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(3, '.').collect();
        if parts.len() != 3 {
            let msg = format!("invalid instrument id '{s}'");
            return Err(AvinError::Value(msg));
        }

        let exchange = Exchange::from_str(parts[0])?;
        let category = Category::from_str(parts[1])?;
        let ticker = Ticker::from_str(parts[2])?;

        Ok(Self::new(exchange, category, ticker))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instrument_id() {
        let iid = InstrumentId::new(
            Exchange::Moex,
            Category::Share,
            Ticker::new("SBER").unwrap(),
        );

        assert_eq!(iid.exchange(), Exchange::Moex);
        assert_eq!(iid.category(), Category::Share);
        assert_eq!(iid.ticker(), &Ticker::new("SBER").unwrap());
    }

    #[test]
    fn display() {
        let iid = InstrumentId::new(
            Exchange::Moex,
            Category::Share,
            Ticker::new("SBER").unwrap(),
        );

        assert_eq!(iid.to_string(), "MOEX.SHARE.SBER");
    }

    #[test]
    fn from_str() {
        let iid = InstrumentId::from_str("moex.SHARE.SBER").unwrap();

        assert_eq!(iid.exchange(), Exchange::Moex);
        assert_eq!(iid.category(), Category::Share);
        assert_eq!(iid.ticker(), &Ticker::new("SBER").unwrap());
    }

    #[test]
    fn ticker_with_dots() {
        let iid = InstrumentId::from_str("moex.SHARE.BRK.B").unwrap();

        assert_eq!(iid.ticker(), &Ticker::new("BRK.B").unwrap());
    }

    #[test]
    fn invalid_id() {
        assert!(InstrumentId::from_str("MOEX.SHARE").is_err());
        assert!(InstrumentId::from_str("foo.SHARE.SBER").is_err());
        assert!(InstrumentId::from_str("MOEX.foo.SBER").is_err());
        assert!(InstrumentId::from_str("MOEX.SHARE.").is_err());
    }
}
