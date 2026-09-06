// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod avin_logger;
mod log_file;

// ───────────────────────────────────────────────────────────────────────────

use std::path::Path;
use std::sync::Mutex;

use crate::SystemError;

use crate::logger::avin_logger::AvinLogger;
use crate::logger::log_file::LogFile;
use crate::workspace::Workspace;

pub(crate) fn init_logger(ws: &Workspace) -> Result<(), SystemError> {
    let log_dir = ws.log();
    let level = ws.config.log.level();
    let history = ws.config.log.history();

    // create log file
    create_dirs(log_dir)?;
    let log_file = LogFile::new(log_dir, history)?;

    // create logger
    let logger = AvinLogger {
        level,
        log_file: Mutex::new(log_file),
    };

    log::set_boxed_logger(Box::new(logger)).map_err(|err| {
        let msg = format!("failed to initialize logger: {err}");
        SystemError::Logger {
            message: msg,
            source: Some(Box::new(err)),
        }
    })?;
    log::set_max_level(level);

    Ok(())
}

fn create_dirs(dir_path: &Path) -> Result<(), SystemError> {
    std::fs::create_dir_all(dir_path).map_err(|err| {
        let msg =
            format!("logger: failed create log dir {}", dir_path.display());
        SystemError::Io {
            message: msg,
            source: err,
        }
    })
}
