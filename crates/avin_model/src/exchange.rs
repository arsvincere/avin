// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_utils::AvinError;

/// Exchange.
///
/// Represents an exchange supported by AVIN.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use avin_model::Exchange;
///
/// // Parsing is case-insensitive.
/// let exchange = Exchange::from_str("moex").unwrap();
/// assert_eq!(exchange, Exchange::MOEX);
/// assert_eq!(exchange.name(), "MOEX");
///
/// for e in Exchange::all() {
///     println!("{e}");
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exchange {
    Binance,
    Bybit,
    MOEX,
    SPB,
}

impl Exchange {
    /// Returns all supported exchanges.
    pub const fn all() -> &'static [Self] {
        &[Self::Binance, Self::Bybit, Self::MOEX, Self::SPB]
    }

    /// Returns the canonical exchange name.
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Binance => "Binance",
            Self::Bybit => "Bybit",
            Self::MOEX => "MOEX",
            Self::SPB => "SPB",
        }
    }
}

impl std::str::FromStr for Exchange {
    type Err = AvinError;

    /// Parses an exchange name.
    ///
    /// Parsing is case-insensitive.
    ///
    /// # Errors
    ///
    /// Returns an error if the exchange name is unknown.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use avin_model::Exchange;
    ///
    /// assert_eq!(Exchange::from_str("BiNaNcE").unwrap(), Exchange::Binance);
    /// assert!(Exchange::from_str("foo").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "binance" => Ok(Self::Binance),
            "bybit" => Ok(Self::Bybit),
            "moex" => Ok(Self::MOEX),
            "spb" => Ok(Self::SPB),
            _ => {
                let available = Self::all()
                    .iter()
                    .map(Self::name)
                    .collect::<Vec<_>>()
                    .join(", ");

                let msg = format!(
                    "unknown exchange '{}', available=[{}]",
                    s, available
                );

                Err(AvinError::InvalidValue(msg))
            }
        }
    }
}

impl std::fmt::Display for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
