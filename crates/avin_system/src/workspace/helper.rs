// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::path::Path;

use serde::de::DeserializeOwned;

use crate::SystemError;

pub fn read_toml<T: DeserializeOwned>(path: &Path) -> Result<T, SystemError> {
    let toml_text =
        std::fs::read_to_string(path).map_err(|err| SystemError::Io {
            message: format!("failed to read TOML file '{}'", path.display()),
            source: err,
        })?;

    toml::from_str(&toml_text).map_err(|err| SystemError::ParseToml {
        message: format!("failed to parse TOML file '{}'", path.display()),
        source: err,
    })
}
