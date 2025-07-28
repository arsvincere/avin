/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      H1ghSpeed
 * E-MAIL:
 * LICENSE:     MIT
 ****************************************************************************/

/// List for selecting the instrument category.
///
/// # ru
/// Перечисление для выбора категории инструмента.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
    pub fn name(&self) -> &str {
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

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::CURRENCY => write!(f, "CURRENCY"),
            Category::INDEX => write!(f, "INDEX"),
            Category::SHARE => write!(f, "SHARE"),
            Category::BOND => write!(f, "BOND"),
            Category::FUTURE => write!(f, "FUTURE"),
            Category::OPTION => write!(f, "OPTION"),
            Category::ETF => write!(f, "ETF"),
        }
    }
}

impl From<&str> for Category {
    fn from(value: &str) -> Category {
        let value = value.to_uppercase();
        match value.as_str() {
            "CURRENCY" => Category::CURRENCY,
            "INDEX" => Category::INDEX,
            "SHARE" => Category::SHARE,
            "BOND" => Category::BOND,
            "FUTURE" => Category::FUTURE,
            "OPTION" => Category::OPTION,
            "ETF" => Category::ETF,
            _ => todo!("not implemented {}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_to_string() {
        assert_eq!(Category::CURRENCY.to_string(), "CURRENCY");
        assert_eq!(Category::INDEX.to_string(), "INDEX");
        assert_eq!(Category::SHARE.to_string(), "SHARE");
        assert_eq!(Category::BOND.to_string(), "BOND");
        assert_eq!(Category::FUTURE.to_string(), "FUTURE");
        assert_eq!(Category::OPTION.to_string(), "OPTION");
        assert_eq!(Category::ETF.to_string(), "ETF");
    }
    #[test]
    fn category_from_str() {
        assert_eq!(Category::from("CURRENCY"), Category::CURRENCY);
        assert_eq!(Category::from("INDEX"), Category::INDEX);
        assert_eq!(Category::from("SHARE"), Category::SHARE);
        assert_eq!(Category::from("BOND"), Category::BOND);
        assert_eq!(Category::from("FUTURE"), Category::FUTURE);
        assert_eq!(Category::from("OPTION"), Category::OPTION);
        assert_eq!(Category::from("ETF"), Category::ETF);
    }
}
