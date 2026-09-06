// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod avin_logger;
mod log_file;

// ───────────────────────────────────────────────────────────────────────────

use crate::SystemError;
use crate::workspace::Workspace;

use self::avin_logger::AvinLogger;

pub(crate) fn init_logger(ws: &Workspace) -> Result<(), SystemError> {
    let log_dir = ws.dirs.log();
    let level = ws.config.log.level();
    let history = ws.config.log.history();

    let logger = AvinLogger::new(level, log_dir, history)?;

    log::set_boxed_logger(Box::new(logger)).map_err(|err| {
        let msg = "logger: initialization failed".to_string();
        SystemError::Logger {
            message: msg,
            source: Some(Box::new(err)),
        }
    })?;
    log::set_max_level(level);

    Ok(())
}
