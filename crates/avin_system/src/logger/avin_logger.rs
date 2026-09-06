// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::sync::Mutex;

use chrono::{DateTime, Local};
use log::{LevelFilter, Log, Metadata, Record};

use super::log_file::LogFile;

pub(super) struct AvinLogger {
    pub(super) level: LevelFilter,
    pub(super) log_file: Mutex<LogFile>,
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

        write_console(record, &now);
        write_file(&self.log_file, record, &now);
    }

    fn flush(&self) {}
}

fn write_console(record: &Record, now: &DateTime<Local>) {
    eprintln!(
        "{} [{}] {}",
        now.format("%H:%M:%S"),
        record.level(),
        record.args()
    );
}

fn write_file(
    log_file: &Mutex<LogFile>,
    record: &Record,
    now: &DateTime<Local>,
) {
    let mut log_file = match log_file.lock() {
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
