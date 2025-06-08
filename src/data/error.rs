/****************************************************************************
 * URL:         http://arsvincere.com
 * AUTHOR:      Alex Avin
 * E-MAIL:      mr.alexavin@gmail.com
 * LICENSE:     MIT
 ****************************************************************************/

// TODO: rename -> AvinError
// TODO: move -> utils

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
