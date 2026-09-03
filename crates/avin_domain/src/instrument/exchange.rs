// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;
use std::str::FromStr;

use crate::DomainError;

/// Identifies an exchange supported by AVIN.
///
/// Each exchange has a stable machine-readable key returned by
/// [`Exchange::key`]. Keys can be parsed case-insensitively using [`FromStr`]
/// and are intended for persistence and configuration.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use avin_domain::Exchange;
///
/// for exchange in Exchange::all() {
///     println!("{exchange}");
/// }
///
/// let exchange = Exchange::from_str("moex").unwrap();
///
/// assert_eq!(exchange, Exchange::Moex);
/// assert_eq!(exchange.key(), "moex");
/// assert_eq!(exchange.to_string(), "MOEX");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Exchange {
    Binance,
    Bybit,
    Moex,
    Spb,
}

impl Exchange {
    /// Returns all supported exchanges.
    pub const fn all() -> &'static [Self] {
        &[Self::Binance, Self::Bybit, Self::Moex, Self::Spb]
    }

    /// Returns a stable machine-readable identifier suitable for persistence
    /// and serialization.
    pub const fn key(&self) -> &'static str {
        match self {
            Self::Binance => "binance",
            Self::Bybit => "bybit",
            Self::Moex => "moex",
            Self::Spb => "spb",
        }
    }
}

impl Display for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binance => f.write_str("Binance"),
            Self::Bybit => f.write_str("Bybit"),
            Self::Moex => f.write_str("MOEX"),
            Self::Spb => f.write_str("SPB"),
        }
    }
}

impl FromStr for Exchange {
    type Err = DomainError;

    /// Parses an exchange key.
    ///
    /// Parsing is case-insensitive.
    ///
    /// # Errors
    ///
    /// Returns an error if the exchange key is unknown.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use avin_domain::Exchange;
    ///
    /// assert_eq!(Exchange::from_str("BiNaNcE").unwrap(), Exchange::Binance);
    /// assert!(Exchange::from_str("foo").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "binance" => Ok(Self::Binance),
            "bybit" => Ok(Self::Bybit),
            "moex" => Ok(Self::Moex),
            "spb" => Ok(Self::Spb),
            _ => {
                let available = Self::all()
                    .iter()
                    .map(|exchange| exchange.key())
                    .collect::<Vec<_>>()
                    .join(", ");

                let msg = format!(
                    "unknown exchange key '{}', available=[{}]",
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
        let expected = [
            Exchange::Binance,
            Exchange::Bybit,
            Exchange::Moex,
            Exchange::Spb,
        ];

        assert_eq!(Exchange::all(), expected);
    }

    #[test]
    fn key() {
        assert_eq!(Exchange::Binance.key(), "binance");
        assert_eq!(Exchange::Bybit.key(), "bybit");
        assert_eq!(Exchange::Moex.key(), "moex");
        assert_eq!(Exchange::Spb.key(), "spb");
    }

    #[test]
    fn display() {
        assert_eq!(Exchange::Binance.to_string(), "Binance");
        assert_eq!(Exchange::Bybit.to_string(), "Bybit");
        assert_eq!(Exchange::Moex.to_string(), "MOEX");
        assert_eq!(Exchange::Spb.to_string(), "SPB");
    }

    #[test]
    fn from_str() {
        for exchange in Exchange::all().iter() {
            assert_eq!(
                Exchange::from_str(exchange.key()).unwrap(),
                *exchange
            );
        }

        assert_eq!(Exchange::from_str("MoEx").unwrap(), Exchange::Moex);

        assert!(matches!(
            Exchange::from_str("foo").unwrap_err(),
            DomainError::Value(_)
        ));
    }
}
