// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::path::Path;

use serde::Deserialize;

use avin_utils::AvinError;

const FORMAT: u32 = 1;

/// Workspace secrets.
///
/// Secrets are loaded from the workspace `secret.toml` file and contain
/// credentials and API keys required to access external services.
///
/// The type intentionally does not implement [`Debug`] to avoid accidental
/// exposure of secret values in logs or diagnostic output.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Secret {
    format: u32,

    tbank_token: String,

    moex_login: String,
    moex_password: String,
    moex_api_key: String,
}

impl Secret {
    pub(super) fn read(path: &Path) -> Result<Self, AvinError> {
        let secret: Self = avin_utils::read_toml(path)?;

        if secret.format != FORMAT {
            return Err(AvinError::Value(format!(
                "unsupported secret.toml format: {}, supported={FORMAT}",
                secret.format
            )));
        }

        Ok(secret)
    }

    /// Returns the TBank API token.
    pub fn tbank_token(&self) -> &str {
        &self.tbank_token
    }

    /// Returns the MOEX account login.
    pub fn moex_login(&self) -> &str {
        &self.moex_login
    }

    /// Returns the MOEX account password.
    pub fn moex_password(&self) -> &str {
        &self.moex_password
    }

    /// Returns the MOEX API key.
    pub fn moex_api_key(&self) -> &str {
        &self.moex_api_key
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    fn secret_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    #[test]
    fn reject_unsupported_format() {
        let file = secret_file(
            r#"
format = 2

tbank_token = ""
moex_login = ""
moex_password = ""
moex_api_key = ""
"#,
        );

        assert!(Secret::read(file.path()).is_err());
    }

    #[test]
    fn read_secret() {
        let file = secret_file(
            r#"
format = 1

tbank_token = "tbank-token"

moex_login = "login"
moex_password = "password"

moex_api_key = "api-key"
"#,
        );

        let secret = Secret::read(file.path()).unwrap();

        assert_eq!(secret.tbank_token(), "tbank-token");
        assert_eq!(secret.moex_login(), "login");
        assert_eq!(secret.moex_password(), "password");
        assert_eq!(secret.moex_api_key(), "api-key");
    }

    #[test]
    fn read_empty_secrets() {
        let file = secret_file(
            r#"
format = 1

tbank_token = ""
moex_login = ""
moex_password = ""
moex_api_key = ""
"#,
        );

        let secret = Secret::read(file.path()).unwrap();

        assert_eq!(secret.tbank_token(), "");
        assert_eq!(secret.moex_login(), "");
        assert_eq!(secret.moex_password(), "");
        assert_eq!(secret.moex_api_key(), "");
    }

    #[test]
    fn reject_unknown_field() {
        let file = secret_file(
            r#"
format = 1

tbank_token = ""
moex_login = ""
moex_password = ""
moex_api_key = ""

ebanina = "42"
"#,
        );

        assert!(Secret::read(file.path()).is_err());
    }

    #[test]
    fn reject_missing_format() {
        let file = secret_file(
            r#"
tbank_token = ""
moex_login = ""
moex_password = ""
moex_api_key = ""
"#,
        );

        assert!(Secret::read(file.path()).is_err());
    }

    #[test]
    fn reject_missing_required_field() {
        let file = secret_file(
            r#"
format = 1

tbank_token = ""
moex_login = ""
moex_password = ""
"#,
        );

        assert!(Secret::read(file.path()).is_err());
    }
}
