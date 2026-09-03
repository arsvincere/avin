// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// use pyo3::prelude::*;
//
// use avin_domain::Watchlist;
//
// use crate::model::PyWatchlistItem;
//
// #[pyclass(module = "avin._native")]
// pub struct PyWatchlist {
//     pub(crate) inner: Watchlist,
// }
//
// #[pymethods]
// impl PyWatchlist {
//     #[new]
//     fn new(name: &str) -> Self {
//         Self {
//             inner: Watchlist::new(name),
//         }
//     }
//
//     fn name(&self) -> &str {
//         self.inner.name()
//     }
//
//     fn items(&self) -> Vec<PyWatchlistItem> {
//         self.inner
//             .items()
//             .iter()
//             .cloned()
//             .map(|inner| PyWatchlistItem { inner })
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
//     fn get(&self, n: usize) -> Option<PyWatchlistItem> {
//         self.inner
//             .get(n)
//             .cloned()
//             .map(|inner| PyWatchlistItem { inner })
//     }
// }
