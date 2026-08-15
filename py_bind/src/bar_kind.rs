use pyo3::prelude::*;

use avin::BarKind;

#[pyclass(name = "BarKind", eq, from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyBarKind {
    inner: BarKind,
}

#[pymethods]
impl PyBarKind {
    #[classattr]
    #[allow(non_upper_case_globals)]
    const Bull: Self = Self {
        inner: BarKind::Bull,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const Doji: Self = Self {
        inner: BarKind::Doji,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const Bear: Self = Self {
        inner: BarKind::Bear,
    };

    #[getter]
    fn name(&self) -> String {
        self.inner.to_string()
    }

    #[getter]
    fn value(&self) -> i8 {
        self.inner as i8
    }
}

impl From<BarKind> for PyBarKind {
    fn from(inner: BarKind) -> Self {
        Self { inner }
    }
}

impl From<PyBarKind> for BarKind {
    fn from(value: PyBarKind) -> Self {
        value.inner
    }
}
