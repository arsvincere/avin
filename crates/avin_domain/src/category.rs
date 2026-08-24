// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;
use std::str::FromStr;

use avin_utils::AvinError;

// TODO: rename Currency -> CurrencyPair???
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Index,
    Share,
    Future,
    Bond,
    Option,
    ETF,
    /// Currency pair / FX market instruments.
    Currency,
}

/// Category.
///
/// Represents a financial instrument category.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use avin_domain::Category;
///
/// for category in Category::all() {
///     println!("{category}");
/// }
///
/// // Parsing is case-insensitive.
/// let category = Category::from_str("future").unwrap();
/// assert_eq!(category, Category::Future);
/// ```
impl Category {
    /// Returns all supported categories.
    pub const fn all() -> &'static [Self] {
        &[
            Self::Index,
            Self::Share,
            Self::Future,
            Self::Bond,
            Self::Option,
            Self::ETF,
            Self::Currency,
        ]
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Index => f.write_str("INDEX"),
            Self::Share => f.write_str("SHARE"),
            Self::Future => f.write_str("FUTURE"),
            Self::Bond => f.write_str("BOND"),
            Self::Option => f.write_str("OPTION"),
            Self::ETF => f.write_str("ETF"),
            Self::Currency => f.write_str("CURRENCY"),
        }
    }
}

impl FromStr for Category {
    type Err = AvinError;

    /// Parses a category.
    ///
    /// Parsing is case-insensitive.
    ///
    /// # Errors
    ///
    /// Returns an error if the category is unknown.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use avin_domain::Category;
    ///
    /// assert_eq!(
    ///     Category::from_str("FuTuRe").unwrap(),
    ///     Category::Future
    /// );
    /// assert_eq!(
    ///     Category::from_str("ETF").unwrap(),
    ///     Category::ETF
    /// );
    ///
    /// assert!(Category::from_str("foo").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "index" => Ok(Self::Index),
            "share" => Ok(Self::Share),
            "future" => Ok(Self::Future),
            "bond" => Ok(Self::Bond),
            "option" => Ok(Self::Option),
            "etf" => Ok(Self::ETF),
            "currency" => Ok(Self::Currency),
            _ => {
                let available = Self::all()
                    .iter()
                    .map(Self::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");

                let msg = format!(
                    "unknown category '{}', available=[{}]",
                    s, available
                );

                Err(AvinError::Value(msg))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        let expected = [
            Category::Index,
            Category::Share,
            Category::Future,
            Category::Bond,
            Category::Option,
            Category::ETF,
            Category::Currency,
        ];

        assert_eq!(Category::all(), expected);
    }

    #[test]
    fn display() {
        assert_eq!(Category::Index.to_string(), "INDEX");
        assert_eq!(Category::Share.to_string(), "SHARE");
        assert_eq!(Category::Future.to_string(), "FUTURE");
        assert_eq!(Category::Bond.to_string(), "BOND");
        assert_eq!(Category::Option.to_string(), "OPTION");
        assert_eq!(Category::ETF.to_string(), "ETF");
        assert_eq!(Category::Currency.to_string(), "CURRENCY");
    }

    #[test]
    fn from_str() {
        assert_eq!(Category::from_str("INDEX").unwrap(), Category::Index);
        assert_eq!(Category::from_str("SHARE").unwrap(), Category::Share);
        assert_eq!(Category::from_str("FUTURE").unwrap(), Category::Future);
        assert_eq!(Category::from_str("BoNd").unwrap(), Category::Bond);
        assert_eq!(Category::from_str("OPTION").unwrap(), Category::Option);
        assert_eq!(Category::from_str("etf").unwrap(), Category::ETF);
        assert_eq!(
            Category::from_str("CURRENCY").unwrap(),
            Category::Currency
        );

        assert!(Category::from_str("foo").is_err());
    }
}
