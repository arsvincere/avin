// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::path::Path;

use serde::de::DeserializeOwned;

use avin_utils::{AvinError, Cmd};

pub fn read_toml<T: DeserializeOwned>(path: &Path) -> Result<T, AvinError> {
    let toml_text = Cmd::read(path)?;

    toml::from_str(&toml_text).map_err(|err| {
        AvinError::Parse(format!("failed to parse {}: {err}", path.display()))
    })
}
