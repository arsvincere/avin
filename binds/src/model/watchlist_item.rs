// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// use pyo3::prelude::*;
//
// use avin_domain::WatchlistItem;
//
// use crate::model::{PyInstrumentId, PyWatchlistGroup};
//
// #[pyclass(module = "avin._native")]
// pub struct PyWatchlistItem {
//     pub(crate) inner: WatchlistItem,
// }
//
// #[pymethods]
// impl PyWatchlistItem {
//     #[staticmethod]
//     fn from_instrument(iid: PyRef<'_, PyInstrumentId>) -> Self {
//         Self {
//             inner: WatchlistItem::Instrument(iid.inner.clone()),
//         }
//     }
//
//     #[staticmethod]
//     fn from_group(group: PyRef<'_, PyWatchlistGroup>) -> Self {
//         Self {
//             inner: WatchlistItem::Group(group.inner.clone()),
//         }
//     }
//
//     fn instrument(&self) -> Option<PyInstrumentId> {
//         match &self.inner {
//             WatchlistItem::Instrument(iid) => {
//                 Some(PyInstrumentId { inner: iid.clone() })
//             }
//             WatchlistItem::Group(_) => None,
//         }
//     }
//
//     fn group(&self) -> Option<PyWatchlistGroup> {
//         match &self.inner {
//             WatchlistItem::Instrument(_) => None,
//             WatchlistItem::Group(group) => Some(PyWatchlistGroup {
//                 inner: group.clone(),
//             }),
//         }
//     }
// }
