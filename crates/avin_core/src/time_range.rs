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
/// let time = Time::from_str("2025-01-02 12:55:03").unwrap();
/// assert!(range.contains(time));
///
/// let time = Time::from_str("2020-01-01").unwrap();
/// assert!(!range.contains(time));
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

    /// Returns the begin of the range.
    pub fn begin(&self) -> Time {
        self.begin
    }

    /// Returns the end of the range.
    pub fn end(&self) -> Time {
        self.end
    }

    /// Checks whether the given value is within the range.
    ///
    /// `begin` value are included.
    /// `end` value are not included.
    pub fn contains(&self, time: Time) -> bool {
        self.begin <= time && time < self.end
    }
}

impl Display for TimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {})", self.begin, self.end)
    }
}
