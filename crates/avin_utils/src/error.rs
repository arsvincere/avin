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
    Value(String),   // invalid value
    Parse(String),   // parse error
    Key(String),     // key missing
    Missing(String), // missing value

    // TODO: Decide whether InstrumentInfo always requires a source error.
    // If yes, replace Option<Box<AvinError>> with Box<AvinError>.
    InstrumentInfo {
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
            Self::Value(msg) => write!(f, "{msg}"),
            Self::Parse(msg) => write!(f, "{msg}"),
            Self::Key(msg) => write!(f, "{msg}"),
            Self::Missing(msg) => write!(f, "{msg}"),

            Self::InstrumentInfo { message, .. } => {
                write!(f, "{message}")
            }
        }
    }
}

impl Error for AvinError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Value(_) => None,
            Self::Parse(_) => None,
            Self::Key(_) => None,
            Self::Missing(_) => None,

            Self::InstrumentInfo { source, .. } => match source {
                Some(error) => Some(error.as_ref()),
                None => None,
            },
        }
    }
}
