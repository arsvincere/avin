/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

/// List for selecting the instrument category.
///
/// # ru
/// Перечисление для выбора категории инструмента.
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, strum::Display, strum::EnumIter,
)]
pub enum Category {
    CURRENCY,
    INDEX,
    SHARE,
    BOND,
    FUTURE,
    OPTION,
    ETF,
}
impl Category {
    /// Return category name
    ///
    /// # ru
    /// Возвращает название категории инструмента
    pub fn name(&self) -> &'static str {
        match self {
            Self::CURRENCY => "CURRENCY",
            Self::INDEX => "INDEX",
            Self::SHARE => "SHARE",
            Self::BOND => "BOND",
            Self::FUTURE => "FUTURE",
            Self::OPTION => "OPTION",
            Self::ETF => "ETF",
        }
    }
}
impl From<&str> for Category {
    fn from(value: &str) -> Category {
        match value.to_uppercase().as_str() {
            "CURRENCY" => Category::CURRENCY,
            "INDEX" => Category::INDEX,
            "SHARE" => Category::SHARE,
            "BOND" => Category::BOND,
            "FUTURE" => Category::FUTURE,
            "OPTION" => Category::OPTION,
            "ETF" => Category::ETF,
            _ => panic!("Invalid value for category: {value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        assert_eq!(Category::CURRENCY.name(), "CURRENCY");
        assert_eq!(Category::INDEX.name(), "INDEX");
        assert_eq!(Category::SHARE.name(), "SHARE");
        assert_eq!(Category::BOND.name(), "BOND");
        assert_eq!(Category::FUTURE.name(), "FUTURE");
        assert_eq!(Category::OPTION.name(), "OPTION");
        assert_eq!(Category::ETF.name(), "ETF");
    }

    #[test]
    fn to_str() {
        assert_eq!(Category::CURRENCY.to_string(), "CURRENCY");
        assert_eq!(Category::INDEX.to_string(), "INDEX");
        assert_eq!(Category::SHARE.to_string(), "SHARE");
        assert_eq!(Category::BOND.to_string(), "BOND");
        assert_eq!(Category::FUTURE.to_string(), "FUTURE");
        assert_eq!(Category::OPTION.to_string(), "OPTION");
        assert_eq!(Category::ETF.to_string(), "ETF");
    }
    #[test]
    fn from_str() {
        assert_eq!(Category::from("CURRENCY"), Category::CURRENCY);
        assert_eq!(Category::from("INDEX"), Category::INDEX);
        assert_eq!(Category::from("SHARE"), Category::SHARE);
        assert_eq!(Category::from("BOND"), Category::BOND);
        assert_eq!(Category::from("FUTURE"), Category::FUTURE);
        assert_eq!(Category::from("OPTION"), Category::OPTION);
        assert_eq!(Category::from("ETF"), Category::ETF);
    }
}
