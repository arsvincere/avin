// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;
use std::str::FromStr;

use avin_utils::AvinError;

/// Trading instrument symbol.
///
/// For example: `"SBER"`, `"SiU6"`, `"BTCUSDT"`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(String);

impl Symbol {
    /// Creates a trading instrument symbol.
    ///
    /// # Errors
    ///
    /// Returns an error if the symbol is empty or contains whitespace.
    pub fn new(value: impl Into<String>) -> Result<Self, AvinError> {
        let s: String = value.into();

        validate_symbol(&s)?;

        Ok(Self(s))
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for Symbol {
    type Err = AvinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

fn validate_symbol(s: &str) -> Result<(), AvinError> {
    if s.is_empty() {
        return Err(AvinError::Value(
            "instrument symbol can't be empty".to_string(),
        ));
    }

    if s.chars().any(|c| c.is_whitespace()) {
        return Err(AvinError::Value(
            "instrument symbol can't contain whitespace".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_symbol() {
        assert!(Symbol::new("SBER").is_ok());
        assert!(Symbol::new("SiU6").is_ok());
        assert!(Symbol::new("BTCUSDT").is_ok());
    }

    #[test]
    fn empty_symbol() {
        assert!(Symbol::new("").is_err());
    }

    #[test]
    fn has_whitespace() {
        assert!(Symbol::new(" ").is_err());
        assert!(Symbol::new("SBER ").is_err());
        assert!(Symbol::new(" SBER").is_err());
        assert!(Symbol::new("SB ER").is_err());
    }

    #[test]
    fn display() {
        let symbol = Symbol::new("SBER").unwrap();
        assert_eq!(symbol.to_string(), "SBER");
    }

    #[test]
    fn from_str() {
        assert_eq!(
            Symbol::from_str("LKOH").unwrap(),
            Symbol::new("LKOH").unwrap()
        );
    }
}
