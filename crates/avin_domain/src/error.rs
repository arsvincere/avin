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
pub enum DomainError {
    Exchange(String),
    Category(String),
    Ticker(String),
    InstrumentId {
        message: String,
        source: Option<ErrorSource>,
    },
    InstrumentInfo {
        message: String,
        source: Option<ErrorSource>,
    },
    InstrumentList(String),
    Share(String),
    Future(String),
    Asset(String),
    Bar(String),
    TimeFrame(String),
    Chart(String),
}

impl DomainError {
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

impl Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exchange(msg) => write!(f, "{msg}"),
            Self::Category(msg) => write!(f, "{msg}"),
            Self::Ticker(msg) => write!(f, "{msg}"),
            Self::InstrumentId { message, .. } => write!(f, "{message}"),
            Self::InstrumentInfo { message, .. } => write!(f, "{message}"),
            Self::InstrumentList(msg) => write!(f, "{msg}"),
            Self::Share(msg) => write!(f, "{msg}"),
            Self::Future(msg) => write!(f, "{msg}"),
            Self::Asset(msg) => write!(f, "{msg}"),
            Self::Bar(msg) => write!(f, "{msg}"),
            Self::TimeFrame(msg) => write!(f, "{msg}"),
            Self::Chart(msg) => write!(f, "{msg}"),
        }
    }
}

impl Error for DomainError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Exchange(_) => None,
            Self::Category(_) => None,
            Self::Ticker(_) => None,
            Self::InstrumentInfo { source, .. }
            | Self::InstrumentId { source, .. } => {
                source.as_deref().map(|err| err as &(dyn Error + 'static))
            }
            Self::InstrumentList(_) => None,
            Self::Share(_) => None,
            Self::Future(_) => None,
            Self::Asset(_) => None,
            Self::Bar(_) => None,
            Self::TimeFrame(_) => None,
            Self::Chart(_) => None,
        }
    }
}
