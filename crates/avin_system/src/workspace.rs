// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: del it after implementation
#![allow(dead_code)]

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use avin_utils::AvinError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub dirs: WorkspaceCfg,
}

impl Workspace {
    pub fn open() -> Result<Self, AvinError> {
        let _path = Self::find()?;
        todo!()
        // let text = Cmd::read(&path)?;
        // let ws: Workspace = toml::from_str(&text).unwrap();
        //
        // ws
    }

    fn find() -> Result<PathBuf, AvinError> {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceCfg {
    log: String,
    configuration: String,
    market_data: String,
    instrumetns: String,
    search: String,
    test: String,
    watchlist: String,
}
