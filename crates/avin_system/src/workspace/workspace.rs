// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::env;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

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

static WORKSPACE: OnceLock<Workspace> = OnceLock::new();

/// Represents the AVIN process runtime environment.
///
/// Loads the workspace configuration from:
/// - `AVIN.toml`
/// - `config.toml`
/// - `data.toml`
/// - `secret.toml`
///
/// Provides access to application settings, market data requirements, secrets,
/// and resolved workspace directories.
#[derive(Debug)]
pub struct Workspace {
    pub dirs: AvinToml,
    pub config: Config,
    pub data: DataManifest,
    pub secret: Secret,
}

impl Workspace {
    /// Returns the global AVIN workspace, initializing it on first access.
    ///
    /// On first access, locates and opens the current workspace, loads and
    /// validates its configuration files, initializes the logger, and stores
    /// the resulting [`Workspace`] for subsequent calls.
    ///
    /// Later calls return the same workspace instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the workspace cannot be opened or the logger
    /// cannot be initialized.
    pub fn get() -> Result<&'static Self, SystemError> {
        if let Some(workspace) = WORKSPACE.get() {
            return Ok(workspace);
        }

        let workspace = Self::load()?;
        crate::logger::init_logger(&workspace)?;

        WORKSPACE
            .set(workspace)
            .expect("workspace must NOT be initialized");

        Ok(WORKSPACE.get().expect("workspace must be initialized"))
    }

    fn load() -> Result<Self, SystemError> {
        let ws_file = locate_workspace_file()?;

        let avin = AvinToml::read(&ws_file)?;
        let config = Config::read(&avin.cfg().join(CONFIG_FILE))?;
        let data = DataManifest::read(&avin.cfg().join(DATA_FILE))?;
        let secret = Secret::read(&avin.cfg().join(SECRET_FILE))?;

        Ok(Self {
            dirs: avin,
            config,
            data,
            secret,
        })
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
