// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum SystemError {
    Value(String),   // invalid value
    Parse(String),   // parse error
    Key(String),     // key missing
    Missing(String), // missing value
    Process(String),
    Io {
        message: String,
        source: std::io::Error,
    },
    // Zip {
    //     message: String,
    //     source: zip::result::ZipError,
    // },
    // Polars {
    //     message: String,
    //     source: PolarsError,
    // },
    InstrumentInfo {
        message: String,
        source: Box<SystemError>,
    },
}

impl SystemError {
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

impl Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(msg) => write!(f, "{msg}"),
            Self::Parse(msg) => write!(f, "{msg}"),
            Self::Key(msg) => write!(f, "{msg}"),
            Self::Missing(msg) => write!(f, "{msg}"),
            Self::Process(msg) => write!(f, "{msg}"),

            Self::Io { message, .. } => write!(f, "{message}"),
            // Self::Zip { message, .. } => write!(f, "{message}"),
            // Self::Polars { message, .. } => write!(f, "{message}"),
            Self::InstrumentInfo { message, .. } => {
                write!(f, "{message}")
            }
        }
    }
}

impl Error for SystemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Value(_) => None,
            Self::Parse(_) => None,
            Self::Key(_) => None,
            Self::Missing(_) => None,
            Self::Process(_) => None,
            Self::Io { source, .. } => Some(source),
            // Self::Zip { source, .. } => Some(source),
            // Self::Polars { source, .. } => Some(source),
            Self::InstrumentInfo { source, .. } => Some(source),
        }
    }
}
