// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::str::FromStr;

use pyo3::prelude::*;

use avin::Category;

use crate::error::avin_error_to_py;

#[pyclass(module = "avin._native")]
pub struct PyCategory {
    pub(crate) inner: Category,
}

#[pymethods]
impl PyCategory {
    #[classattr]
    #[allow(non_upper_case_globals)]
    const Currency: Self = Self {
        inner: Category::Currency,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const Index: Self = Self {
        inner: Category::Index,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const Share: Self = Self {
        inner: Category::Share,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const Future: Self = Self {
        inner: Category::Future,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const Bond: Self = Self {
        inner: Category::Bond,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const Option: Self = Self {
        inner: Category::Option,
    };

    #[classattr]
    const ETF: Self = Self {
        inner: Category::ETF,
    };

    #[staticmethod]
    fn all() -> Vec<Self> {
        Category::all()
            .iter()
            .copied()
            .map(|inner| Self { inner })
            .collect()
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        let inner = Category::from_str(s).map_err(avin_error_to_py)?;

        Ok(Self { inner })
    }

    fn display(&self) -> String {
        self.inner.to_string()
    }

    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
