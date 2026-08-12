// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

/// Closed interval [low, high].
///
/// ## ru
/// Закрытый диапазон [low, high] - используется для представления
/// ценового диапазона, и определяет несколько утилитарных методов:
/// проверка на вхождение, выразить диапазон в процентах и тп.
///
/// Диапазон может быть:
/// 1. Возрастающий - конечное значение больше начального.
/// 2. Убывающий - конечное значение меньше начального.
///
/// ## Examples
/// ```
/// use avin_model::Range;
///
/// let r = Range::new(1000.0, 1500.0);
/// assert_eq!(r.delta(), 500.0);
///
/// let r = Range::new(1500.0, 1000.0);
/// assert_eq!(r.delta(), -500.0);
/// ```
#[derive(Debug)]
pub struct PriceChange {
    /// Начало диапазона (включительно)
    pub low: f64,
    /// Конец диапазона (включительно)
    pub high: f64,
}
impl Range {
    /// Create new range.
    ///
    /// ## ru
    /// Конструктор.
    ///
    /// ## Examples
    /// ```
    /// use avin_model::Range;
    ///
    /// let r = Range::new(1000.0, 1500.0);
    /// assert_eq!(r.low, 1000.0);
    /// assert_eq!(r.high, 1500.0);
    /// ```
    pub fn new(low: f64, high: f64) -> Self {
        Range { low, high }
    }

    /// Return min of range.
    ///
    /// ## ru
    /// Возвращает минимум диапазона.
    ///
    /// ## Examples
    /// ```
    /// use avin_model::Range;
    ///
    /// let r = Range::new(100.0, 101.5);
    /// assert_eq!(r.min(), 100.0);
    ///
    /// let r = Range::new(100.0, 99.1);
    /// assert_eq!(r.min(), 99.1);
    /// ```
    pub fn min(&self) -> f64 {
        if self.low < self.high {
            self.low
        } else {
            self.high
        }
    }
    /// Return max of range.
    ///
    /// ## ru
    /// Возвращает максимум диапазона.
    ///
    /// ## Examples
    /// ```
    /// use avin_model::Range;
    ///
    /// let r = Range::new(100.0, 101.5);
    /// assert_eq!(r.max(), 101.5);
    ///
    /// let r = Range::new(100.0, 99.1);
    /// assert_eq!(r.max(), 100.0);
    /// ```
    pub fn max(&self) -> f64 {
        if self.low > self.high {
            self.low
        } else {
            self.high
        }
    }
    /// Returns the middle of the range.
    ///
    /// ## ru
    /// Возвращает середину диапазона.
    ///
    /// ## Examples
    /// ```
    /// use avin_model::Range;
    ///
    /// let r = Range::new(100.0, 105.0);
    /// assert_eq!(r.mid(), 102.5);
    /// ```
    pub fn mid(&self) -> f64 {
        let min = self.min();
        let max = self.max();
        let half = (max - min) / 2.0;

        min + half
    }
    /// Check for value in range.
    ///
    /// ## ru
    /// Проверка на вхождения в диапазон.
    ///
    /// ## Examples
    /// ```
    /// use avin_model::Range;
    ///
    /// let r = Range::new(100.0, 105.0);
    /// assert_eq!(r.contains(103.0), true);
    /// assert_eq!(r.contains(100.0), true);
    /// assert_eq!(r.contains(105.0), true);
    /// assert_eq!(r.contains(105.1), false);
    /// assert_eq!(r.contains(99.9), false);
    /// ```
    pub fn contains(&self, value: f64) -> bool {
        self.min() <= value && value <= self.max()
    }

    /// Abs of range.
    ///
    /// ## ru
    /// Модуль диапазона.
    ///
    /// ## Examples
    /// ```
    /// use avin_model::Range;
    ///
    /// let r = Range::new(1000.0, 1050.0);
    /// assert_eq!(r.abs(), 50.0);
    ///
    /// let r = Range::new(1050.0, 1000.0);
    /// assert_eq!(r.abs(), 50.0);
    /// ```
    pub fn abs(&self) -> f64 {
        self.max() - self.min()
    }
    /// Normalized abs of range.
    ///
    /// ## ru
    /// Нормализованный модуль диапазона.
    ///
    /// ## Examples
    /// ```
    /// use avin_model::Range;
    ///
    /// let r = Range::new(900.0, 1000.0);
    /// assert_eq!(r.abs_n(), 0.10);
    /// ```
    pub fn abs_n(&self) -> f64 {
        let mn = self.min();
        let mx = self.max();

        (mx - mn) / mx
    }
    /// Abs of range in percent.
    ///
    /// ## ru
    /// Модуль диапазона в процентах
    ///
    /// ## Examples
    /// ```
    /// use avin_model::Range;
    ///
    /// let r = Range::new(970.0, 1000.0);
    /// assert_eq!(r.abs_p(), 3.0);
    /// ```
    pub fn abs_p(&self) -> f64 {
        let mn = self.min();
        let mx = self.max();

        let value = (mx - mn) / mx * 100.0;

        value
    }

