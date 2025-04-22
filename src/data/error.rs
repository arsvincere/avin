#[derive(Debug)]
pub enum DataError {
    NotFound(String),
    ReadError(String),
}

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NotFound(s) => write!(f, "NotFound: {}", s),
            Self::ReadError(s) => write!(f, "ReadError: {}", s),
        }
    }
}
