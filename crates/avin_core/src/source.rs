// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;
use std::str::FromStr;

use avin_utils::AvinError;

/// Market data source.
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
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TBank => f.write_str("TBank"),
            Self::MoexAlgo => f.write_str("MoexAlgo"),
        }
    }
}

impl FromStr for Source {
    type Err = AvinError;

    /// Parses an source name.
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
    /// assert!(Source::from_str("foo").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "tbank" => Ok(Self::TBank),
            "moexalgo" => Ok(Self::MoexAlgo),
            _ => {
                let available = Self::all()
                    .iter()
                    .map(Self::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");

                let msg = format!(
                    "unknown Source '{}', available=[{}]",
                    s, available
                );

                Err(AvinError::Value(msg))
            }
        }
    }
}
