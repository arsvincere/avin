/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      H1ghSpeed
 * E-MAIL:
 * LICENSE:     MIT
 ****************************************************************************/

/// List for selecting the source of downloading market data.
///
/// # ru
/// Перечисление для выбора источника загрузки рыночных данных.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Source {
    MOEX,
    TINKOFF,
}
impl Source {
    /// Return market data source name
    ///
    /// # ru
    /// Возвращает название источника биржевых данных
    pub fn name(&self) -> String {
        match self {
            Self::MOEX => "MOEX".to_string(),
            Self::TINKOFF => "TINKOFF".to_string(),
        }
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::MOEX => write!(f, "MOEX"),
            Source::TINKOFF => write!(f, "TINKOFF"),
        }
    }
}

impl From<&str> for Source {
    fn from(value: &str) -> Source {
        let value = value.to_uppercase();
        match value.as_str() {
            "MOEX" => Source::MOEX,
            "TINKOFF" => Source::TINKOFF,
            _ => todo!("not implemented {}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(Source::MOEX.to_string(), "MOEX");
        assert_eq!(Source::TINKOFF.to_string(), "TINKOFF");
    }
    #[test]
    fn from_str() {
        assert_eq!(Source::MOEX, "moex".into());
        assert_eq!(Source::TINKOFF, "TiNkoFf".into());
    }
}
