// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::str::FromStr;

use pyo3::prelude::*;

use avin::InstrumentId;

use crate::error::avin_error_to_py;
use crate::model::{PyExchange, PyInstrumentKind, PySymbol};

#[pyclass(module = "avin._native")]
pub struct PyInstrumentId {
    pub(crate) inner: InstrumentId,
}

#[pymethods]
impl PyInstrumentId {
    #[new]
    fn new(
        exchange: PyRef<'_, PyExchange>,
        kind: PyRef<'_, PyInstrumentKind>,
        symbol: PyRef<'_, PySymbol>,
    ) -> Self {
        let inner = InstrumentId::new(
            exchange.inner,
            kind.inner,
            symbol.inner.clone(),
        );

        Self { inner }
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        let inner = InstrumentId::from_str(s).map_err(avin_error_to_py)?;

        Ok(Self { inner })
    }

    fn display(&self) -> String {
        self.inner.to_string()
    }

    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn exchange(&self) -> PyExchange {
        PyExchange {
            inner: self.inner.exchange(),
        }
    }

    fn kind(&self) -> PyInstrumentKind {
        PyInstrumentKind {
            inner: self.inner.kind(),
        }
    }

    fn symbol(&self) -> PySymbol {
        PySymbol {
            inner: self.inner.symbol().clone(),
        }
    }
}
