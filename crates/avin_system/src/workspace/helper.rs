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
    // TODO: обертка ошибки
    let toml_text = std::fs::read_to_string(path).unwrap();

    toml::from_str(&toml_text).map_err(|err| {
        SystemError::Parse(format!(
            "failed to parse {}: {err}",
            path.display()
        ))
    })
}
