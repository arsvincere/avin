// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;
use std::str::FromStr;

use avin_utils::AvinError;

use crate::{Exchange, InstrumentKind, Symbol};

/// Trading instrument identifier.
///
/// An instrument ID consists of an exchange, instrument kind, and symbol.
/// Its canonical text representation is `EXCHANGE.KIND.SYMBOL`.
///
/// The symbol may contain dots because only the first two dots separate
/// the instrument ID components.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use avin_model::{Exchange, InstrumentId, InstrumentKind, Symbol};
///
/// let iid = InstrumentId::new(
///     Exchange::MOEX,
///     InstrumentKind::Stock,
///     Symbol::new("SBER").unwrap(),
/// );
///
/// assert_eq!(iid.to_string(), "MOEX.Stock.SBER");
///
/// let iid = InstrumentId::from_str("MOEX.Stock.SBER").unwrap();
/// assert_eq!(iid.exchange(), Exchange::MOEX);
/// assert_eq!(iid.kind(), InstrumentKind::Stock);
/// assert_eq!(iid.symbol(), &Symbol::new("SBER").unwrap());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstrumentId {
    exchange: Exchange,
    kind: InstrumentKind,
    symbol: Symbol,
}

impl InstrumentId {
    /// Creates an instrument ID from its components.
    pub fn new(
        exchange: Exchange,
        kind: InstrumentKind,
        symbol: Symbol,
    ) -> Self {
        Self {
            exchange,
            kind,
            symbol,
        }
    }

    /// Returns the exchange.
    pub fn exchange(&self) -> Exchange {
        self.exchange
    }

    /// Returns the instrument kind.
    pub fn kind(&self) -> InstrumentKind {
        self.kind
    }

    /// Returns the instrument symbol.
    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }
}

impl Display for InstrumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.exchange, self.kind, self.symbol)
    }
}

impl FromStr for InstrumentId {
    type Err = AvinError;

    /// Parses an instrument ID from `EXCHANGE.KIND.SYMBOL`.
    ///
    /// Exchange and instrument kind parsing is case-insensitive.
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
    /// use avin_model::{Exchange, InstrumentId, InstrumentKind};
    ///
    /// let iid = InstrumentId::from_str("moex.stock.SBER").unwrap();
    ///
    /// assert_eq!(iid.exchange(), Exchange::MOEX);
    /// assert_eq!(iid.kind(), InstrumentKind::Stock);
    /// assert_eq!(iid.symbol().to_string(), "SBER");
    ///
    /// assert!(InstrumentId::from_str("MOEX.Stock").is_err());
    /// assert!(InstrumentId::from_str("foo.Stock.SBER").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(3, '.').collect();
        if parts.len() != 3 {
            let msg = format!("invalid instrument id '{s}'");
            return Err(AvinError::InvalidValue(msg));
        }

        let exchange = Exchange::from_str(parts[0])?;
        let kind = InstrumentKind::from_str(parts[1])?;
        let symbol = Symbol::from_str(parts[2])?;

        Ok(Self::new(exchange, kind, symbol))
    }
}
