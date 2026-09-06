// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::env;
use std::path::{Path, PathBuf};

use crate::SystemError;

use super::avin::AvinToml;
use super::config::Config;
use super::data::DataManifest;
use super::secret::Secret;

const WS_FILE: &str = "AVIN.toml";
const WS_FILE_HIDDEN: &str = ".AVIN.toml";
const WS_ENV_VAR: &str = "AVIN_WORKSPACE";

const CONFIG_FILE: &str = "config.toml";
const DATA_FILE: &str = "data.toml";
// const GUI_FILE: &str = "gui.toml";
const SECRET_FILE: &str = "secret.toml";

/// Open AVIN workspace.
///
/// `Workspace` represents the process runtime environment loaded from the
/// workspace configuration files. It provides access to application settings,
/// desired market data, secrets, and resolved workspace directories.
pub struct Workspace {
    avin: AvinToml,
    pub config: Config,
    pub data: DataManifest,
    pub secret: Secret,
}

impl Workspace {
    /// Opens the current AVIN workspace.
    ///
    /// The workspace is located in the current directory or through the
    /// AVIN_WORKSPACE environment variable. Its AVIN.toml, config.toml,
    /// data.toml, and secret.toml files are then loaded and validated.
    ///
    /// # Errors
    ///
    /// Returns an error if the workspace cannot be located or any required
    /// workspace file cannot be read, parsed, or validated.
    pub fn open() -> Result<Self, SystemError> {
        let ws_file = locate_workspace_file()?;

        let avin = AvinToml::read(&ws_file)?;
        let config = Config::read(&avin.cfg().join(CONFIG_FILE))?;
        let data = DataManifest::read(&avin.cfg().join(DATA_FILE))?;
        let secret = Secret::read(&avin.cfg().join(SECRET_FILE))?;

        Ok(Self {
            avin,
            config,
            data,
            secret,
        })
    }

    // AvinToml proxy methods:

    /// Returns the log directory.
    pub fn log(&self) -> &Path {
        self.avin.log()
    }

    /// Returns the market data directory.
    pub fn market_data(&self) -> &Path {
        self.avin.market_data()
    }

    /// Returns the instrument info cache directory.
    pub fn instruments(&self) -> &Path {
        self.avin.instruments()
    }

    /// Returns the pattern search results directory.
    pub fn search(&self) -> &Path {
        self.avin.search()
    }

    /// Returns the tester results directory.
    pub fn test(&self) -> &Path {
        self.avin.test()
    }

    /// Returns the watchlist directory.
    pub fn watchlist(&self) -> &Path {
        self.avin.watchlist()
    }
}

fn locate_workspace_file() -> Result<PathBuf, SystemError> {
    // locate in current dir
    let cur_dir = env::current_dir().map_err(|err| {
        let msg = "failed to get current working directory".to_string();
        SystemError::Workspace {
            message: msg,
            source: Some(Box::new(err)),
        }
    })?;

    if let Some(ws_file) = workspace_file_in(&cur_dir) {
        return Ok(ws_file);
    }

    // locate in env dir
    let Some(env_dir) = env::var_os(WS_ENV_VAR) else {
        let msg = format!(
            "AVIN workspace file not found: \
            neither '{WS_FILE}' nor '{WS_FILE_HIDDEN}' exists in {}, \
            and env var {WS_ENV_VAR} is not set",
            cur_dir.display()
        );
        return Err(SystemError::Workspace {
            message: msg,
            source: None,
        });
    };

    let env_dir = PathBuf::from(env_dir);
    if let Some(ws_file) = workspace_file_in(&env_dir) {
        return Ok(ws_file);
    }

    let msg = format!(
        "AVIN workspace file not found: \
        env var {WS_ENV_VAR}={}, \
        but neither '{WS_FILE}' nor '{WS_FILE_HIDDEN}' exists there",
        env_dir.display()
    );
    Err(SystemError::Workspace {
        message: msg,
        source: None,
    })
}

fn workspace_file_in(dir: &Path) -> Option<PathBuf> {
    let path = dir.join(WS_FILE);
    if path.is_file() {
        return Some(path);
    }

    let path = dir.join(WS_FILE_HIDDEN);
    if path.is_file() {
        return Some(path);
    }

    None
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn workspace_file_in_prefers_avin_toml() {
        let dir = tempdir().unwrap();

        let avin = dir.path().join(WS_FILE);
        let hidden = dir.path().join(WS_FILE_HIDDEN);

        fs::write(&avin, "").unwrap();
        fs::write(&hidden, "").unwrap();

        let path = workspace_file_in(dir.path()).unwrap();

        assert_eq!(path, avin);
    }

    #[test]
    fn workspace_file_in_finds_hidden_avin_toml() {
        let dir = tempdir().unwrap();

        let hidden = dir.path().join(WS_FILE_HIDDEN);
        fs::write(&hidden, "").unwrap();

        let path = workspace_file_in(dir.path()).unwrap();

        assert_eq!(path, hidden);
    }

    #[test]
    fn workspace_file_in_returns_none() {
        let dir = tempdir().unwrap();

        assert!(workspace_file_in(dir.path()).is_none());
    }
}
