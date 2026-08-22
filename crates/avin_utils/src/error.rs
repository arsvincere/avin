// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum AvinError {
    InvalidValue(String),
    InvalidInstrumentInfo(String),
}

impl std::fmt::Display for AvinError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidValue(s) => write!(f, "{s}"),
            Self::InvalidInstrumentInfo(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for AvinError {}
