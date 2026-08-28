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

use avin_utils::{AvinError, Cmd};

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
