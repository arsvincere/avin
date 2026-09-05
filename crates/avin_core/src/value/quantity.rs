// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::cmp::Ordering;
use std::fmt::Display;

use crate::CoreError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quantity(f64);

/// Canonical quantity representation used across AVIN.
///
/// Internally, it stores the quantity as an `f64`.
///
/// Only non-negative, finite values are supported.
/// Negative values, NaN and +/- infinity are rejected.
///
/// # Examples
///
/// ```
/// use avin_core::Quantity;
///
/// let quantity = Quantity::new(459.5).unwrap();
///
/// assert_eq!(quantity.value(), 459.5);
/// assert_eq!(quantity.to_string(), "459.5");
/// ```
impl Quantity {
    /// Creates a new [`Quantity`].
    ///
    /// # Errors
    ///
    /// Returns an error if the quantity is not finite or negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use avin_core::Quantity;
    ///
    /// assert!(Quantity::new(100.0).is_ok());
    /// assert!(Quantity::new(0.001346).is_ok());
    /// assert!(Quantity::new(0.0).is_ok());
    ///
    /// assert!(Quantity::new(-50.5).is_err());
    /// assert!(Quantity::new(f64::INFINITY).is_err());
    /// assert!(Quantity::new(f64::NEG_INFINITY).is_err());
    /// assert!(Quantity::new(f64::NAN).is_err());
    /// ```
    pub fn new(value: f64) -> Result<Self, CoreError> {
        if !value.is_finite() {
            return Err(CoreError::Quantity(format!(
                "quantity must be finite, got {value}"
            )));
        }

        if value < 0.0 {
            return Err(CoreError::Quantity(format!(
                "quantity must be non-negative, got {value}"
            )));
        }

        // fix Price::new(-0.0).unwrap().to_string() -> "-0"
        let value = if value == -0.0 { 0.0 } else { value };

        Ok(Self(value))
    }

    /// Returns the quantity as a `f64`.
    pub fn value(self) -> f64 {
        self.0
    }

    /// Returns whether the quantity is zero.
    pub fn is_zero(self) -> bool {
        self.0 == 0.0
    }
}

impl Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Eq for Quantity {}

impl Ord for Quantity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .partial_cmp(&other.0)
            .expect("Quantity always contains a finite value")
    }
}

impl PartialOrd for Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let quantity = Quantity::new(320.55).unwrap();

        assert_eq!(quantity.value(), 320.55);
    }

    #[test]
    fn zero() {
        let quantity = Quantity::new(0.0).unwrap();

        assert_eq!(quantity.value(), 0.0);
    }

    #[test]
    fn finite() {
        assert!(matches!(
            Quantity::new(f64::INFINITY).unwrap_err(),
            CoreError::Quantity(_)
        ));
        assert!(matches!(
            Quantity::new(f64::NEG_INFINITY).unwrap_err(),
            CoreError::Quantity(_)
        ));
        assert!(matches!(
            Quantity::new(f64::NAN).unwrap_err(),
            CoreError::Quantity(_)
        ));
    }

    #[test]
    fn not_negative() {
        assert!(matches!(
            Quantity::new(-37.63).unwrap_err(),
            CoreError::Quantity(_)
        ));
    }

    #[test]
    fn negative_zero() {
        let pos = Quantity::new(0.0).unwrap();
        let neg = Quantity::new(-0.0).unwrap();

        assert!(!neg.value().is_sign_negative());
        assert_eq!(pos, neg);
    }

    #[test]
    fn display() {
        let quantity = Quantity::new(4593.1).unwrap();
        assert_eq!(quantity.to_string(), "4593.1");

        let quantity = Quantity::new(-0.0).unwrap();
        assert_eq!(quantity.to_string(), "0");
    }

    #[test]
    fn ordering() {
        let small = Quantity::new(10.0).unwrap();
        let large = Quantity::new(20.0).unwrap();

        assert!(small < large);
        assert!(large > small);
        assert_eq!(small, Quantity::new(10.0).unwrap());
    }
}
