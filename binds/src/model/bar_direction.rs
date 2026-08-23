// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use pyo3::prelude::*;

use avin_domain::BarDirection;

#[pyclass(module = "avin._native")]
pub struct PyBarDirection {
    pub(crate) inner: BarDirection,
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
    const Neutral: Self = Self {
        inner: BarDirection::Neutral,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const Bear: Self = Self {
        inner: BarDirection::Bear,
    };

    fn display(&self) -> String {
        self.inner.to_string()
    }

    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn value(&self) -> i8 {
        self.inner as i8
    }
}
