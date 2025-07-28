use std::fmt;

#[derive(Debug)]
pub enum Category {
    CURRENCY,
    INDEX,
    SHARE,
    BOND,
    FUTURE,
    OPTION,
    ETF,
}


impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::CURRENCY => write!(f, "MOEX"),
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
            _ => panic!("not implemented"),
        }
    }
}