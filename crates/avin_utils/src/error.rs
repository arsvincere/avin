// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum AvinError {
    InvalidValue(String),
    InvalidInstrumentInfo { source: Box<AvinError> },
}

impl Display for AvinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidValue(msg) => {
                write!(f, "InvalidValue\n    message: {msg}")
            }

            Self::InvalidInstrumentInfo { source } => {
                write!(f, "InvalidInstrumentInfo\ncaused by: {source}")
            }
        }
    }
}

impl Error for AvinError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidValue(_) => None,
            Self::InvalidInstrumentInfo { source } => Some(source.as_ref()),
        }
    }
}
