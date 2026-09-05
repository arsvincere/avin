// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;

use crate::DomainError;

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
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
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

fn validate_ticker(s: &str) -> Result<(), DomainError> {
    if s.is_empty() {
        return Err(DomainError::Ticker(
            "instrument ticker can't be empty".to_string(),
        ));
    }

    if s.chars().any(|c| c.is_whitespace()) {
        return Err(DomainError::Ticker(
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
        assert!(matches!(
            Ticker::new("").unwrap_err(),
            DomainError::Ticker(_)
        ));
    }

    #[test]
    fn has_whitespace() {
        assert!(matches!(
            Ticker::new(" ").unwrap_err(),
            DomainError::Ticker(_)
        ));
        assert!(matches!(
            Ticker::new("SBER ").unwrap_err(),
            DomainError::Ticker(_)
        ));
        assert!(matches!(
            Ticker::new(" SBER").unwrap_err(),
            DomainError::Ticker(_)
        ));
        assert!(matches!(
            Ticker::new("SB ER").unwrap_err(),
            DomainError::Ticker(_)
        ));
    }

    #[test]
    fn display() {
        let ticker = Ticker::new("SBER").unwrap();
        assert_eq!(ticker.to_string(), "SBER");
    }
}
