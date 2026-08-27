// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod avin;
mod config;
mod global;

pub use self::global::WORKSPACE;

// ───────────────────────────────────────────────────────────────────────────

use std::env;
use std::path::{Path, PathBuf};

use avin_utils::AvinError;

use self::avin::AvinToml;
use self::config::Config;

const WORKSPACE_ENV: &str = "AVIN_WORKSPACE";
const AVIN_FILE: &str = "AVIN.toml";
const AVIN_FILE_HIDDEN: &str = ".AVIN.toml";

const CONFIG_FILE: &str = "config.toml";
// const DATA_FILE: &str = "data.toml";
// const GUI_FILE: &str = "gui.toml";
// const SECRET_FILE: &str = "secret.toml";

#[derive(Debug)]
pub struct Workspace {
    pub avin: AvinToml,
    pub cfg: Config,
}

impl Workspace {
    pub fn create(_path: &Path) -> Result<(), AvinError> {
        // создает directory, если ее еще нет
        // создает AVIN.toml
        // создает необходимые cfg/*
        // создает workspace dirs
        todo!()
    }

    pub fn open() -> Result<Self, AvinError> {
        let ws_file = Self::locate_workspace_file()?;

        let avin = AvinToml::read(&ws_file)?;
        let cfg = Config::read(&avin.configuration().join(CONFIG_FILE))?;

        Ok(Self { avin, cfg })
    }

    pub fn log(&self) -> &Path {
        self.avin.log()
    }

    pub fn market_data(&self) -> &Path {
        self.avin.market_data()
    }

    pub fn instruments(&self) -> &Path {
        self.avin.instruments()
    }

    pub fn search(&self) -> &Path {
        self.avin.search()
    }

    pub fn test(&self) -> &Path {
        self.avin.test()
    }

    pub fn watchlist(&self) -> &Path {
        self.avin.watchlist()
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
                AVIN_FILE, AVIN_FILE_HIDDEN
            )));
        }

        Err(AvinError::Missing(format!(
            "not an AVIN workspace: {} | {} not found",
            AVIN_FILE, AVIN_FILE_HIDDEN
        )))
    }

}

fn workspace_file_in(dir: &Path) -> Option<PathBuf> {
    let path = dir.join(AVIN_FILE);
    if path.is_file() {
        return Some(path);
    }

    let path = dir.join(AVIN_FILE_HIDDEN);
    if path.is_file() {
        return Some(path);
    }

    None
}
