// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use avin::PriceRange;

#[pyclass(module = "avin._native")]
pub struct PyPriceRange {
    pub(crate) inner: PriceRange,
}

#[pymethods]
impl PyPriceRange {
    #[new]
    fn new(low: f64, high: f64) -> PyResult<Self> {
        let inner = PriceRange::new(low, high)
            .map_err(|err| PyValueError::new_err(err.to_string()))?;

        Ok(Self { inner })
    }

    fn display(&self) -> String {
        self.inner.to_string()
    }

    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn low(&self) -> f64 {
        self.inner.low()
    }

    fn high(&self) -> f64 {
        self.inner.high()
    }

    fn contains(&self, value: f64) -> bool {
        self.inner.contains(value)
    }

    fn middle(&self) -> f64 {
        self.inner.middle()
    }

    fn width(&self) -> f64 {
        self.inner.width()
    }
}
