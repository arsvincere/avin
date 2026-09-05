// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;

use crate::{CoreError, Time};

/// Half-open time range `[begin, end)`.
///
/// The `begin` bound is included.
/// The `end` bound is excluded.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use avin_core::{Time, TimeRange};
///
/// let begin = Time::from_str("2025-01-01").unwrap();
/// let end = Time::from_str("2026-01-01").unwrap();
/// let range = TimeRange::new(begin, end).unwrap();
///
/// assert_eq!(range.to_string(), "[2025-01-01, 2026-01-01)");
///
/// let inside = Time::from_str("2025-01-02 12:55:03").unwrap();
/// assert!(range.contains(inside));
///
/// let outside = Time::from_str("2020-01-01").unwrap();
/// assert!(!range.contains(outside));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeRange {
    begin: Time,
    end: Time,
}

impl TimeRange {
    /// Creates a new time range with the given begin and end bounds.
    ///
    /// # Errors
    ///
    /// Returns an error if `begin > end`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use avin_core::{Time, TimeRange};
    ///
    /// let begin = Time::from_str("2025-01-01").unwrap();
    /// let end = Time::from_str("2026-01-01").unwrap();
    /// let range = TimeRange::new(begin, end);
    ///
    /// assert!(range.is_ok());
    ///
    /// let begin = Time::from_str("2025-01-01").unwrap();
    /// let end = Time::from_str("2024-01-01").unwrap();
    /// let range = TimeRange::new(begin, end);
    ///
    /// assert!(range.is_err());
    /// ```
    pub fn new(begin: Time, end: Time) -> Result<Self, CoreError> {
        if begin > end {
            return Err(CoreError::TimeRange(format!(
                "TimeRange begin > end [{begin}, {end})"
            )));
        }

        Ok(TimeRange { begin, end })
    }

    /// Returns the inclusive beginning bound of the range.
    pub fn begin(&self) -> Time {
        self.begin
    }

    /// Returns the exclusive end bound of the range.
    pub fn end(&self) -> Time {
        self.end
    }

    /// Checks whether the given value is within the range.
    ///
    /// The `begin` bound is included.
    /// The `end` bound is excluded.
    pub fn contains(&self, time: Time) -> bool {
        self.begin <= time && time < self.end
    }
}

impl Display for TimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {})", self.begin, self.end)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn bounds() {
        let begin = Time::from_str("2025-01-01").unwrap();
        let end = Time::from_str("2026-01-01").unwrap();
        let range = TimeRange::new(begin, end).unwrap();

        assert_eq!(range.begin(), begin);
        assert_eq!(range.end(), end);
    }

    #[test]
    fn invalid_range() {
        let begin = Time::from_str("2026-01-01").unwrap();
        let end = Time::from_str("2025-01-01").unwrap();

        assert!(matches!(
            TimeRange::new(begin, end).unwrap_err(),
            CoreError::TimeRange(_)
        ));
    }

    #[test]
    fn empty_range() {
        let time = Time::from_str("2025-01-01").unwrap();
        let range = TimeRange::new(time, time).unwrap();

        assert!(!range.contains(time));
    }

    #[test]
    fn contains() {
        let begin = Time::from_str("2025-01-01").unwrap();
        let end = Time::from_str("2026-01-01").unwrap();
        let range = TimeRange::new(begin, end).unwrap();

        let inside = Time::from_str("2025-06-01").unwrap();
        let before = Time::from_str("2024-12-31").unwrap();
        let after = Time::from_str("2026-01-02").unwrap();

        assert!(range.contains(inside));

        assert!(!range.contains(before));
        assert!(!range.contains(after));
    }

    #[test]
    fn contains_bounds() {
        let begin = Time::from_str("2025-01-01").unwrap();
        let end = Time::from_str("2026-01-01").unwrap();
        let range = TimeRange::new(begin, end).unwrap();

        assert!(range.contains(begin));
        assert!(!range.contains(end));
    }

    #[test]
    fn display() {
        let begin = Time::from_str("2025-01-01").unwrap();
        let end = Time::from_str("2026-01-01").unwrap();
        let range = TimeRange::new(begin, end).unwrap();

        assert_eq!(range.to_string(), "[2025-01-01, 2026-01-01)");
    }
}
