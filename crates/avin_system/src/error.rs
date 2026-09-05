// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::error::Error;
use std::fmt::Display;

type ErrorSource = Box<dyn Error + Send + Sync + 'static>;

#[derive(Debug)]
pub enum SystemError {
    Io {
        message: String,
        source: std::io::Error,
    },
    ParseToml {
        message: String,
        source: toml::de::Error,
    },
    AvinToml {
        message: String,
        source: Option<ErrorSource>,
    },
    Config {
        message: String,
        source: Option<ErrorSource>,
    },
    DataManifest {
        message: String,
        source: Option<ErrorSource>,
    },
    Secret {
        message: String,
        source: Option<ErrorSource>,
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
            Self::Io { message, .. } => write!(f, "{message}"),
            Self::ParseToml { message, .. } => write!(f, "{message}"),
            Self::AvinToml { message, .. } => write!(f, "{message}"),
            Self::Config { message, .. } => write!(f, "{message}"),
            Self::DataManifest { message, .. } => write!(f, "{message}"),
            Self::Secret { message, .. } => write!(f, "{message}"),

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
            Self::Io { source, .. } => Some(source),
            Self::ParseToml { source, .. } => Some(source),
            Self::AvinToml { source, .. }
            | Self::Config { source, .. }
            | Self::DataManifest { source, .. }
            | Self::Secret { source, .. } => {
                source.as_deref().map(|err| err as &(dyn Error + 'static))
            }

            Self::Value(_) => None,
            Self::Parse(_) => None,
            Self::Key(_) => None,
            Self::Missing(_) => None,
            Self::Process(_) => None,
            Self::InstrumentInfo { source, .. } => Some(source),
        }
    }
}