    /// Delta of range (signed).
    ///
    /// ## ru
    /// Дельта диапазона (знаковая).
    ///
    /// ## Examples
    /// ```
    /// use avin_model::Range;
    ///
    /// let r = Range::new(1000.0, 1050.0);
    /// assert_eq!(r.delta(), 50.0);
    ///
    /// let r = Range::new(1000.0, 900.0);
    /// assert_eq!(r.delta(), -100.0);
    /// ```
    pub fn delta(&self) -> f64 {
        self.high - self.low
    }
    /// Normalized delta of range (signed).
    ///
    /// ## ru
    /// Нормализованная дельта диапазона (знаковая) - показывает коэффициент
    /// изменения конечной цены относительно начальной.
    ///
    /// ## Examples
    /// ```
    /// use avin_model::Range;
    ///
    /// let r = Range::new(1000.0, 1050.0);
    /// assert_eq!(r.delta_n(), 0.05);
    ///
    /// let r = Range::new(1000.0, 900.0);
    /// assert_eq!(r.delta_n(), -0.10);
    /// ```
    pub fn delta_n(&self) -> f64 {
        (self.high - self.low) / self.low
    }
    /// Delta of range in percent.
    ///
    /// ## ru
    /// Дельта диапазона (знаковая) в процентах - показывает процент
    /// изменения конечной цены относительно начальной.
    ///
    /// ## Examples
    /// ```
    /// use avin_model::Range;
    ///
    /// let r = Range::new(1000.0, 1050.0);
    /// assert_eq!(r.delta_p(), 5.0);
    ///
    /// let r = Range::new(1000.0, 900.0);
    /// assert_eq!(r.delta_p(), -10.0);
    /// ```
    pub fn delta_p(&self) -> f64 {
        let value = (self.high - self.low) / self.low * 100.0;

        value
    }
    /// Is range increase.
    ///
    /// ## ru
    /// Если диапазон возврастающий - true.
    ///
    /// ## Examples
    /// ```
    /// use avin_model::Range;
    ///
    /// let r = Range::new(1000.0, 1050.0);
    /// assert_eq!(r.is_increase(), true);
    ///
    /// let r = Range::new(1000.0, 900.0);
    /// assert_eq!(r.is_increase(), false);
    /// ```
    pub fn is_increase(&self) -> bool {
        self.delta() > 0.0
    }
    /// Is range decrease.
    ///
    /// ## ru
    /// Если диапазон убывающий - true.
    ///
    /// ## Examples
    /// ```
    /// use avin_model::Range;
    ///
    /// let r = Range::new(1000.0, 1050.0);
    /// assert_eq!(r.is_decrease(), false);
    ///
    /// let r = Range::new(1000.0, 900.0);
    /// assert_eq!(r.is_decrease(), true);
    /// ```
    pub fn is_decrease(&self) -> bool {
        self.delta() < 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max() {
        let r = Range::new(100.0, 110.0);
        assert_eq!(r.max(), 110.0);
    }
    #[test]
    fn min() {
        let r = Range::new(100.0, 110.0);
        assert_eq!(r.min(), 100.0);
    }
    #[test]
    fn mid() {
        let r = Range::new(100.0, 110.0);
        assert_eq!(r.mid(), 105.0);
    }
    #[test]
    fn abs() {
        let r = Range::new(5000.0, 4000.0);
        assert_eq!(r.abs(), 1000.0);
        assert_eq!(r.abs_n(), 0.2);
        assert_eq!(r.abs_p(), 20.0);
    }
    #[test]
    fn delta() {
        let r = Range::new(5000.0, 4000.0);
        assert_eq!(r.delta(), -1000.0);
        assert_eq!(r.delta_n(), -0.2);
        assert_eq!(r.delta_p(), -20.0);
    }
}
