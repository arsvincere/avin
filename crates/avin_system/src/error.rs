// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::error::Error;
use std::fmt::Display;

use avin_core::CoreError;
use avin_domain::DomainError;

#[derive(Debug)]
pub enum SystemError {
    Core {
        message: String,
        source: CoreError,
    },
    Domain {
        message: String,
        source: DomainError,
    },
    Io {
        message: String,
        source: std::io::Error,
    },

    Value(String),   // invalid value
    Parse(String),   // parse error
    Key(String),     // key missing
    Missing(String), // missing value
    Process(String),
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
            Self::Core { message, .. } => write!(f, "{message}"),
            Self::Domain { message, .. } => write!(f, "{message}"),
            Self::Io { message, .. } => write!(f, "{message}"),

            Self::Value(message) => write!(f, "{message}"),
            Self::Parse(message) => write!(f, "{message}"),
            Self::Key(message) => write!(f, "{message}"),
            Self::Missing(message) => write!(f, "{message}"),
            Self::Process(message) => write!(f, "{message}"),
            Self::InstrumentInfo { message, .. } => write!(f, "{message}"),
        }
    }
}

impl Error for SystemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Core { source, .. } => Some(source),
            Self::Domain { source, .. } => Some(source),
            Self::Io { source, .. } => Some(source),

            Self::Value(_) => None,
            Self::Parse(_) => None,
            Self::Key(_) => None,
            Self::Missing(_) => None,
            Self::Process(_) => None,
            Self::InstrumentInfo { source, .. } => Some(source),
        }
    }
}
