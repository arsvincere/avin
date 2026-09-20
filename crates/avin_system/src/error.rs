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
pub enum SystemError {
    AvinToml {
        message: String,
        source: Option<Source>,
    },
    Config {
        message: String,
        source: Option<Source>,
    },
    Data {
        message: String,
        source: Option<Source>,
    },
    Secret {
        message: String,
        source: Option<Source>,
    },
    Workspace {
        message: String,
        source: Option<Source>,
    },
    Logger {
        message: String,
        source: Option<Source>,
    },
}

impl SystemError {
    pub fn avin_toml(msg: impl Into<String>, err: Option<Source>) -> Self {
        Self::AvinToml {
            message: msg.into(),
            source: err,
        }
    }

    pub fn config(msg: impl Into<String>, err: Option<Source>) -> Self {
        Self::Config {
            message: msg.into(),
            source: err,
        }
    }

    pub fn data(msg: impl Into<String>, err: Option<Source>) -> Self {
        Self::Data {
            message: msg.into(),
            source: err,
        }
    }

    pub fn secret(msg: impl Into<String>, err: Option<Source>) -> Self {
        Self::Secret {
            message: msg.into(),
            source: err,
        }
    }

    pub fn workspace(msg: impl Into<String>, err: Option<Source>) -> Self {
        Self::Workspace {
            message: msg.into(),
            source: err,
        }
    }

    pub fn logger(msg: impl Into<String>, err: Option<Source>) -> Self {
        Self::Logger {
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

impl Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AvinToml { message, .. } => write!(f, "{message}"),
            Self::Config { message, .. } => write!(f, "{message}"),
            Self::Data { message, .. } => write!(f, "{message}"),
            Self::Secret { message, .. } => write!(f, "{message}"),
            Self::Workspace { message, .. } => write!(f, "{message}"),
            Self::Logger { message, .. } => write!(f, "{message}"),
        }
    }
}

impl Error for SystemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::AvinToml { source, .. }
            | Self::Config { source, .. }
            | Self::Data { source, .. }
            | Self::Secret { source, .. }
            | Self::Workspace { source, .. }
            | Self::Logger { source, .. } => {
                source.as_deref().map(|err| err as &(dyn Error + 'static))
            }
        }
    }
}
