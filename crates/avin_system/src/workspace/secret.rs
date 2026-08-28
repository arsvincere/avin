// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::path::Path;

use serde::Deserialize;

use avin_utils::AvinError;

#[derive(Deserialize)]
pub struct Secret {
    #[allow(dead_code)]
    format: u32,

    tbank_token: String,

    moex_login: String,
    moex_password: String,
    moex_api_key: String,
}

impl Secret {
    pub(super) fn read(path: &Path) -> Result<Self, AvinError> {
        avin_utils::read_toml(path)
    }

    pub fn tbank_token(&self) -> &str {
        &self.tbank_token
    }

    pub fn moex_login(&self) -> &str {
        &self.moex_login
    }

    pub fn moex_password(&self) -> &str {
        &self.moex_password
    }

    pub fn moex_api_key(&self) -> &str {
        &self.moex_api_key
    }
}
