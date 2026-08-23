// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::collections::HashMap;

use pyo3::prelude::*;

use avin_model::InstrumentInfo;

use crate::model::{PyExchange, PyInstrumentId, PyInstrumentKind, PySymbol};

#[pyclass(module = "avin._native")]
pub struct PyInstrumentInfo {
    pub(crate) inner: InstrumentInfo,
}

#[pymethods]
impl PyInstrumentInfo {
    fn iid(&self) -> PyInstrumentId {
        PyInstrumentId {
            inner: self.inner.iid(),
        }
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
            inner: self.inner.symbol(),
        }
    }

    fn figi(&self) -> &str {
        self.inner.figi()
    }

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn lot(&self) -> u32 {
        self.inner.lot()
    }

    fn step(&self) -> f64 {
        self.inner.step()
    }

    fn raw_info(&self) -> &HashMap<String, String> {
        self.inner.raw_info()
    }
}
