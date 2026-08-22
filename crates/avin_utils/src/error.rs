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
    ParseError(String),

    InvalidInstrumentInfo {
        message: String,
        source: Option<Box<AvinError>>,
    },
}

impl AvinError {
    pub fn report(&self) -> String {
        let mut report = self.to_string();

        let mut source = self.source();
        while let Some(err) = source {
            report.push_str(&format!("\ncaused by: {err}"));
            source = err.source();
        }

        report
    }
}

impl Display for AvinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidValue(message) => write!(f, "{message}"),
            Self::ParseError(message) => write!(f, "{message}"),

            Self::InvalidInstrumentInfo { message, .. } => {
                write!(f, "{message}")
            }
        }
    }
}

impl Error for AvinError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidValue(_) => None,
            Self::ParseError(_) => None,

            Self::InvalidInstrumentInfo { source, .. } => match source {
                Some(error) => Some(error.as_ref()),
                None => None,
            },
        }
    }
}
