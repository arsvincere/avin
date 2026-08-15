use pyo3::prelude::*;

use avin::BarKind;

#[repr(i8)]
#[pyclass(name = "BarKind", eq, from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PyBarKind {
    Bull = 1,
    Doji = 0,
    Bear = -1,
}
#[pymethods]
impl PyBarKind {
    #[getter]
    fn name(&self) -> &'static str {
        match self {
            Self::Bull => "Bull",
            Self::Doji => "Doji",
            Self::Bear => "Bear",
        }
    }

    #[getter]
    fn value(&self) -> i8 {
        *self as i8
    }
}

impl From<BarKind> for PyBarKind {
    fn from(value: BarKind) -> Self {
        match value {
            BarKind::Bull => Self::Bull,
            BarKind::Doji => Self::Doji,
            BarKind::Bear => Self::Bear,
        }
    }
}

impl From<PyBarKind> for BarKind {
    fn from(value: PyBarKind) -> Self {
        match value {
            PyBarKind::Bull => Self::Bull,
            PyBarKind::Doji => Self::Doji,
            PyBarKind::Bear => Self::Bear,
        }
    }
}
