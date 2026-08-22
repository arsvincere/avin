// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::str::FromStr;

use pyo3::prelude::*;

use avin::InstrumentKind;

use crate::error::avin_error_to_py;

#[pyclass(module = "avin._native")]
pub struct PyInstrumentKind {
    pub(crate) inner: InstrumentKind,
}

#[pymethods]
impl PyInstrumentKind {
    #[classattr]
    #[allow(non_upper_case_globals)]
    const Currency: Self = Self {
        inner: InstrumentKind::Currency,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const Index: Self = Self {
        inner: InstrumentKind::Index,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const Stock: Self = Self {
        inner: InstrumentKind::Stock,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const Future: Self = Self {
        inner: InstrumentKind::Future,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const Bond: Self = Self {
        inner: InstrumentKind::Bond,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const Option: Self = Self {
        inner: InstrumentKind::Option,
    };

    #[classattr]
    const ETF: Self = Self {
        inner: InstrumentKind::ETF,
    };

    #[staticmethod]
    fn all() -> Vec<Self> {
        InstrumentKind::all()
            .iter()
            .copied()
            .map(|inner| Self { inner })
            .collect()
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        let inner = InstrumentKind::from_str(s).map_err(avin_error_to_py)?;

        Ok(Self { inner })
    }

    fn display(&self) -> String {
        self.inner.to_string()
    }

    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
