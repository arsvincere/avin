// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fmt::Display;
use std::str::FromStr;

use avin_utils::AvinError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstrumentKind {
    Currency,
    Index,
    Stock,
    Future,
    Bond,
    Option,
    ETF,
}

/// Instrument kind.
///
/// Represents the kind of a financial instrument.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use avin_model::InstrumentKind;
///
/// for kind in InstrumentKind::all() {
///     println!("{kind}");
/// }
///
/// // Parsing is case-insensitive.
/// let kind = InstrumentKind::from_str("future").unwrap();
/// assert_eq!(kind, InstrumentKind::Future);
/// ```
impl InstrumentKind {
    /// Returns all supported instrument kinds.
    pub const fn all() -> &'static [Self] {
        &[
            Self::Currency,
            Self::Index,
            Self::Stock,
            Self::Future,
            Self::Bond,
            Self::Option,
            Self::ETF,
        ]
    }
}

impl Display for InstrumentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Currency => f.write_str("Currency"),
            Self::Index => f.write_str("Index"),
            Self::Stock => f.write_str("Stock"),
            Self::Future => f.write_str("Future"),
            Self::Bond => f.write_str("Bond"),
            Self::Option => f.write_str("Option"),
            Self::ETF => f.write_str("ETF"),
        }
    }
}

impl FromStr for InstrumentKind {
    type Err = AvinError;

    /// Parses an instrument kind.
    ///
    /// Parsing is case-insensitive.
    ///
    /// # Errors
    ///
    /// Returns an error if the instrument kind is unknown.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use avin_model::InstrumentKind;
    ///
    /// assert_eq!(
    ///     InstrumentKind::from_str("FuTuRe").unwrap(),
    ///     InstrumentKind::Future
    /// );
    /// assert_eq!(
    ///     InstrumentKind::from_str("ETF").unwrap(),
    ///     InstrumentKind::ETF
    /// );
    ///
    /// assert!(InstrumentKind::from_str("foo").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "currency" => Ok(Self::Currency),
            "index" => Ok(Self::Index),
            "stock" => Ok(Self::Stock),
            "future" => Ok(Self::Future),
            "bond" => Ok(Self::Bond),
            "option" => Ok(Self::Option),
            "etf" => Ok(Self::ETF),
            _ => {
                let available = Self::all()
                    .iter()
                    .map(Self::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");

                let msg = format!(
                    "unknown instrument kind '{}', available=[{}]",
                    s, available
                );

                Err(AvinError::InvalidValue(msg))
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
            InstrumentKind::Currency,
            InstrumentKind::Index,
            InstrumentKind::Stock,
            InstrumentKind::Future,
            InstrumentKind::Bond,
            InstrumentKind::Option,
            InstrumentKind::ETF,
        ];

        assert_eq!(InstrumentKind::all(), expected);
    }

    #[test]
    fn display() {
        assert_eq!(InstrumentKind::Currency.to_string(), "Currency");
        assert_eq!(InstrumentKind::Index.to_string(), "Index");
        assert_eq!(InstrumentKind::Stock.to_string(), "Stock");
        assert_eq!(InstrumentKind::Future.to_string(), "Future");
        assert_eq!(InstrumentKind::Bond.to_string(), "Bond");
        assert_eq!(InstrumentKind::Option.to_string(), "Option");
        assert_eq!(InstrumentKind::ETF.to_string(), "ETF");
    }

    #[test]
    fn from_str() {
        assert_eq!(
            InstrumentKind::from_str("Currency").unwrap(),
            InstrumentKind::Currency
        );
        assert_eq!(
            InstrumentKind::from_str("Index").unwrap(),
            InstrumentKind::Index
        );
        assert_eq!(
            InstrumentKind::from_str("Stock").unwrap(),
            InstrumentKind::Stock
        );
        assert_eq!(
            InstrumentKind::from_str("Future").unwrap(),
            InstrumentKind::Future
        );
        assert_eq!(
            InstrumentKind::from_str("BoNd").unwrap(),
            InstrumentKind::Bond
        );
        assert_eq!(
            InstrumentKind::from_str("OPTION").unwrap(),
            InstrumentKind::Option
        );
        assert_eq!(
            InstrumentKind::from_str("etf").unwrap(),
            InstrumentKind::ETF
        );

        assert!(InstrumentKind::from_str("foo").is_err());
    }
}
