// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::str::FromStr;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use avin::Exchange;

#[pyclass(module = "avin._native")]
pub struct PyExchange {
    pub(crate) inner: Exchange,
}

#[pymethods]
impl PyExchange {
    #[classattr]
    #[allow(non_upper_case_globals)]
    const Binance: Self = Self {
        inner: Exchange::Binance,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const Bybit: Self = Self {
        inner: Exchange::Bybit,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const MOEX: Self = Self {
        inner: Exchange::MOEX,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const SPB: Self = Self {
        inner: Exchange::SPB,
    };

    #[staticmethod]
    fn all() -> Vec<Self> {
        Exchange::all()
            .iter()
            .copied()
            .map(|inner| Self { inner })
            .collect()
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        let inner = Exchange::from_str(s)
            .map_err(|err| PyValueError::new_err(err.to_string()))?;

        Ok(Self { inner })
    }

    fn display(&self) -> String {
        self.inner.to_string()
    }

    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn name(&self) -> &'static str {
        self.inner.name()
    }
}
