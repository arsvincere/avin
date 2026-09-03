// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// use std::collections::HashMap;
//
// use pyo3::prelude::*;
//
// use avin_domain::InstrumentInfo;
//
// use crate::model::{PyCategory, PyExchange, PyInstrumentId, PyTicker};
//
// #[pyclass(module = "avin._native")]
// pub struct PyInstrumentInfo {
//     pub(crate) inner: InstrumentInfo,
// }
//
// #[pymethods]
// impl PyInstrumentInfo {
//     fn iid(&self) -> PyInstrumentId {
//         PyInstrumentId {
//             inner: self.inner.iid(),
//         }
//     }
//
//     fn exchange(&self) -> PyExchange {
//         PyExchange {
//             inner: self.inner.exchange(),
//         }
//     }
//
//     fn category(&self) -> PyCategory {
//         PyCategory {
//             inner: self.inner.category(),
//         }
//     }
//
//     fn ticker(&self) -> PyTicker {
//         PyTicker {
//             inner: self.inner.ticker(),
//         }
//     }
//
//     fn figi(&self) -> &str {
//         self.inner.figi()
//     }
//
//     fn name(&self) -> &str {
//         self.inner.name()
//     }
//
//     fn lot(&self) -> u32 {
//         self.inner.lot()
//     }
//
//     fn step(&self) -> f64 {
//         self.inner.step()
//     }
//
//     fn raw_info(&self) -> &HashMap<String, String> {
//         self.inner.raw_info()
//     }
// }
