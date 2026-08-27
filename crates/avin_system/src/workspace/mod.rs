// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod config;
mod dirs;

// ───────────────────────────────────────────────────────────────────────────

use std::env;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use avin_utils::{AvinError, Cmd};

use self::config::Config;
use self::dirs::WorkspaceDirs;

const WORKSPACE_FILE: &str = "AVIN.toml";
const WORKSPACE_FILE_HIDDEN: &str = ".AVIN.toml";
const WORKSPACE_ENV: &str = "AVIN_WORKSPACE";

#[derive(Debug, Deserialize)]
pub struct Workspace {
    #[allow(dead_code)]
    format: u32,
    pub dirs: WorkspaceDirs,
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
