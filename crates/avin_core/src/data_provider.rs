// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;
use std::str::FromStr;

use avin_utils::AvinError;

/// Identifies a market data provider supported by AVIN.
///
/// `DataProvider` is a low-level identifier shared across AVIN components. It
/// identifies the provider from which market data is obtained, but contains
/// no provider-specific behavior or configuration.
///
/// Each provider has a stable machine-readable key returned by
/// [`DataProvider::key`]. Keys can be parsed case-insensitively using
/// [`FromStr`] and are intended for configuration and persistent paths.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use avin_core::DataProvider;
///
/// let provider = DataProvider::from_str("tbank").unwrap();
///
/// assert_eq!(provider, DataProvider::TBank);
/// assert_eq!(provider.to_string(), "T-Bank");
/// assert_eq!(provider.key(), "tbank");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataProvider {
    TBank,
    MoexAlgo,
}

impl DataProvider {
    /// Returns all supported providers.
    pub const fn all() -> &'static [Self] {
        &[Self::TBank, Self::MoexAlgo]
    }

    /// Returns a stable machine-readable identifier suitable for persistence
    /// and serialization.
    pub fn key(&self) -> &'static str {
        match self {
            Self::TBank => "tbank",
            Self::MoexAlgo => "moexalgo",
        }
    }
}

impl Display for DataProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TBank => f.write_str("T-Bank"),
            Self::MoexAlgo => f.write_str("MOEX ALGOPACK"),
        }
    }
}

impl FromStr for DataProvider {
    type Err = AvinError;

    /// Parses a market data provider key.
    ///
    /// Parsing is case-insensitive.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider name is unknown.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use avin_core::DataProvider;
    ///
    /// assert_eq!(DataProvider::from_str("TBank").unwrap(), DataProvider::TBank);
    /// assert_eq!(DataProvider::from_str("tbank").unwrap(), DataProvider::TBank);
    /// assert!(DataProvider::from_str("foo").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "tbank" => Ok(Self::TBank),
            "moexalgo" => Ok(Self::MoexAlgo),
            _ => {
                let available = Self::all()
                    .iter()
                    .map(|provider| provider.key())
                    .collect::<Vec<_>>()
                    .join(", ");

                let msg = format!(
                    "unknown data provider key '{}', available=[{}]",
                    s, available
                );

                Err(AvinError::Value(msg))
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
            DataProvider::all(),
            &[DataProvider::TBank, DataProvider::MoexAlgo]
        );
    }

    #[test]
    fn key() {
        assert_eq!(DataProvider::TBank.key(), "tbank");
        assert_eq!(DataProvider::MoexAlgo.key(), "moexalgo");
    }

    #[test]
    fn display() {
        assert_eq!(DataProvider::TBank.to_string(), "T-Bank");
        assert_eq!(DataProvider::MoexAlgo.to_string(), "MOEX ALGOPACK");
    }

    #[test]
    fn from_str() {
        assert_eq!(
            DataProvider::from_str("tbank").unwrap(),
            DataProvider::TBank
        );
        assert_eq!(
            DataProvider::from_str("mOExalGO").unwrap(),
            DataProvider::MoexAlgo
        );

        assert!(matches!(
            DataProvider::from_str("foo").unwrap_err(),
            AvinError::Value(_)
        ));
    }
}
