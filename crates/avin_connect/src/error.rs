// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::error::Error;
use std::fmt::Display;

type Source = Box<dyn Error + Send + Sync + 'static>;

#[derive(Debug)]
pub enum ConnectorError {
    Authorization {
        message: String,
        source: Option<Source>,
    },
    Connection {
        message: String,
        source: Option<Source>,
    },
    Request {
        message: String,
        source: Option<Source>,
    },
    Conversion {
        message: String,
        source: Option<Source>,
    },
}

impl ConnectorError {
    pub fn authorization(
        msg: impl Into<String>,
        err: Option<Source>,
    ) -> Self {
        Self::Authorization {
            message: msg.into(),
            source: err,
        }
    }

    pub fn connection(msg: impl Into<String>, err: Option<Source>) -> Self {
        Self::Connection {
            message: msg.into(),
            source: err,
        }
    }

    pub fn request(msg: impl Into<String>, err: Option<Source>) -> Self {
        Self::Request {
            message: msg.into(),
            source: err,
        }
    }

    pub fn conversion(msg: impl Into<String>, err: Option<Source>) -> Self {
        Self::Conversion {
            message: msg.into(),
            source: err,
        }
    }

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
