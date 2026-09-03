// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;

use chrono::Datelike;

use crate::{CoreError, Time};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Year(u16);

/// Canonical calendar year representation used across AVIN.
///
/// Internally, it stores the year as a `u16`.
///
/// Supported years are limited by the range supported by [`Time`].
///
/// # Examples
///
/// ```
/// use avin_core::Year;
///
/// let year = Year::new(2026).unwrap();
///
/// assert_eq!(year.value(), 2026);
/// assert_eq!(year.to_string(), "2026");
/// ```
impl Year {
    /// Creates a new [`Year`].
    ///
    /// # Errors
    ///
    /// Returns an error if the year is outside the range supported by [`Time`].
    ///
    /// # Examples
    ///
    /// ```
    /// use avin_core::Year;
    ///
    /// assert!(Year::new(2026).is_ok());
    ///
    /// assert!(Year::new(1382).is_err());
    /// assert!(Year::new(3567).is_err());
    /// ```
    pub fn new(year: u16) -> Result<Self, CoreError> {
        let min = Time::new(i64::MIN).dt().year() as u16;
        let max = Time::new(i64::MAX).dt().year() as u16;

        if year < min || year > max {
            return Err(CoreError::Year(format!(
                "year {year} is outside supported range [{min}, {max}]"
            )));
        }

        Ok(Self(year))
    }

    /// Returns the calendar year as a `u16`.
    pub fn value(self) -> u16 {
        self.0
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let year = Year::new(2026).unwrap();

        assert_eq!(year.value(), 2026);
    }

    #[test]
    fn min_supported() {
        let min = Time::new(i64::MIN).dt().year() as u16;
        let year = Year::new(min).unwrap();

        assert_eq!(year.value(), min);
    }

    #[test]
    fn max_supported() {
        let max = Time::new(i64::MAX).dt().year() as u16;
        let year = Year::new(max).unwrap();

        assert_eq!(year.value(), max);
    }

    #[test]
    fn outside_supported_range() {
        let min = Time::new(i64::MIN).dt().year() as u16;
        let max = Time::new(i64::MAX).dt().year() as u16;

        assert!(matches!(
            Year::new(min - 1).unwrap_err(),
            CoreError::Year(_)
        ));

        assert!(matches!(
            Year::new(max + 1).unwrap_err(),
            CoreError::Year(_)
        ));
    }

    #[test]
    fn display() {
        let year = Year::new(2026).unwrap();

        assert_eq!(year.to_string(), "2026");
    }

    #[test]
    fn ordering() {
        let earlier = Year::new(2025).unwrap();
        let later = Year::new(2026).unwrap();

        assert!(earlier < later);
    }
}
