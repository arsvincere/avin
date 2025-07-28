use std::fmt;

#[derive(Debug)]
pub enum Source {
    MOEX,
    TINKOFF
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
            _ => panic!("not implemented"),
        }
    }
}