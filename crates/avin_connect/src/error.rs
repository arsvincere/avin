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
pub enum ConnectorError {
    Authorization {
        message: String,
        source: Option<ErrorSource>,
    },
    Connection {
        message: String,
        source: Option<ErrorSource>,
    },
    Request {
        message: String,
        source: Option<ErrorSource>,
    },
    Conversion {
        message: String,
        source: Option<ErrorSource>,
    },
}

impl ConnectorError {
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

impl Display for ConnectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorization { message, .. } => write!(f, "{message}"),
            Self::Connection { message, .. } => write!(f, "{message}"),
            Self::Request { message, .. } => write!(f, "{message}"),
            Self::Conversion { message, .. } => write!(f, "{message}"),
        }
    }
}

impl Error for ConnectorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Authorization { source, .. }
            | Self::Connection { source, .. }
            | Self::Request { source, .. }
            | Self::Conversion { source, .. } => {
                source.as_deref().map(|err| err as &(dyn Error + 'static))
            }
        }
    }
}
