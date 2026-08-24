// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;
use std::str::FromStr;

use avin_utils::AvinError;

/// Trading instrument ticker.
///
/// For example: `"SBER"`, `"SiU6"`, `"BTCUSDT"`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ticker(String);

impl Ticker {
    /// Creates a trading instrument ticker.
    ///
    /// # Errors
    ///
    /// Returns an error if the ticker is empty or contains whitespace.
    pub fn new(value: impl Into<String>) -> Result<Self, AvinError> {
        let s: String = value.into();

        validate_ticker(&s)?;

        Ok(Self(s))
    }
}

impl Display for Ticker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for Ticker {
    type Err = AvinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

fn validate_ticker(s: &str) -> Result<(), AvinError> {
    if s.is_empty() {
        return Err(AvinError::Value(
            "instrument ticker can't be empty".to_string(),
        ));
    }

    if s.chars().any(|c| c.is_whitespace()) {
        return Err(AvinError::Value(
            "instrument ticker can't contain whitespace".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_ticker() {
        assert!(Ticker::new("SBER").is_ok());
        assert!(Ticker::new("SiU6").is_ok());
        assert!(Ticker::new("BTCUSDT").is_ok());
    }

    #[test]
    fn empty_ticker() {
        assert!(Ticker::new("").is_err());
    }

    #[test]
    fn has_whitespace() {
        assert!(Ticker::new(" ").is_err());
        assert!(Ticker::new("SBER ").is_err());
        assert!(Ticker::new(" SBER").is_err());
        assert!(Ticker::new("SB ER").is_err());
    }

    #[test]
    fn display() {
        let ticker = Ticker::new("SBER").unwrap();
        assert_eq!(ticker.to_string(), "SBER");
    }

    #[test]
    fn from_str() {
        assert_eq!(
            Ticker::from_str("LKOH").unwrap(),
            Ticker::new("LKOH").unwrap()
        );
    }
}
