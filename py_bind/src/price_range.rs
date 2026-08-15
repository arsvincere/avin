use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use avin::{AvinError, PriceRange};

#[pyclass(name = "PriceRange")]
pub struct PyPriceRange {
    inner: PriceRange,
}

#[pymethods]
impl PyPriceRange {
    #[new]
    fn new(low: f64, high: f64) -> PyResult<Self> {
        let inner = PriceRange::new(low, high).map_err(|err| match err {
            AvinError::InvalidValue(message) => PyValueError::new_err(message),
        })?;

        Ok(Self { inner })
    }

    fn __contains__(&self, value: f64) -> bool {
        self.inner.contains(value)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __repr__(&self) -> String {
        format!("PriceRange({}, {})", self.inner.low(), self.inner.high())
    }

    #[getter]
    fn low(&self) -> f64 {
        self.inner.low()
    }

    #[getter]
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
