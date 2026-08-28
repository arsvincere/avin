// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use chrono::{DateTime, Local, NaiveDate};
use log::{LevelFilter, Log, Metadata, Record};

use avin_utils::{AvinError, Cmd};

use crate::Workspace;

pub(crate) fn init_logger(workspace: &Workspace) -> Result<(), AvinError> {
    let log_dir = workspace.log();
    let level = workspace.config.log.level();
    let history = workspace.config.log.history();

    Cmd::make_dirs(log_dir)?;

    let log_file =
        LogFile::new(log_dir, history).map_err(|err| AvinError::Io {
            message: "failed to open log file".to_string(),
            source: err,
        })?;

    let logger = AvinLogger {
        level,
        log_file: Mutex::new(log_file),
    };

    log::set_boxed_logger(Box::new(logger)).map_err(|err| {
        AvinError::Process(format!("failed to initialize logger: {err}"))
    })?;

    log::set_max_level(level);

    Ok(())
}

// Logger --------------------------------------------------------------------

struct AvinLogger {
    level: LevelFilter,
    log_file: Mutex<LogFile>,
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

// Log file ------------------------------------------------------------------

struct LogFile {
    dir: PathBuf,
    date: NaiveDate,
    file: File,
}

impl LogFile {
    fn new(dir: &Path, history: usize) -> io::Result<Self> {
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

    fn write(
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

// Output --------------------------------------------------------------------

fn write_console(record: &Record, now: &DateTime<Local>) {
    eprintln!(
        "{} [{}] {}",
        now.format("%H:%M:%S"),
        record.level(),
        record.args()
    );
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

fn open_log_file(dir: &Path, date: NaiveDate) -> io::Result<File> {
    let path = dir.join(format!("{date}.log"));

    OpenOptions::new().create(true).append(true).open(path)
}

// History -------------------------------------------------------------------

fn cleanup_old_logs(
    dir: &Path,
    today: NaiveDate,
    history: usize,
) -> Result<(), AvinError> {
    let files = Cmd::get_files(dir)?;

    for path in files.iter() {
        let name = Cmd::name(path)?;

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
            Cmd::delete(path)?;
        }
    }

    Ok(())
}
