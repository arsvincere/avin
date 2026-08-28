// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod avin_logger;
mod log_file;

// ───────────────────────────────────────────────────────────────────────────

use std::sync::Mutex;

use avin_utils::{AvinError, Cmd};

use crate::logger::avin_logger::AvinLogger;
use crate::logger::log_file::LogFile;
use crate::workspace::Workspace;

pub(crate) fn init_logger(ws: &Workspace) -> Result<(), AvinError> {
    let log_dir = ws.log();
    let level = ws.config.log.level();
    let history = ws.config.log.history();

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
