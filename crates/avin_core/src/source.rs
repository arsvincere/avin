// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;
use std::str::FromStr;

use avin_utils::AvinError;

/// Identifies a market data source supported by AVIN.
///
/// `Source` is a low-level identifier shared across AVIN components. It names
/// the provider from which market data is obtained, but contains no
/// provider-specific behavior or configuration.
///
/// Source names have a canonical representation defined by [`Display`] and
/// can be parsed case-insensitively using [`FromStr`].
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use avin_core::Source;
///
/// let source = Source::from_str("tbank").unwrap();
///
/// assert_eq!(source, Source::TBank);
/// assert_eq!(source.to_string(), "T-Bank");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Source {
    TBank,
    MoexAlgo,
}

impl Source {
    /// Returns all supported sources.
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

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TBank => f.write_str("T-Bank"),
            Self::MoexAlgo => f.write_str("MOEX ALGOPACK"),
        }
    }
}

impl FromStr for Source {
    type Err = AvinError;

    /// Parses a market data source name.
    ///
    /// Parsing is case-insensitive.
    ///
    /// # Errors
    ///
    /// Returns an error if the source name is unknown.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use avin_core::Source;
    ///
    /// assert_eq!(Source::from_str("TBank").unwrap(), Source::TBank);
    /// assert_eq!(Source::from_str("tbank").unwrap(), Source::TBank);
    /// assert!(Source::from_str("foo").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "tbank" => Ok(Self::TBank),
            "moexalgo" => Ok(Self::MoexAlgo),
            _ => {
                let available = Self::all()
                    .iter()
                    .map(|source| source.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                let msg = format!(
                    "unknown source '{}', available=[{}]",
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
        assert_eq!(Source::all(), &[Source::TBank, Source::MoexAlgo]);
    }

    #[test]
    fn key() {
        assert_eq!(Source::TBank.key(), "tbank");
        assert_eq!(Source::MoexAlgo.key(), "moexalgo");
    }

    #[test]
    fn display() {
        assert_eq!(Source::TBank.to_string(), "T-Bank");
        assert_eq!(Source::MoexAlgo.to_string(), "MOEX ALGOPACK");
    }

    #[test]
    fn from_str() {
        assert_eq!(Source::from_str("tbank").unwrap(), Source::TBank);
        assert_eq!(Source::from_str("TBANK").unwrap(), Source::TBank);

        assert!(matches!(
            Source::from_str("foo").unwrap_err(),
            AvinError::Value(_)
        ));
    }
}
