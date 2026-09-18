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
    level: LevelFilter,
    log_file: Mutex<LogFile>,
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
        let (color, reset) = match record.level() {
            log::Level::Error => ("\x1b[31m", "\x1b[0m"), // red
            log::Level::Warn => ("\x1b[33m", "\x1b[0m"),  // yellow
            log::Level::Info => ("\x1b[32m", "\x1b[0m"),  // green
            log::Level::Debug => ("\x1b[36m", "\x1b[0m"), // cyan
            log::Level::Trace => ("\x1b[90m", "\x1b[0m"), // gray
        };

        eprintln!(
            "{} {}[{}]{} {}",
            now.format("%H:%M:%S"),
            color,
            record.level(),
            reset,
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

        // log file write failure is non-fatal
        if let Err(err) = log_file.write(record, now) {
            eprintln!("{}", err.report());
        }
    }
}

impl Log for AvinLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let is_avin = metadata.target().starts_with("avin");

        is_avin && metadata.level().to_level_filter() <= self.level
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
