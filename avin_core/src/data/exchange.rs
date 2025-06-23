/****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

/// List for selecting the instrument exchange.
///
/// # ru
/// Перечисление для выбора биржи.
#[derive(Debug, PartialEq, Clone, strum::Display)]
pub enum Exchange {
    MOEX,
}
impl Exchange {
    pub fn name(&self) -> &str {
        match self {
            Self::MOEX => "MOEX",
        }
    }
}
impl From<&str> for Exchange {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "MOEX" => Exchange::MOEX,
            _ => panic!("Invalid exchange: {}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(Exchange::MOEX.to_string(), "MOEX");
        assert_eq!(Exchange::MOEX.name(), "MOEX");
    }
}
