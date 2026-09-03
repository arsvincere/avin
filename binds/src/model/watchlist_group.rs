// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// use pyo3::prelude::*;
//
// use avin_domain::WatchlistGroup;
//
// use crate::model::PyInstrumentId;
//
// #[pyclass(module = "avin._native")]
// pub struct PyWatchlistGroup {
//     pub(crate) inner: WatchlistGroup,
// }
//
// #[pymethods]
// impl PyWatchlistGroup {
//     #[new]
//     fn new(name: &str) -> Self {
//         Self {
//             inner: WatchlistGroup::new(name),
//         }
//     }
//
//     fn name(&self) -> &str {
//         self.inner.name()
//     }
//
//     fn instruments(&self) -> Vec<PyInstrumentId> {
//         self.inner
//             .instruments()
//             .iter()
//             .cloned()
//             .map(|inner| PyInstrumentId { inner })
//             .collect()
//     }
//
//     fn is_empty(&self) -> bool {
//         self.inner.is_empty()
//     }
//
//     fn len(&self) -> usize {
//         self.inner.len()
//     }
//
//     fn get(&self, n: usize) -> Option<PyInstrumentId> {
//         self.inner
//             .get(n)
//             .cloned()
//             .map(|inner| PyInstrumentId { inner })
//     }
// }
