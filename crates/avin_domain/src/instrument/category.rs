// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;
use std::str::FromStr;

use crate::DomainError;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Index,
    Share,
    Future,
    Bond,
    Option,
    Etf,
    CurrencyPair,
}

impl Category {
    /// Returns all supported categories.
    pub const fn all() -> &'static [Self] {
        &[
            Self::Index,
            Self::Share,
            Self::Future,
            Self::Bond,
            Self::Option,
            Self::Etf,
            Self::CurrencyPair,
        ]
    }

    /// Returns a stable machine-readable identifier suitable for persistence
    /// and serialization.
    pub const fn key(&self) -> &'static str {
        match self {
            Self::Index => "index",
            Self::Share => "share",
            Self::Future => "future",
            Self::Bond => "bond",
            Self::Option => "option",
            Self::Etf => "etf",
            Self::CurrencyPair => "currency_pair",
        }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Index => f.write_str("Index"),
            Self::Share => f.write_str("Share"),
            Self::Future => f.write_str("Future"),
            Self::Bond => f.write_str("Bond"),
            Self::Option => f.write_str("Option"),
            Self::Etf => f.write_str("ETF"),
            Self::CurrencyPair => f.write_str("Currency pair"),
        }
    }
}

impl FromStr for Category {
    type Err = DomainError;

    /// Parses a category key.
    ///
    /// Parsing is case-insensitive.
    ///
    /// # Errors
    ///
    /// Returns an error if the category key is unknown.
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
    ///     Category::Etf
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
            "etf" => Ok(Self::Etf),
            "currency_pair" => Ok(Self::CurrencyPair),
            _ => {
                let available = Self::all()
                    .iter()
                    .map(|category| category.key())
                    .collect::<Vec<_>>()
                    .join(", ");

                let msg = format!(
                    "unknown category key '{}', available=[{}]",
                    s, available
                );

                Err(DomainError::Category(msg))
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
            Category::Etf,
            Category::CurrencyPair,
        ];

        assert_eq!(Category::all(), expected);
    }

    #[test]
    fn key() {
        assert_eq!(Category::Index.key(), "index");
        assert_eq!(Category::Share.key(), "share");
        assert_eq!(Category::Future.key(), "future");
        assert_eq!(Category::Bond.key(), "bond");
        assert_eq!(Category::Option.key(), "option");
        assert_eq!(Category::Etf.key(), "etf");
        assert_eq!(Category::CurrencyPair.key(), "currency_pair");
    }

    #[test]
    fn display() {
        assert_eq!(Category::Index.to_string(), "Index");
        assert_eq!(Category::Share.to_string(), "Share");
        assert_eq!(Category::Future.to_string(), "Future");
        assert_eq!(Category::Bond.to_string(), "Bond");
        assert_eq!(Category::Option.to_string(), "Option");
        assert_eq!(Category::Etf.to_string(), "ETF");
        assert_eq!(Category::CurrencyPair.to_string(), "Currency pair");
    }

    #[test]
    fn from_str() {
        for category in Category::all().iter() {
            assert_eq!(
                Category::from_str(category.key()).unwrap(),
                *category
            );
        }

        assert_eq!(Category::from_str("sHarE").unwrap(), Category::Share);

        assert!(matches!(
            Category::from_str("foo").unwrap_err(),
            DomainError::Category(_)
        ));
    }
}
