use pyo3::prelude::*;

use avin::BarDirection;

#[pyclass(name = "BarDirection", eq, from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyBarDirection {
    inner: BarDirection,
}

#[pymethods]
impl PyBarDirection {
    #[classattr]
    #[allow(non_upper_case_globals)]
    const Bull: Self = Self {
        inner: BarDirection::Bull,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const Doji: Self = Self {
        inner: BarDirection::Doji,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const Bear: Self = Self {
        inner: BarDirection::Bear,
    };

    fn __str__(&self) -> String {
        self.name()
    }

    #[getter]
    fn name(&self) -> String {
        self.inner.to_string()
    }

    #[getter]
    fn value(&self) -> i8 {
        self.inner as i8
    }
}
