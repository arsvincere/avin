// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;
use std::str::FromStr;

use avin_utils::AvinError;

use crate::{Category, Exchange, Symbol};

/// Canonical instrument identifier used by AVIN.
///
/// An `InstrumentId` combines an exchange, category, and symbol into
/// a compact, human-readable form such as `MOEX.SHARE.SBER`.
///
/// Unlike external identifiers such as FIGI, ISIN, or provider-specific UIDs,
/// it can be interpreted directly by a person.
///
/// Its canonical text representation is `EXCHANGE.CATEGORY.SYMBOL`.
/// The symbol may contain dots because only the first two dots separate
/// the identifier components.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use avin_domain::{Exchange, InstrumentId, Category, Symbol};
///
/// let iid = InstrumentId::new(
///     Exchange::MOEX,
///     Category::Share,
///     Symbol::new("SBER").unwrap(),
/// );
///
/// assert_eq!(iid.to_string(), "MOEX.SHARE.SBER");
///
/// let iid = InstrumentId::from_str("MOEX.SHARE.SBER").unwrap();
/// assert_eq!(iid.exchange(), Exchange::MOEX);
/// assert_eq!(iid.category(), Category::Share);
/// assert_eq!(iid.symbol(), &Symbol::new("SBER").unwrap());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstrumentId {
    exchange: Exchange,
    category: Category,
    symbol: Symbol,
}

impl InstrumentId {
    /// Creates an instrument ID from its components.
    pub fn new(
        exchange: Exchange,
        category: Category,
        symbol: Symbol,
    ) -> Self {
        Self {
            exchange,
            category,
            symbol,
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

    /// Returns the instrument symbol.
    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }
}

impl Display for InstrumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.exchange, self.category, self.symbol)
    }
}

impl FromStr for InstrumentId {
    type Err = AvinError;

    /// Parses an instrument ID from `EXCHANGE.CATEGORY.SYMBOL`.
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
    /// assert_eq!(iid.exchange(), Exchange::MOEX);
    /// assert_eq!(iid.category(), Category::Share);
    /// assert_eq!(iid.symbol().to_string(), "SBER");
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
        let symbol = Symbol::from_str(parts[2])?;

        Ok(Self::new(exchange, category, symbol))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instrument_id() {
        let iid = InstrumentId::new(
            Exchange::MOEX,
            Category::Share,
            Symbol::new("SBER").unwrap(),
        );

        assert_eq!(iid.exchange(), Exchange::MOEX);
        assert_eq!(iid.category(), Category::Share);
        assert_eq!(iid.symbol(), &Symbol::new("SBER").unwrap());
    }

    #[test]
    fn display() {
        let iid = InstrumentId::new(
            Exchange::MOEX,
            Category::Share,
            Symbol::new("SBER").unwrap(),
        );

        assert_eq!(iid.to_string(), "MOEX.SHARE.SBER");
    }

    #[test]
    fn from_str() {
        let iid = InstrumentId::from_str("moex.SHARE.SBER").unwrap();

        assert_eq!(iid.exchange(), Exchange::MOEX);
        assert_eq!(iid.category(), Category::Share);
        assert_eq!(iid.symbol(), &Symbol::new("SBER").unwrap());
    }

    #[test]
    fn symbol_with_dots() {
        let iid = InstrumentId::from_str("moex.SHARE.BRK.B").unwrap();

        assert_eq!(iid.symbol(), &Symbol::new("BRK.B").unwrap());
    }

    #[test]
    fn invalid_id() {
        assert!(InstrumentId::from_str("MOEX.SHARE").is_err());
        assert!(InstrumentId::from_str("foo.SHARE.SBER").is_err());
        assert!(InstrumentId::from_str("MOEX.foo.SBER").is_err());
        assert!(InstrumentId::from_str("MOEX.SHARE.").is_err());
    }
}
