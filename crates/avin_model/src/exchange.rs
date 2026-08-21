// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;
use std::str::FromStr;

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
/// for exchange in Exchange::all() {
///     println!("{exchange}");
/// }
///
/// // Parsing is case-insensitive.
/// let exchange = Exchange::from_str("moex").unwrap();
/// assert_eq!(exchange, Exchange::MOEX);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    // TODO: а этот метод вообще нужен? Кому нужен?
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

impl Display for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for Exchange {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        let expected = [
            Exchange::Binance,
            Exchange::Bybit,
            Exchange::MOEX,
            Exchange::SPB,
        ];

        assert_eq!(Exchange::all(), expected);
    }

    #[test]
    fn name() {
        assert_eq!(Exchange::Binance.name(), "Binance");
        assert_eq!(Exchange::Bybit.name(), "Bybit");
        assert_eq!(Exchange::MOEX.name(), "MOEX");
        assert_eq!(Exchange::SPB.name(), "SPB");
    }

    #[test]
    fn display() {
        assert_eq!(Exchange::Binance.to_string(), "Binance");
        assert_eq!(Exchange::Bybit.to_string(), "Bybit");
        assert_eq!(Exchange::MOEX.to_string(), "MOEX");
        assert_eq!(Exchange::SPB.to_string(), "SPB");
    }

    #[test]
    fn from_str() {
        assert_eq!(Exchange::from_str("BInaNce").unwrap(), Exchange::Binance);
        assert_eq!(Exchange::from_str("Bybit").unwrap(), Exchange::Bybit);
        assert_eq!(Exchange::from_str("MoEx").unwrap(), Exchange::MOEX);
        assert_eq!(Exchange::from_str("SPB").unwrap(), Exchange::SPB);

        assert!(Exchange::from_str("foo").is_err());
    }
}
