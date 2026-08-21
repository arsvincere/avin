// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_utils::AvinError;

/// Closed price interval `[low, high]`.
///
/// Represents a price range including both boundary values.
///
/// # Examples
///
/// ```
/// use avin_model::PriceRange;
///
/// let range = PriceRange::new(100.0, 105.0).unwrap();
/// assert_eq!(range.low(), 100.0);
/// assert_eq!(range.high(), 105.0);
///
/// assert!(range.contains(100.0));
/// assert!(range.contains(103.0));
/// assert!(range.contains(105.0));
///
/// assert!(!range.contains(105.1));
/// assert!(!range.contains(99.9));
///
/// assert_eq!(range.middle(), 102.5);
///
/// assert_eq!(range.width(), 5.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PriceRange {
    low: f64,
    high: f64,
}

impl PriceRange {
    /// Creates a new price range with the given lower and upper bounds.
    ///
    /// # Errors
    ///
    /// Returns an error if either bound is non-finite (`NaN`, positive or
    /// negative infinity), or if `low > high`.
    ///
    /// # Examples
    ///
    /// ```
    /// use avin_model::PriceRange;
    ///
    /// assert!(PriceRange::new(100.0, 110.0).is_ok());
    ///
    /// assert!(PriceRange::new(105.0, 100.0).is_err());
    /// assert!(PriceRange::new(f64::NAN, 100.0).is_err());
    /// assert!(PriceRange::new(f64::INFINITY, 100.0).is_err());
    /// ```
    pub fn new(low: f64, high: f64) -> Result<Self, AvinError> {
        if !low.is_finite() || !high.is_finite() {
            return Err(AvinError::InvalidValue(format!(
                "PriceRange non-finite [{low}, {high}]"
            )));
        }

        if low > high {
            return Err(AvinError::InvalidValue(format!(
                "PriceRange low > high [{low}, {high}]"
            )));
        }

        Ok(Self { low, high })
    }

    /// Returns the lower bound of the range.
    pub fn low(&self) -> f64 {
        self.low
    }

    /// Returns the upper bound of the range.
    pub fn high(&self) -> f64 {
        self.high
    }

    /// Checks whether the given value is within the range.
    ///
    /// Both boundary values are included.
    pub fn contains(&self, value: f64) -> bool {
        self.low <= value && value <= self.high
    }

    /// Returns the midpoint of the range.
    pub fn middle(&self) -> f64 {
        self.low.midpoint(self.high)
    }

    /// Returns the width of the range.
    pub fn width(&self) -> f64 {
        self.high - self.low
    }
}

impl std::fmt::Display for PriceRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.low, self.high)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds() {
        let r = PriceRange::new(100.0, 110.0).unwrap();

        assert_eq!(r.low(), 100.0);
        assert_eq!(r.high(), 110.0);
    }

    #[test]
    fn invalid_range() {
        let r = PriceRange::new(110.0, 100.0);

        assert!(r.is_err());
    }

    #[test]
    fn infinite_range() {
        assert!(PriceRange::new(f64::INFINITY, 100.0).is_err());
        assert!(PriceRange::new(100.0, f64::INFINITY).is_err());
        assert!(PriceRange::new(f64::NEG_INFINITY, 100.0).is_err());
        assert!(PriceRange::new(100.0, f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn nan_range() {
        assert!(PriceRange::new(f64::NAN, 100.0).is_err());
        assert!(PriceRange::new(100.0, f64::NAN).is_err());
    }

    #[test]
    fn contains() {
        let r = PriceRange::new(100.0, 110.0).unwrap();

        assert!(r.contains(105.0));
        assert!(r.contains(100.0));
        assert!(r.contains(110.0));

        assert!(!r.contains(111.1));
        assert!(!r.contains(5.0));
    }

    #[test]
    fn middle() {
        let r = PriceRange::new(100.0, 110.0).unwrap();

        assert_eq!(r.middle(), 105.0);
    }

    #[test]
    fn middle_large_opposite_bounds() {
        let r = PriceRange::new(-1e308, 1e308).unwrap();

        assert_eq!(r.middle(), 0.0);
    }

    #[test]
    fn width() {
        let r = PriceRange::new(4000.0, 5000.0).unwrap();

        assert_eq!(r.width(), 1000.0);
    }

    #[test]
    fn display() {
        let r = PriceRange::new(125.5, 129.1).unwrap();

        assert_eq!(r.to_string(), "[125.5, 129.1]");
    }
}
