// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::SystemError;

const FORMAT: u32 = 1;

/// AVIN workspace layout.
///
/// The layout is loaded from the workspace `AVIN.toml` file and defines
/// directories used for configuration, logs, market data, research results,
/// tests, and watchlists.
///
/// Relative paths are resolved against the workspace root. Absolute paths are
/// used as configured.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AvinToml {
    format: u32,
    dirs: AvinTomlDirs,
}

impl AvinToml {
    pub(super) fn read(path: &Path) -> Result<Self, SystemError> {
        let mut avin: Self = super::helper::read_toml(path)?;

        if avin.format != FORMAT {
            return Err(SystemError::Value(format!(
                "unsupported AVIN.toml format: {}, supported={FORMAT}",
                avin.format
            )));
        }

        let ws_dir = path.parent().unwrap();
        avin.dirs.resolve(ws_dir);

        Ok(avin)
    }

    pub(super) fn cfg(&self) -> &Path {
        &self.dirs.cfg
    }

    /// Returns the log directory.
    pub fn log(&self) -> &Path {
        &self.dirs.log
    }

    /// Returns the market data directory.
    pub fn market_data(&self) -> &Path {
        &self.dirs.market_data
    }

    /// Returns the instrument data directory.
    pub fn instruments(&self) -> &Path {
        &self.dirs.instruments
    }

    /// Returns the pattern search directory.
    pub fn search(&self) -> &Path {
        &self.dirs.search
    }

    /// Returns the test results directory.
    pub fn test(&self) -> &Path {
        &self.dirs.test
    }

    /// Returns the watchlist directory.
    pub fn watchlist(&self) -> &Path {
        &self.dirs.watchlist
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AvinTomlDirs {
    cfg: PathBuf,
    log: PathBuf,
    market_data: PathBuf,
    instruments: PathBuf,
    search: PathBuf,
    test: PathBuf,
    watchlist: PathBuf,
}

impl AvinTomlDirs {
    fn resolve(&mut self, root: &Path) {
        self.cfg = resolve(root, &self.cfg);
        self.log = resolve(root, &self.log);
        self.market_data = resolve(root, &self.market_data);
        self.instruments = resolve(root, &self.instruments);
        self.search = resolve(root, &self.search);
        self.test = resolve(root, &self.test);
        self.watchlist = resolve(root, &self.watchlist);
    }
}

fn resolve(dir: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        dir.join(path)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    fn avin_toml(content: &str) -> (tempfile::TempDir, PathBuf) {
        let dir = tempdir().unwrap();
        let path = dir.path().join("AVIN.toml");

        fs::write(&path, content).unwrap();

        (dir, path)
    }

    #[test]
    fn reject_unsupported_format() {
        let (_dir, path) = avin_toml(
            r#"
format = 2

[dirs]
cfg = "cfg"
log = ".log"
market_data = "data"
instruments = "data/instruments"
search = "search"
test = "test"
watchlist = "watchlist"
"#,
        );

        assert!(AvinToml::read(&path).is_err());
    }

    #[test]
    fn read_relative_paths() {
        let (dir, path) = avin_toml(
            r#"
format = 1

[dirs]
cfg = "cfg"
log = ".log"
market_data = "data"
instruments = "data/instruments"
search = "search"
test = "test"
watchlist = "watchlist"
"#,
        );

        let avin = AvinToml::read(&path).unwrap();

        assert_eq!(avin.cfg(), dir.path().join("cfg"));
        assert_eq!(avin.log(), dir.path().join(".log"));
        assert_eq!(avin.market_data(), dir.path().join("data"));
        assert_eq!(avin.instruments(), dir.path().join("data/instruments"));
        assert_eq!(avin.search(), dir.path().join("search"));
        assert_eq!(avin.test(), dir.path().join("test"));
        assert_eq!(avin.watchlist(), dir.path().join("watchlist"));
    }

    #[test]
    fn keep_absolute_paths() {
        let dir = tempdir().unwrap();

        let cfg = dir.path().join("external-cfg");
        let log = dir.path().join("external-log");
        let market_data = dir.path().join("external-data");
        let instruments = dir.path().join("external-instruments");
        let search = dir.path().join("external-search");
        let test = dir.path().join("external-test");
        let watchlist = dir.path().join("external-watchlist");

        let content = format!(
            r#"
format = 1

[dirs]
cfg = "{}"
log = "{}"
market_data = "{}"
instruments = "{}"
search = "{}"
test = "{}"
watchlist = "{}"
"#,
            cfg.display(),
            log.display(),
            market_data.display(),
            instruments.display(),
            search.display(),
            test.display(),
            watchlist.display(),
        );

        let path = dir.path().join("AVIN.toml");
        fs::write(&path, content).unwrap();

        let avin = AvinToml::read(&path).unwrap();

        assert_eq!(avin.cfg(), cfg);
        assert_eq!(avin.log(), log);
        assert_eq!(avin.market_data(), market_data);
        assert_eq!(avin.instruments(), instruments);
        assert_eq!(avin.search(), search);
        assert_eq!(avin.test(), test);
        assert_eq!(avin.watchlist(), watchlist);
    }

    #[test]
    fn reject_unknown_root_field() {
        let (_dir, path) = avin_toml(
            r#"
format = 1
ebanina = 42

[dirs]
cfg = "cfg"
log = ".log"
market_data = "data"
instruments = "data/instruments"
search = "search"
test = "test"
watchlist = "watchlist"
"#,
        );

        assert!(AvinToml::read(&path).is_err());
    }

    #[test]
    fn reject_unknown_dirs_field() {
        let (_dir, path) = avin_toml(
            r#"
format = 1

[dirs]
cfg = "cfg"
log = ".log"
market_data = "data"
instruments = "data/instruments"
search = "search"
test = "test"
watchlist = "watchlist"
foo = "bar"
"#,
        );

        assert!(AvinToml::read(&path).is_err());
    }

    #[test]
    fn reject_missing_format() {
        let (_dir, path) = avin_toml(
            r#"
[dirs]
cfg = "cfg"
log = ".log"
market_data = "data"
instruments = "data/instruments"
search = "search"
test = "test"
watchlist = "watchlist"
"#,
        );

        assert!(AvinToml::read(&path).is_err());
    }

    #[test]
    fn reject_missing_required_dir() {
        let (_dir, path) = avin_toml(
            r#"
format = 1

[dirs]
cfg = "cfg"
log = ".log"
market_data = "data"
instruments = "data/instruments"
search = "search"
test = "test"
"#,
        );

        assert!(AvinToml::read(&path).is_err());
    }
}
