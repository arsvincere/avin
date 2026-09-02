// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum CoreError {
    Time(String),
}

impl Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Time(msg) => f.write_str(msg),
        }
    }
}

impl Error for CoreError {}
