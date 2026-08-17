// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::str::FromStr;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use avin::{AvinError, Exchange};

#[pyclass(name = "Exchange", module = "avin._native")]
pub struct PyExchange {
    inner: Exchange,
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

    fn name(&self) -> &'static str {
        self.inner.name()
    }

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
        let inner = Exchange::from_str(s).map_err(|err| match err {
            AvinError::InvalidValue(msg) => PyValueError::new_err(msg),
        })?;

        Ok(Self { inner })
    }
}
