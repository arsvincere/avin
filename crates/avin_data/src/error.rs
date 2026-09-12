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
pub enum DataError {
    TmpError {
        message: String,
        source: Option<ErrorSource>,
    },
    Unavailable {
        message: String,
        source: Option<ErrorSource>,
    },
    Connect {
        message: String,
        source: Option<ErrorSource>,
    },
}

impl DataError {
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
            Self::TmpError { message, .. } => write!(f, "{message}"),
            Self::Unavailable { message, .. } => write!(f, "{message}"),
            Self::Connect { message, .. } => write!(f, "{message}"),
        }
    }
}

impl Error for DataError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TmpError { source, .. }
            | Self::Unavailable { source, .. }
            | Self::Connect { source, .. } => {
                source.as_deref().map(|err| err as &(dyn Error + 'static))
            }
        }
    }
}
