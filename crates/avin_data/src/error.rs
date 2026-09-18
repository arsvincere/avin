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
pub enum DataError {
    Pack {
        message: String,
        source: Option<Source>,
    },
    Unavailable {
        message: String,
        source: Option<Source>,
    },
    Connect {
        message: String,
        source: Option<Source>,
    },
}

impl DataError {
    pub fn pack(msg: impl Into<String>, err: Option<Source>) -> Self {
        Self::Pack {
            message: msg.into(),
            source: err,
        }
    }

    pub fn unavailable(msg: impl Into<String>, err: Option<Source>) -> Self {
        Self::Unavailable {
            message: msg.into(),
            source: err,
        }
    }

    pub fn connect(msg: impl Into<String>, err: Option<Source>) -> Self {
        Self::Connect {
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

impl Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pack { message, .. } => write!(f, "{message}"),
            Self::Unavailable { message, .. } => write!(f, "{message}"),
            Self::Connect { message, .. } => write!(f, "{message}"),
        }
    }
}

impl Error for DataError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Pack { source, .. }
            | Self::Unavailable { source, .. }
            | Self::Connect { source, .. } => {
                source.as_deref().map(|err| err as &(dyn Error + 'static))
            }
        }
    }
}
