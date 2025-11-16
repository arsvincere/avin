/*****************************************************************************
 * URL:         http://avin.info
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

#[derive(Debug, Clone)]
pub enum AvinError {
    InvalidValue(String),
    NotFound(String),
    NotLoaded(String),
    NotExist(String), // TODO: rename -> Unavailable
    IOError(String),
}

impl std::fmt::Display for AvinError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidValue(s) => write!(f, "InvalidValue: {s}"),
            Self::NotFound(s) => write!(f, "NotFound: {s}"),
            Self::NotLoaded(s) => write!(f, "NotLoaded: {s}"),
            Self::NotExist(s) => write!(f, "NotExist: {s}"),
            Self::IOError(s) => write!(f, "IOError: {s}"),
        }
    }
}

impl std::error::Error for AvinError {}
