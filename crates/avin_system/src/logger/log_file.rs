// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Local, NaiveDate};
use log::Record;

use crate::SystemError;

pub(super) struct LogFile {
    dir: PathBuf,
    date: NaiveDate,
    history: usize,
    pub(super) file: File,
}

impl LogFile {
    pub(super) fn new(
        dir: &Path,
        history: usize,
    ) -> Result<Self, SystemError> {
        create_dirs(dir)?;

        let date = Local::now().date_naive();
        let file = open_log_file(dir, date)?;

        // don't crash on fail cleanup
        if let Err(err) = cleanup_old_logs(dir, date, history) {
            eprintln!("{err}");
        }

        Ok(Self {
            dir: dir.to_path_buf(),
            date,
            history,
            file,
        })
    }

    pub(super) fn write(
        &mut self,
        record: &Record,
        now: &DateTime<Local>,
    ) -> Result<(), SystemError> {
        let date = now.date_naive();

        if date != self.date {
            self.file = open_log_file(&self.dir, date)?;
            self.date = date;

            // don't crash on fail cleanup
            if let Err(err) = cleanup_old_logs(&self.dir, date, self.history)
            {
                eprintln!("{err}");
            }
        }

        let result = writeln!(
            self.file,
            "{} [{}] {}: {}",
            now.format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            record.args()
        );

        result.map_err(|err| {
            let path = self.dir.join(format!("{}.log", self.date));
            let msg = format!(
                "logger: failed to write log record to {}",
                path.display()
            );
            SystemError::Logger {
                message: msg,
                source: Some(Box::new(err)),
            }
        })
    }
}

fn create_dirs(dir_path: &Path) -> Result<(), SystemError> {
    std::fs::create_dir_all(dir_path).map_err(|err| {
        let msg =
            format!("logger: failed create log dir {}", dir_path.display());
        SystemError::Logger {
            message: msg,
            source: Some(Box::new(err)),
        }
    })
}

fn open_log_file(dir: &Path, date: NaiveDate) -> Result<File, SystemError> {
    let path = dir.join(format!("{date}.log"));

    let result = OpenOptions::new().create(true).append(true).open(&path);

    result.map_err(|err| {
        let msg =
            format!("logger: failed to open log file {}", path.display());
        SystemError::Logger {
            message: msg,
            source: Some(Box::new(err)),
        }
    })
}

fn cleanup_old_logs(
    dir: &Path,
    today: NaiveDate,
    history: usize,
) -> Result<(), SystemError> {
    let files = get_files(dir)?;

    for path in files.iter() {
        let Some(name) = path.file_name().and_then(|name| name.to_str())
        else {
            continue;
        };

        let Some(date_str) = name.strip_suffix(".log") else {
            continue;
        };

        let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") else {
            continue;
        };

        if date >= today {
            continue;
        }

        let age = today.signed_duration_since(date).num_days();

        if age >= history as i64 {
            std::fs::remove_file(path).map_err(|err| {
                let msg = format!(
                    "logger: failed to delete old log file {}",
                    path.display()
                );
                SystemError::Logger {
                    message: msg,
                    source: Some(Box::new(err)),
                }
            })?;
        }
    }

    Ok(())
}

fn get_files(dir_path: &Path) -> Result<Vec<PathBuf>, SystemError> {
    let iter = std::fs::read_dir(dir_path).map_err(|err| {
        let msg = format!(
            "logger: failed to read directory {}",
            dir_path.display()
        );
        SystemError::Logger {
            message: msg,
            source: Some(Box::new(err)),
        }
    })?;

    let mut files = Vec::new();

    for entry in iter {
        let entry = entry.map_err(|err| {
            let msg = format!(
                "logger: failed to read entry in directory {}",
                dir_path.display()
            );
            SystemError::Logger {
                message: msg,
                source: Some(Box::new(err)),
            }
        })?;

        let file_type = entry.file_type().map_err(|err| {
            let msg = format!(
                "logger: failed to read file type for {}",
                entry.path().display()
            );
            SystemError::Logger {
                message: msg,
                source: Some(Box::new(err)),
            }
        })?;

        if file_type.is_file() {
            files.push(entry.path());
        }
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn cleanup_removes_expired_logs() {
        let dir = tempdir().unwrap();
        let today = NaiveDate::from_ymd_opt(2026, 8, 29).unwrap();

        let expired = dir.path().join("2026-08-24.log");
        let recent = dir.path().join("2026-08-25.log");
        let current = dir.path().join("2026-08-29.log");

        fs::write(&expired, "").unwrap();
        fs::write(&recent, "").unwrap();
        fs::write(&current, "").unwrap();

        cleanup_old_logs(dir.path(), today, 5).unwrap();

        assert!(!expired.exists());
        assert!(recent.exists());
        assert!(current.exists());
    }

    #[test]
    fn cleanup_zero_history_removes_previous_logs() {
        let dir = tempdir().unwrap();
        let today = NaiveDate::from_ymd_opt(2026, 8, 29).unwrap();

        let previous = dir.path().join("2026-08-28.log");
        let current = dir.path().join("2026-08-29.log");

        fs::write(&previous, "").unwrap();
        fs::write(&current, "").unwrap();

        cleanup_old_logs(dir.path(), today, 0).unwrap();

        assert!(!previous.exists());
        assert!(current.exists());
    }

    #[test]
    fn cleanup_ignores_other_files() {
        let dir = tempdir().unwrap();
        let today = NaiveDate::from_ymd_opt(2026, 8, 29).unwrap();

        let text = dir.path().join("notes.txt");
        let invalid_log = dir.path().join("debug.log");

        fs::write(&text, "").unwrap();
        fs::write(&invalid_log, "").unwrap();

        cleanup_old_logs(dir.path(), today, 5).unwrap();

        assert!(text.exists());
        assert!(invalid_log.exists());
    }
}
