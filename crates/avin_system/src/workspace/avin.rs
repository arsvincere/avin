// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::path::{Path, PathBuf};

use serde::Deserialize;

use avin_utils::AvinError;

#[derive(Debug, Deserialize)]
pub struct AvinToml {
    #[allow(dead_code)]
    format: u32,
    dirs: AvinTomlDirs,
}

impl AvinToml {
    pub(super) fn read(path: &Path) -> Result<Self, AvinError> {
        let mut avin: Self = avin_utils::read_toml(path)?;

        let ws_dir = path.parent().unwrap();
        avin.dirs.resolve(ws_dir);

        Ok(avin)
    }

    pub(super) fn configuration(&self) -> &Path {
        &self.dirs.configuration
    }

    pub fn log(&self) -> &Path {
        &self.dirs.log
    }

    pub fn market_data(&self) -> &Path {
        &self.dirs.market_data
    }

    pub fn instruments(&self) -> &Path {
        &self.dirs.instruments
    }

    pub fn search(&self) -> &Path {
        &self.dirs.search
    }

    pub fn test(&self) -> &Path {
        &self.dirs.test
    }

    pub fn watchlist(&self) -> &Path {
        &self.dirs.watchlist
    }
}

#[derive(Debug, Deserialize)]
pub struct AvinTomlDirs {
    log: PathBuf,
    configuration: PathBuf,
    market_data: PathBuf,
    instruments: PathBuf,
    search: PathBuf,
    test: PathBuf,
    watchlist: PathBuf,
}

impl AvinTomlDirs {
    fn resolve(&mut self, root: &Path) {
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
