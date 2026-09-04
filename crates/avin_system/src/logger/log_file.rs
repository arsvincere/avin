// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Local, NaiveDate};
use log::Record;

use crate::SystemError;

pub(super) struct LogFile {
    dir: PathBuf,
    date: NaiveDate,
    pub(super) file: File,
}

impl LogFile {
    pub(super) fn new(dir: &Path, history: usize) -> io::Result<Self> {
        let date = Local::now().date_naive();
        let file = open_log_file(dir, date)?;

        if let Err(err) = cleanup_old_logs(dir, date, history) {
            eprintln!("AVIN logger: failed to clean old logs: {err}");
        }

        Ok(Self {
            dir: dir.to_path_buf(),
            date,
            file,
        })
    }

    pub(super) fn write(
        &mut self,
        record: &Record,
        now: &DateTime<Local>,
    ) -> io::Result<()> {
        let date = now.date_naive();

        if date != self.date {
            self.file = open_log_file(&self.dir, date)?;
            self.date = date;
        }

        write_file_record(&mut self.file, record, now)
    }
}

pub fn get_files(dir_path: &Path) -> Result<Vec<PathBuf>, SystemError> {
    let iter = match std::fs::read_dir(dir_path) {
        Ok(iter) => iter,
        Err(err) => {
            return Err(SystemError::Io {
                message: format!(
                    "Failed to read dir: {}",
                    dir_path.display()
                ),
                source: err,
            });
        }
    };

    let mut files = Vec::new();

    for entry in iter {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                return Err(SystemError::Io {
                    message: format!(
                        "Failed to read entry in directory: {}",
                        dir_path.display()
                    ),
                    source: err,
                });
            }
        };

        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(err) => {
                return Err(SystemError::Io {
                    message: format!(
                        "Failed to read file type: {}",
                        entry.path().display()
                    ),
                    source: err,
                });
            }
        };

        if file_type.is_file() {
            files.push(entry.path());
        }
    }

    Ok(files)
}

fn open_log_file(dir: &Path, date: NaiveDate) -> io::Result<File> {
    let path = dir.join(format!("{date}.log"));

    OpenOptions::new().create(true).append(true).open(path)
}

fn write_file_record(
    file: &mut File,
    record: &Record,
    now: &DateTime<Local>,
) -> io::Result<()> {
    writeln!(
        file,
        "{} [{}] {}: {}",
        now.format("%Y-%m-%d %H:%M:%S"),
        record.level(),
        record.target(),
        record.args()
    )
}

fn cleanup_old_logs(
    dir: &Path,
    today: NaiveDate,
    history: usize,
) -> Result<(), SystemError> {
    let files = get_files(dir)?;

    for path in files.iter() {
        // TODO: обертка ошибки
        let name = path.file_name().unwrap().to_str().unwrap();

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

        if history == 0 || age >= history as i64 {
            // TODO: обертка ошибки
            std::fs::remove_file(path).unwrap();
        }
    }

    Ok(())
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
