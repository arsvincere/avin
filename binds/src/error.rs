// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use pyo3::PyErr;
use pyo3::exceptions::{PyKeyError, PyOSError, PyValueError};

use avin_utils::AvinError;

pub(crate) fn avin_error_to_py(err: AvinError) -> PyErr {
    let message = err.report();

    match err {
        AvinError::Value(_) => PyValueError::new_err(message),
        AvinError::Parse(_) => PyValueError::new_err(message),
        AvinError::Key(_) => PyKeyError::new_err(message),
        AvinError::Missing(_) => PyValueError::new_err(message),

        AvinError::Io { .. } => PyOSError::new_err(message),
        AvinError::InstrumentInfo { .. } => PyValueError::new_err(message),
    }
}
