// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WorkspaceDirs {
    log: PathBuf,
    configuration: PathBuf,
    market_data: PathBuf,
    instruments: PathBuf,
    search: PathBuf,
    test: PathBuf,
    watchlist: PathBuf,
}

impl WorkspaceDirs {
    pub fn log(&self) -> PathBuf {
        self.log.clone()
    }

    pub fn configuration(&self) -> PathBuf {
        self.configuration.clone()
    }

    pub fn market_data(&self) -> PathBuf {
        self.market_data.clone()
    }

    pub fn instruments(&self) -> PathBuf {
        self.instruments.clone()
    }

    pub fn search(&self) -> PathBuf {
        self.search.clone()
    }

    pub fn test(&self) -> PathBuf {
        self.test.clone()
    }

    pub fn watchlist(&self) -> PathBuf {
        self.watchlist.clone()
    }

    pub(crate) fn resolve(&mut self, root: &Path) {
        self.log = resolve(root, &self.log);
        self.configuration = resolve(root, &self.configuration);
        self.market_data = resolve(root, &self.market_data);
        self.instruments = resolve(root, &self.instruments);
        self.search = resolve(root, &self.search);
        self.test = resolve(root, &self.test);
        self.watchlist = resolve(root, &self.watchlist);
    }
}

fn resolve(dir: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        dir.join(path)
    }
}
