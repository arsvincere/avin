// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::{path::Path, str::FromStr};

use log::LevelFilter;
use serde::Deserialize;

use avin_core::DataProvider;
use avin_utils::AvinError;

const FORMAT: u32 = 1;

/// Workspace configuration.
///
/// The configuration is loaded from the workspace `config.toml` file and
/// contains default application settings and logging settings.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    format: u32,
    pub default: ConfigDefault,
    pub log: ConfigLog,
}

impl Config {
    pub(super) fn read(path: &Path) -> Result<Self, AvinError> {
        let config: Self = super::helper::read_toml(path)?;

        if config.format != FORMAT {
            return Err(AvinError::Value(format!(
                "unsupported config.toml format: {}, supported={FORMAT}",
                config.format
            )));
        }

        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<(), AvinError> {
        // TODO: здесь надо завернуть в SytemError
        DataProvider::from_str(&self.default.data_provider).map_err(
            |_| AvinError::Value("error parse data provider".to_string()),
        )?;

        LevelFilter::from_str(&self.log.level).map_err(|_| {
            AvinError::Value(format!("unknown log level: {}", self.log.level))
        })?;

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
    data_provider: String,
    watchlist: String,
    bars_count: usize,
    tick_days: usize,
}

impl ConfigDefault {
    /// Returns the default market data provider.
    pub fn data_provider(&self) -> DataProvider {
        DataProvider::from_str(&self.data_provider).unwrap()
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
    level: String,
    history: usize,
}

impl ConfigLog {
    /// Returns the configured logging level.
    pub fn level(&self) -> LevelFilter {
        LevelFilter::from_str(&self.level).unwrap()
    }

    /// Returns the number of days to keep log history.
    pub fn history(&self) -> usize {
        self.history
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
    fn reject_unsupported_format() {
        let file = config_file(
            r#"
format = 2

[default]
data_provider = "tbank"
watchlist = "trio"
bars_count = 5000
tick_days = 7

[log]
level = "info"
history = 5
"#,
        );

        assert!(Config::read(file.path()).is_err());
    }

    #[test]
    fn read_config() {
        let file = config_file(
            r#"
format = 1

[default]
data_provider = "tbank"
watchlist = "trio"
bars_count = 5000
tick_days = 7

[log]
level = "info"
history = 5
"#,
        );

        let config = Config::read(file.path()).unwrap();

        assert_eq!(config.default.data_provider(), DataProvider::TBank);
        assert_eq!(config.default.watchlist(), "trio");
        assert_eq!(config.default.bars_count(), 5000);
        assert_eq!(config.default.tick_days(), 7);

        assert_eq!(config.log.level(), LevelFilter::Info);
        assert_eq!(config.log.history(), 5);
    }

    #[test]
    fn read_data_provider_case_insensitive() {
        let file = config_file(
            r#"
format = 1

[default]
data_provider = "TBANK"
watchlist = "trio"
bars_count = 5000
tick_days = 7

[log]
level = "info"
history = 5
"#,
        );

        let config = Config::read(file.path()).unwrap();

        assert_eq!(config.default.data_provider(), DataProvider::TBank);
    }

    #[test]
    fn reject_invalid_data_provider() {
        let file = config_file(
            r#"
format = 1

[default]
data_provider = "ebanina"
watchlist = "trio"
bars_count = 5000
tick_days = 7

[log]
level = "info"
history = 5
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
data_provider = "tbank"
watchlist = "trio"
bars_count = 5000
tick_days = 7

[log]
level = "info"
history = 5
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
data_provider = "tbank"
watchlist = "trio"
bars_count = 5000
tick_days = 7

[log]
level = "info"
history = 5
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
data_provider = "tbank"
watchlist = "trio"
bars_count = 5000
tick_days = 7

[log]
level = "info"
history = 5
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
data_provider = "tbank"
watchlist = "trio"
bars_count = 5000
tick_days = 7

[log]
level = "info"
history = 5
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
data_provider = "tbank"
watchlist = "trio"
bars_count = 5000

[log]
level = "info"
history = 5
"#,
        );

        assert!(Config::read(file.path()).is_err());
    }
}
