// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;

use crate::{CoreError, Price};

/// Closed price interval `[low, high]`.
///
/// Represents a price range including both boundary values.
///
/// # Examples
///
/// ```
/// use avin_core::{Price, PriceRange};
///
/// let low = Price::new(100.0).unwrap();
/// let high = Price::new(105.0).unwrap();
/// let range = PriceRange::new(low, high).unwrap();
///
/// assert_eq!(range.low(), Price::new(100.0).unwrap());
/// assert_eq!(range.high(), Price::new(105.0).unwrap());
///
/// assert!(range.contains(Price::new(100.0).unwrap()));
/// assert!(range.contains(Price::new(103.0).unwrap()));
/// assert!(range.contains(Price::new(105.0).unwrap()));
///
/// assert!(!range.contains(Price::new(105.1).unwrap()));
/// assert!(!range.contains(Price::new(99.9).unwrap()));
///
/// assert_eq!(range.middle(), Price::new(102.5).unwrap());
///
/// assert_eq!(range.width(), 5.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PriceRange {
    low: Price,
    high: Price,
}

impl PriceRange {
    /// Creates a new price range with the given lower and upper bounds.
    ///
    /// # Errors
    ///
    /// Returns an error if `low > high`.
    ///
    /// # Examples
    ///
    /// ```
    /// use avin_core::{Price, PriceRange};
    ///
    /// let low = Price::new(100.0).unwrap();
    /// let high = Price::new(110.0).unwrap();
    /// assert!(PriceRange::new(low, high).is_ok());
    ///
    /// let low = Price::new(105.0).unwrap();
    /// let high = Price::new(100.0).unwrap();
    /// assert!(PriceRange::new(low, high).is_err());
    /// ```
    pub fn new(low: Price, high: Price) -> Result<Self, CoreError> {
        if low > high {
            return Err(CoreError::PriceRange(format!(
                "PriceRange low > high [{low}, {high}]"
            )));
        }

        Ok(Self { low, high })
    }

    /// Returns the lower bound of the range.
    pub fn low(self) -> Price {
        self.low
    }

    /// Returns the upper bound of the range.
    pub fn high(self) -> Price {
        self.high
    }

    /// Checks whether the given price is within the range.
    ///
    /// Both boundary values are included.
    pub fn contains(self, price: Price) -> bool {
        self.low <= price && price <= self.high
    }

    /// Returns the midpoint of the range.
    pub fn middle(self) -> Price {
        let middle = self.low.value().midpoint(self.high.value());

        Price::new(middle).expect("midpoint of finite prices must be finite")
    }

    /// Returns the width of the range.
    pub fn width(self) -> f64 {
        self.high.value() - self.low.value()
    }
}

impl Display for PriceRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.low, self.high)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds() {
        let from = Price::new(100.0).unwrap();
        let till = Price::new(110.0).unwrap();
        let r = PriceRange::new(from, till).unwrap();

        assert_eq!(r.low(), from);
        assert_eq!(r.high(), till);
    }

    #[test]
    fn invalid_range() {
        let from = Price::new(110.0).unwrap();
        let till = Price::new(100.0).unwrap();
        let r = PriceRange::new(from, till);

        let err = r.unwrap_err();

        assert!(matches!(err, CoreError::PriceRange(_)));
    }
    #[test]
    fn contains() {
        let from = Price::new(100.0).unwrap();
        let till = Price::new(110.0).unwrap();
        let r = PriceRange::new(from, till).unwrap();

        assert!(r.contains(Price::new(105.0).unwrap()));
        assert!(r.contains(Price::new(100.0).unwrap()));
        assert!(r.contains(Price::new(110.0).unwrap()));

        assert!(!r.contains(Price::new(111.1).unwrap()));
        assert!(!r.contains(Price::new(5.0).unwrap()));
    }

    #[test]
    fn middle() {
        let from = Price::new(100.0).unwrap();
        let till = Price::new(110.0).unwrap();
        let r = PriceRange::new(from, till).unwrap();

        assert_eq!(r.middle(), Price::new(105.0).unwrap());
    }

    #[test]
    fn middle_large_opposite_bounds() {
        let from = Price::new(-1e308).unwrap();
        let till = Price::new(1e308).unwrap();
        let r = PriceRange::new(from, till).unwrap();

        assert_eq!(r.middle(), Price::new(0.0).unwrap());
    }

    #[test]
    fn width() {
        let from = Price::new(4000.0).unwrap();
        let till = Price::new(5000.0).unwrap();
        let r = PriceRange::new(from, till).unwrap();

        assert_eq!(r.width(), 1000.0);
    }

    #[test]
    fn zero_width() {
        let price = Price::new(100.0).unwrap();
        let range = PriceRange::new(price, price).unwrap();

        assert!(range.contains(price));
        assert_eq!(range.width(), 0.0);
    }

    #[test]
    fn display() {
        let from = Price::new(125.5).unwrap();
        let till = Price::new(129.1).unwrap();
        let r = PriceRange::new(from, till).unwrap();

        assert_eq!(r.to_string(), "[125.5, 129.1]");
    }
}
