// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::error::Error;
use std::fmt::Display;

use avin_core::CoreError;

#[derive(Debug)]
pub enum DomainError {
    Core {
        context: String,
        source: CoreError,
    },
    Exchange(String),
    Category(String),
    Ticker(String),
    InstrumentId(String),
    InstrumentInfo {
        context: String,
        source: Option<Box<DomainError>>,
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
            Self::Core { context, .. } => write!(f, "{context}"),
            Self::Exchange(msg) => write!(f, "{msg}"),
            Self::Category(msg) => write!(f, "{msg}"),
            Self::Ticker(msg) => write!(f, "{msg}"),
            Self::InstrumentId(msg) => write!(f, "{msg}"),
            Self::InstrumentInfo { context, .. } => {
                write!(f, "{context}")
            }
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
            Self::Core { source, .. } => Some(source),
            Self::Exchange(_) => None,
            Self::Category(_) => None,
            Self::Ticker(_) => None,
            Self::InstrumentId(_) => None,
            Self::InstrumentInfo { source, .. } => match source {
                Some(source) => Some(source),
                None => None,
            },
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
