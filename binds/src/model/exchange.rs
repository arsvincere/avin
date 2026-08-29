// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::str::FromStr;

use pyo3::prelude::*;

use avin_domain::Exchange;

use crate::error::avin_error_to_py;

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
        inner: Exchange::Moex,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const SPB: Self = Self {
        inner: Exchange::Spb,
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
        let inner = Exchange::from_str(s).map_err(avin_error_to_py)?;

        Ok(Self { inner })
    }

    fn display(&self) -> String {
        self.inner.to_string()
    }

    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn key(&self) -> &'static str {
        self.inner.key()
    }
}
