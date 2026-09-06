// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::path::Path;
use std::sync::Mutex;

use chrono::{DateTime, Local};
use log::{LevelFilter, Log, Metadata, Record};

use crate::SystemError;

use super::log_file::LogFile;

pub(super) struct AvinLogger {
    pub(super) level: LevelFilter,
    pub(super) log_file: Mutex<LogFile>,
}

impl AvinLogger {
    pub(super) fn new(
        level: LevelFilter,
        log_dir: &Path,
        history: usize,
    ) -> Result<Self, SystemError> {
        let log_file = LogFile::new(log_dir, history)?;

        Ok(Self {
            level,
            log_file: Mutex::new(log_file),
        })
    }

    fn write_console(&self, record: &Record, now: &DateTime<Local>) {
        eprintln!(
            "{} [{}] {}",
            now.format("%H:%M:%S"),
            record.level(),
            record.args()
        );
    }

    fn write_file(&self, record: &Record, now: &DateTime<Local>) {
        let mut log_file = match self.log_file.lock() {
            Ok(log_file) => log_file,
            Err(err) => {
                eprintln!("logger: log file lock poisoned: {err}");
                return;
            }
        };

        if let Err(err) = log_file.write(record, now) {
            eprintln!("{err}");
        }
    }
}

impl Log for AvinLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level().to_level_filter() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let now = Local::now();

        self.write_console(record, &now);
        self.write_file(record, &now);
    }

    fn flush(&self) {}
}
