// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::str::FromStr;

use pyo3::prelude::*;

use avin::Ticker;

use crate::error::avin_error_to_py;

#[pyclass(module = "avin._native")]
pub struct PyTicker {
    pub(crate) inner: Ticker,
}

#[pymethods]
impl PyTicker {
    #[new]
    fn new(value: &str) -> PyResult<Self> {
        let inner = Ticker::new(value).map_err(avin_error_to_py)?;

        Ok(Self { inner })
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        let inner = Ticker::from_str(s).map_err(avin_error_to_py)?;

        Ok(Self { inner })
    }

    fn display(&self) -> String {
        self.inner.to_string()
    }

    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
