// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::{path::Path, str::FromStr};

use avin_core::Source;
use serde::Deserialize;

use avin_utils::AvinError;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[allow(dead_code)]
    format: u32,
    pub default: ConfigDefault,
    pub log: ConfigLog,
}

/// Workspace configuration.
///
/// The configuration is loaded from the workspace `config.toml` file and
/// contains default application settings and logging settings.
impl Config {
    pub(super) fn read(path: &Path) -> Result<Self, AvinError> {
        let config: Self = avin_utils::read_toml(path)?;

        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<(), AvinError> {
        Source::from_str(&self.default.source)?;

        Ok(())
    }
}

/// Default application settings.
///
/// These settings define the values used when an operation does not provide
/// an explicit override.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigDefault {
    source: String,
    watchlist: String,
    bars_count: usize,
    tick_days: usize,
}

impl ConfigDefault {
    /// Returns the default market data source.
    pub fn source(&self) -> Source {
        Source::from_str(&self.source).unwrap()
    }

    /// Returns the default watchlist name.
    pub fn watchlist(&self) -> &str {
        &self.watchlist
    }

    /// Returns the default number of bars to load.
    pub fn bars_count(&self) -> usize {
        self.bars_count
    }

    /// Returns the default number of days of tick data to load.
    pub fn tick_days(&self) -> usize {
        self.tick_days
    }
}

/// Logging settings.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigLog {
    history: usize,
    debug: bool,
    info: bool,
}

impl ConfigLog {
    /// Returns the number of days to keep log history.
    pub fn history(&self) -> usize {
        self.history
    }

    /// Returns whether debug logging is enabled.
    pub fn debug(&self) -> bool {
        self.debug
    }

    /// Returns whether info logging is enabled.
    pub fn info(&self) -> bool {
        self.info
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    fn config_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    #[test]
    fn read_config() {
        let file = config_file(
            r#"
format = 1

[default]
source = "tbank"
watchlist = "trio"
bars_count = 5000
tick_days = 7

[log]
history = 5
debug = false
info = true
"#,
        );

        let config = Config::read(file.path()).unwrap();

        assert_eq!(config.default.source(), Source::TBank);
        assert_eq!(config.default.watchlist(), "trio");
        assert_eq!(config.default.bars_count(), 5000);
        assert_eq!(config.default.tick_days(), 7);

        assert_eq!(config.log.history(), 5);
        assert!(!config.log.debug());
        assert!(config.log.info());
    }

    #[test]
    fn read_source_case_insensitive() {
        let file = config_file(
            r#"
format = 1

[default]
source = "TBANK"
watchlist = "trio"
bars_count = 5000
tick_days = 7

[log]
history = 5
debug = false
info = true
"#,
        );

        let config = Config::read(file.path()).unwrap();

        assert_eq!(config.default.source(), Source::TBank);
    }

    #[test]
    fn reject_invalid_source() {
        let file = config_file(
            r#"
format = 1

[default]
source = "ebanina"
watchlist = "trio"
bars_count = 5000
tick_days = 7

[log]
history = 5
debug = false
info = true
"#,
        );

        assert!(Config::read(file.path()).is_err());
    }

    #[test]
    fn reject_unknown_root_field() {
        let file = config_file(
            r#"
format = 1
ebanina = 42

[default]
source = "tbank"
watchlist = "trio"
bars_count = 5000
tick_days = 7

[log]
history = 5
debug = false
info = true
"#,
        );

        assert!(Config::read(file.path()).is_err());
    }

    #[test]
    fn reject_unknown_default_field() {
        let file = config_file(
            r#"
format = 1

[default]
EBANINA = 42
source = "tbank"
watchlist = "trio"
bars_count = 5000
tick_days = 7

[log]
history = 5
debug = false
info = true
"#,
        );

        assert!(Config::read(file.path()).is_err());
    }

    #[test]
    fn reject_unknown_log_field() {
        let file = config_file(
            r#"
format = 1

[default]
source = "tbank"
watchlist = "trio"
bars_count = 5000
tick_days = 7

[log]
history = 5
debug = false
info = true
ebanina = 42
"#,
        );

        assert!(Config::read(file.path()).is_err());
    }

    #[test]
    fn reject_missing_format() {
        let file = config_file(
            r#"
[default]
source = "tbank"
watchlist = "trio"
bars_count = 5000
tick_days = 7

[log]
history = 5
debug = false
info = true
"#,
        );

        assert!(Config::read(file.path()).is_err());
    }

    #[test]
    fn reject_missing_required_field() {
        let file = config_file(
            r#"
format = 1

[default]
source = "tbank"
watchlist = "trio"
bars_count = 5000

[log]
history = 5
debug = false
info = true
"#,
        );

        assert!(Config::read(file.path()).is_err());
    }
}
