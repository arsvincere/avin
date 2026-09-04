// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;

use crate::CoreError;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Price(f64);

/// Canonical price representation used across AVIN.
///
/// Internally, it stores the price as an `f64`.
///
/// Only finite values are supported; NaN and +/- infinity are rejected.
///
/// # Examples
///
/// ```
/// use avin_core::Price;
///
/// let price = Price::new(4593.1).unwrap();
///
/// assert_eq!(price.value(), 4593.1);
/// assert_eq!(price.to_string(), "4593.1");
/// ```
impl Price {
    /// Creates a new [`Price`].
    ///
    /// # Errors
    ///
    /// Returns an error if the price is not finite.
    ///
    /// # Examples
    ///
    /// ```
    /// use avin_core::Price;
    ///
    /// assert!(Price::new(4593.1).is_ok());
    ///
    /// assert!(Price::new(f64::INFINITY).is_err());
    /// assert!(Price::new(f64::NEG_INFINITY).is_err());
    /// assert!(Price::new(f64::NAN).is_err());
    /// ```
    pub fn new(value: f64) -> Result<Self, CoreError> {
        if !value.is_finite() {
            return Err(CoreError::Price(format!(
                "price must be finite, got {value}"
            )));
        }

        Ok(Self(value))
    }

    /// Returns the price as a `f64`.
    pub fn value(self) -> f64 {
        self.0
    }
}

impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let price = Price::new(320.55).unwrap();

        assert_eq!(price.value(), 320.55);
    }

    #[test]
    fn finite() {
        assert!(Price::new(f64::INFINITY).is_err());
        assert!(Price::new(f64::NEG_INFINITY).is_err());
        assert!(Price::new(f64::NAN).is_err());
    }

    #[test]
    fn negative() {
        // price of Future can be negative
        assert_eq!(Price::new(-37.63).unwrap().value(), -37.63);
    }

    #[test]
    fn display() {
        let price = Price::new(4593.1).unwrap();

        assert_eq!(price.to_string(), "4593.1");
    }
}
