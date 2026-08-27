// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::path::Path;

use serde::Deserialize;

use avin_utils::AvinError;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[allow(dead_code)]
    format: u32,
    pub default: ConfigDefault,
    pub log: ConfigLog,
}

impl Config {
    pub(super) fn read(path: &Path) -> Result<Self, AvinError> {
        avin_utils::read_toml(path)
    }
}

#[derive(Debug, Deserialize)]
pub struct ConfigDefault {
    data_provider: String,
    watchlist: String,
    bars_count: usize,
    tick_days: usize,
}

impl ConfigDefault {
    pub fn data_provider(&self) -> &str {
        &self.data_provider
    }

    pub fn watchlist(&self) -> &str {
        &self.watchlist
    }

    pub fn bars_count(&self) -> usize {
        self.bars_count
    }

    pub fn tick_days(&self) -> usize {
        self.tick_days
    }
}

#[derive(Debug, Deserialize)]
pub struct ConfigLog {
    history: usize,
    debug: bool,
    info: bool,
}

impl ConfigLog {
    pub fn history(&self) -> usize {
        self.history
    }

    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn info(&self) -> bool {
        self.info
    }
}
