/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

/// List for selecting the source of downloading market data.
///
/// # ru
/// Перечисление для выбора источника загрузки рыночных данных.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, strum::Display)]
pub enum Source {
    MOEXALGO,
    TINKOFF,
}
impl Source {
    /// Return market data source name.
    ///
    /// # ru
    /// Возвращает название источника биржевых данных.
    pub fn name(&self) -> &'static str {
        match self {
            Self::MOEXALGO => "MOEXALGO",
            Self::TINKOFF => "TINKOFF",
        }
    }
}
impl From<&str> for Source {
    fn from(value: &str) -> Source {
        match value.to_uppercase().as_str() {
            "MOEXALGO" => Source::MOEXALGO,
            "TINKOFF" => Source::TINKOFF,
            _ => panic!("Invalid source: {value}"),
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
        assert_eq!(Source::MOEXALGO, "MOEXALGO".into());
        assert_eq!(Source::TINKOFF, "TiNkoFf".into());
    }
}
