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
pub enum StorageError {
    Conversion {
        message: String,
        source: Option<Source>,
    },
    Fs {
        message: String,
        source: Option<Source>,
    },
    Save {
        message: String,
        source: Option<Source>,
    },
    Load {
        message: String,
        source: Option<Source>,
    },
    Delete {
        message: String,
        source: Option<Source>,
    },
}

impl StorageError {
    pub fn conversion(msg: impl Into<String>, err: Option<Source>) -> Self {
        Self::Conversion {
            message: msg.into(),
            source: err,
        }
    }

    pub fn fs(msg: impl Into<String>, err: Option<Source>) -> Self {
        Self::Fs {
            message: msg.into(),
            source: err,
        }
    }

    pub fn save(msg: impl Into<String>, err: Option<Source>) -> Self {
        Self::Save {
            message: msg.into(),
            source: err,
        }
    }

    pub fn load(msg: impl Into<String>, err: Option<Source>) -> Self {
        Self::Load {
            message: msg.into(),
            source: err,
        }
    }

    pub fn delete(msg: impl Into<String>, err: Option<Source>) -> Self {
        Self::Delete {
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

impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conversion { message, .. } => write!(f, "{message}"),
            Self::Fs { message, .. } => write!(f, "{message}"),
            Self::Save { message, .. } => write!(f, "{message}"),
            Self::Load { message, .. } => write!(f, "{message}"),
            Self::Delete { message, .. } => write!(f, "{message}"),
        }
    }
}

impl Error for StorageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Conversion { source, .. }
            | Self::Fs { source, .. }
            | Self::Save { source, .. }
            | Self::Load { source, .. }
            | Self::Delete { source, .. } => {
                source.as_deref().map(|err| err as &(dyn Error + 'static))
            }
        }
    }
}
