/****************************************************************************
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
impl From<&str> for Source {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "MOEX" => Source::MOEX,
            "TINKOFF" => Source::TINKOFF,
            _ => panic!("Invalid value for Source: {value}"),
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
