// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use pyo3::prelude::*;

use avin_domain::InstrumentList;

use crate::error::avin_error_to_py;
use crate::model::{PyInstrumentId, PyInstrumentInfo};

#[pyclass(module = "avin._native")]
pub struct PyInstrumentList {
    pub(crate) inner: InstrumentList,
}

#[pymethods]
impl PyInstrumentList {
    #[new]
    fn new() -> Self {
        Self {
            inner: InstrumentList::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    // PERF: `iter()` clones all `InstrumentInfo` values into Python objects.
    // This is acceptable for now, but may become expensive for large lists.
    fn iter(&self) -> Vec<PyInstrumentInfo> {
        self.inner
            .iter()
            .cloned()
            .map(|inner| PyInstrumentInfo { inner })
            .collect()
    }

    fn add(
        &mut self,
        instrument: PyRef<'_, PyInstrumentInfo>,
    ) -> PyResult<()> {
        self.inner
            .add(instrument.inner.clone())
            .map_err(avin_error_to_py)
    }

    fn find(
        &self,
        iid: PyRef<'_, PyInstrumentId>,
    ) -> Option<PyInstrumentInfo> {
        self.inner
            .find(&iid.inner)
            .cloned()
            .map(|inner| PyInstrumentInfo { inner })
    }
}
