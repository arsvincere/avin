// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::io::{self, Write};
use std::sync::Mutex;

use chrono::{DateTime, Local};
use log::{LevelFilter, Log, Metadata, Record};

use crate::logger::log_file::LogFile;

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

        let mut log_file = match self.log_file.lock() {
            Ok(log_file) => log_file,
            Err(err) => {
                eprintln!("AVIN logger: log file lock poisoned: {err}");
                return;
            }
        };

        if let Err(err) = log_file.write(record, &now) {
            eprintln!("AVIN logger: failed to write log file: {err}");
        }
    }

    fn flush(&self) {
        let _ = io::stderr().flush();

        if let Ok(mut log_file) = self.log_file.lock() {
            let _ = log_file.file.flush();
        }
    }
}

fn write_console(record: &Record, now: &DateTime<Local>) {
    eprintln!(
        "{} [{}] {}",
        now.format("%H:%M:%S"),
        record.level(),
        record.args()
    );
}
