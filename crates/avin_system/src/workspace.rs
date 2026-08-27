// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: del it after implementation
#![allow(dead_code)]

use std::env;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use avin_utils::{AvinError, Cmd};

const WORKSPACE_FILE: &str = "AVIN.toml";
const WORKSPACE_FILE_HIDDEN: &str = ".AVIN.toml";
const WORKSPACE_ENV: &str = "AVIN_WORKSPACE";

#[derive(Debug, Deserialize)]
pub struct Workspace {
    format: u32,
    pub dirs: WorkspaceDirs,
}

impl Workspace {
    pub fn open() -> Result<Self, AvinError> {
        let ws_file = Self::locate_workspace_file()?;
        let ws_dir = ws_file.parent().unwrap();

        let toml_text = Cmd::read(&ws_file)?;

        let mut ws: Self = toml::from_str(&toml_text).map_err(|err| {
            AvinError::Parse(format!(
                "failed to parse {}: {err}",
                ws_file.display()
            ))
        })?;

        ws.dirs.resolve(ws_dir);

        Ok(ws)
    }

    fn locate_workspace_file() -> Result<PathBuf, AvinError> {
        let cur_dir = env::current_dir().map_err(|err| AvinError::Io {
            message: "failed to get current directory".to_string(),
            source: err,
        })?;

        if let Some(ws_file) = workspace_file_in(&cur_dir) {
            return Ok(ws_file);
        }

        if let Some(ws_dir) = env::var_os(WORKSPACE_ENV) {
            let ws_dir = PathBuf::from(ws_dir);

            if let Some(ws_file) = workspace_file_in(&ws_dir) {
                return Ok(ws_file);
            }

            return Err(AvinError::Missing(format!(
                "{} | {} not found in {WORKSPACE_ENV}",
                WORKSPACE_FILE, WORKSPACE_FILE_HIDDEN
            )));
        }

        Err(AvinError::Missing(format!(
            "not an AVIN workspace: {} | {} not found",
            WORKSPACE_FILE, WORKSPACE_FILE_HIDDEN
        )))
    }
}

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

fn workspace_file_in(dir: &Path) -> Option<PathBuf> {
    let path = dir.join(WORKSPACE_FILE);
    if path.is_file() {
        return Some(path);
    }

    let path = dir.join(WORKSPACE_FILE_HIDDEN);
    if path.is_file() {
        return Some(path);
    }

    None
}

fn resolve(dir: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        dir.join(path)
    }
}
