/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

use std::str::FromStr;

use avin_utils::AvinError;
use strum::VariantNames;

/// List for selecting the source of downloading market data.
///
/// # ru
/// Перечисление для выбора источника загрузки рыночных данных.
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    strum::Display,
    strum::EnumIter,
    strum::VariantNames,
)]
pub enum Source {
    TINKOFF,
    MOEXALGO,
}
impl Source {
    /// Return market data source name.
    ///
    /// # ru
    /// Возвращает название источника биржевых данных.
    pub fn name(&self) -> &'static str {
        match self {
            Self::TINKOFF => "TINKOFF",
            Self::MOEXALGO => "MOEXALGO",
        }
    }
}
impl FromStr for Source {
    type Err = AvinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "MOEXALGO" => Ok(Source::MOEXALGO),
            "TINKOFF" => Ok(Source::TINKOFF),
            _ => {
                let msg = format!("{s}, available={:?}", Source::VARIANTS);
                let e = AvinError::InvalidValue(msg);
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        assert_eq!(Source::MOEXALGO.name(), "MOEXALGO");
        assert_eq!(Source::TINKOFF.name(), "TINKOFF");
    }
    #[test]
    fn to_str() {
        assert_eq!(Source::MOEXALGO.to_string(), "MOEXALGO");
        assert_eq!(Source::TINKOFF.to_string(), "TINKOFF");
    }
    #[test]
    fn from_str() {
        assert_eq!(Source::MOEXALGO, "MOEXALGO".parse().unwrap());
        assert_eq!(Source::TINKOFF, Source::from_str("TINKOFF").unwrap())
    }
}
