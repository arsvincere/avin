// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use avin_utils::AvinError;

/// # en
/// Closed price interval [low, high].
///
/// Represents a price range including both boundary values.
///
/// # ru
/// Закрытый ценовой интервал [low, high].
///
/// Представляет ценовой диапазон, включающий начальное и конечное значение.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PriceRange {
    low: f64,
    high: f64,
}

impl PriceRange {
    /// # en
    /// Creates a new price range with the given lower and upper bounds.
    ///
    /// Returns an error if either bound is non-finite or if `low > high`.
    ///
    /// # ru
    /// Создает новый ценовой диапазон с заданными нижней и верхней границами.
    ///
    /// Возвращает ошибку, если хотя бы одна граница не является конечным числом
    /// или если `low > high`.
    ///
    /// # Examples
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

    /// # en
    /// Returns the lower bound of the range.
    ///
    /// # ru
    /// Возвращает нижнюю границу диапазона.
    ///
    /// # Examples
    /// ```
    /// use avin_model::PriceRange;
    ///
    /// let r = PriceRange::new(1000.0, 1500.0).unwrap();
    /// assert_eq!(r.low(), 1000.0);
    /// ```
    pub fn low(&self) -> f64 {
        self.low
    }

    /// # en
    /// Returns the upper bound of the range.
    ///
    /// # ru
    /// Возвращает верхнюю границу диапазона.
    ///
    /// # Examples
    /// ```
    /// use avin_model::PriceRange;
    ///
    /// let r = PriceRange::new(1000.0, 1500.0).unwrap();
    /// assert_eq!(r.high(), 1500.0);
    /// ```
    pub fn high(&self) -> f64 {
        self.high
    }

    /// # en
    /// Checks whether the given value is within the range.
    ///
    /// Both boundary values are included.
    ///
    /// # ru
    /// Проверяет, входит ли заданное значение в диапазон.
    ///
    /// Обе границы диапазона включены.
    ///
    /// # Examples
    /// ```
    /// use avin_model::PriceRange;
    ///
    /// let r = PriceRange::new(100.0, 105.0).unwrap();
    ///
    /// assert!(r.contains(103.0));
    /// assert!(r.contains(100.0));
    /// assert!(r.contains(105.0));
    ///
    /// assert!(!r.contains(105.1));
    /// assert!(!r.contains(99.9));
    /// ```
    pub fn contains(&self, value: f64) -> bool {
        self.low <= value && value <= self.high
    }

    /// # en
    /// Returns the midpoint of the range.
    ///
    /// # ru
    /// Возвращает середину диапазона.
    ///
    /// # Examples
    /// ```
    /// use avin_model::PriceRange;
    ///
    /// let r = PriceRange::new(100.0, 105.0).unwrap();
    /// assert_eq!(r.middle(), 102.5);
    /// ```
    pub fn middle(&self) -> f64 {
        self.low + (self.high - self.low) / 2.0
    }

    /// # en
    /// Returns the width of the range.
    ///
    /// # ru
    /// Возвращает ширину диапазона.
    ///
    /// # Examples
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

    #[test]
    fn display() {
        let r = PriceRange::new(125.5, 129.1).unwrap();

        assert_eq!(r.to_string(), "[125.5, 129.1]");
    }
}
