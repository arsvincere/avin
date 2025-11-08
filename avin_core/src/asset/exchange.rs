/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

/// List for selecting the instrument exchange.
///
/// # ru
/// Перечисление для выбора биржи.
#[derive(Debug, PartialEq, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum Exchange {
    MOEX,
}
impl Exchange {
    /// Return exchange name
    ///
    /// # ru
    /// Возвращает название биржи
    pub fn name(&self) -> &'static str {
        match self {
            Self::MOEX => "MOEX",
        }
    }
}
impl From<&str> for Exchange {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "MOEX" => Exchange::MOEX,
            _ => panic!("Invalid exchange: {value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        assert_eq!(Exchange::MOEX.name(), "MOEX");
    }
    #[test]
    fn to_str() {
        assert_eq!(Exchange::MOEX.to_string(), "MOEX");
    }
    #[test]
    fn from_str() {
        assert_eq!(Exchange::from("MOEX"), Exchange::MOEX);
    }
}
