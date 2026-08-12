// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_utils::AvinError;

/// Closed interval [low, high].
///
/// ## ru
/// Ценовой диапазон [low, high].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PriceRange {
    /// Нижняя граница диапазона (включительно)
    low: f64,
    /// Верхняя граница диапазона (включительно)
    high: f64,
}
impl PriceRange {
    /// Create new price range.
    ///
    /// ## ru
    /// Создать новый ценовой диапазон.
    ///
    /// ## Examples
    /// ```
    /// use avin_model::PriceRange;
    ///
    /// let r = PriceRange::new(100.0, 120.0).unwrap();
    /// assert_eq!(r.low(), 100.0);
    /// assert_eq!(r.high(), 120.0);
    /// ```
    pub fn new(low: f64, high: f64) -> Result<Self, AvinError> {
        if !low.is_finite() || !high.is_finite() {
            return Err(AvinError::InvalidValue(format!(
                "PriceRange: non-finite [{low}, {high}]"
            )));
        }

        if low > high {
            return Err(AvinError::InvalidValue(format!(
                "PriceRange: low > high [{low}, {high}]"
            )));
        }

        Ok(Self { low, high })
    }

    /// Low bound of range.
    ///
    /// ## ru
    /// Нижняя граница диапазона.
    ///
    /// ## Examples
    /// ```
    /// use avin_model::PriceRange;
    ///
    /// let r = PriceRange::new(1000.0, 1500.0).unwrap();
    /// assert_eq!(r.low(), 1000.0);
    /// ```
    pub fn low(&self) -> f64 {
        self.low
    }

    /// High bound of range.
    ///
    /// ## ru
    /// Верхняя граница диапазона.
    ///
    /// ## Examples
    /// ```
    /// use avin_model::PriceRange;
    ///
    /// let r = PriceRange::new(1000.0, 1500.0).unwrap();
    /// assert_eq!(r.high(), 1500.0);
    /// ```
    pub fn high(&self) -> f64 {
        self.high
    }

    /// Check for value in PriceRange.
    ///
    /// ## ru
    /// Проверка на вхождения в диапазон.
    ///
    /// ## Examples
    /// ```
    /// use avin_model::PriceRange;
    ///
    /// let r = PriceRange::new(100.0, 105.0).unwrap();
    ///
    /// assert_eq!(r.contains(103.0), true);
    /// assert_eq!(r.contains(100.0), true);
    /// assert_eq!(r.contains(105.0), true);
    ///
    /// assert_eq!(r.contains(105.1), false);
    /// assert_eq!(r.contains(99.9), false);
    /// ```
    pub fn contains(&self, value: f64) -> bool {
        self.low <= value && value <= self.high
    }

    /// Returns the middle of the PriceRange.
    ///
    /// ## ru
    /// Возвращает середину диапазона.
    ///
    /// ## Examples
    /// ```
    /// use avin_model::PriceRange;
    ///
    /// let r = PriceRange::new(100.0, 105.0).unwrap();
    /// assert_eq!(r.middle(), 102.5);
    /// ```
    pub fn middle(&self) -> f64 {
        self.low + (self.high - self.low) / 2.0
    }

    /// Width of PriceRange.
    ///
    /// ## ru
    /// Ширина диапазона.
    ///
    /// ## Examples
    /// ```
    /// use avin_model::PriceRange;
    ///
    /// let r = PriceRange::new(1000.0, 1050.0).unwrap();
    /// assert_eq!(r.width(), 50.0);
    /// ```
    pub fn width(&self) -> f64 {
        self.high - self.low
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

        assert!(!r.contains(111.1));
        assert!(!r.contains(5.0));
    }

    #[test]
    fn middle() {
        let r = PriceRange::new(100.0, 110.0).unwrap();

        assert_eq!(r.middle(), 105.0);
    }

    #[test]
    fn width() {
        let r = PriceRange::new(4000.0, 5000.0).unwrap();

        assert_eq!(r.width(), 1000.0);
    }
}
