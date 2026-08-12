use avin_core::Range;
use pyo3::prelude::*;

#[pyclass(name = "Range")]
pub struct PyRange {
    inner: Range,
}

#[pymethods]
impl PyRange {
    #[new]
    fn new(from: f64, till: f64) -> Self {
        Self {
            inner: Range::new(from, till),
        }
    }

    #[getter]
    fn from(&self) -> f64 {
        self.inner.from
    }

    #[getter]
    fn till(&self) -> f64 {
        self.inner.till
    }

    fn min(&self) -> f64 {
        self.inner.min()
    }

    fn max(&self) -> f64 {
        self.inner.max()
    }

    fn mid(&self) -> f64 {
        self.inner.mid()
    }

    fn contains(&self, value: f64) -> bool {
        self.inner.contains(value)
    }

    fn abs(&self) -> f64 {
        self.inner.abs()
    }

    fn abs_n(&self) -> f64 {
        self.inner.abs_n()
    }

    fn abs_p(&self) -> f64 {
        self.inner.abs_p()
    }

    fn delta(&self) -> f64 {
        self.inner.delta()
    }

    fn delta_n(&self) -> f64 {
        self.inner.delta_n()
    }

    fn delta_p(&self) -> f64 {
        self.inner.delta_p()
    }

    fn is_increase(&self) -> bool {
        self.inner.is_increase()
    }

    fn is_decrease(&self) -> bool {
        self.inner.is_decrease()
    }

    fn __contains__(&self, value: f64) -> bool {
        self.inner.contains(value)
    }

    fn __repr__(&self) -> String {
        format!("Range({}, {})", self.inner.from, self.inner.till)
    }
}
