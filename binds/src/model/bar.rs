// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

// TODO: ts: i64 -> time: Time

// use chrono::{DateTime, Utc};
// use pyo3::prelude::*;
//
// use avin_domain::Bar;
//
// use crate::model::{PyBarDirection, PyPriceRange};
//
// #[pyclass(module = "avin._native")]
// pub struct PyBar {
//     pub(crate) inner: Bar,
// }
//
// #[pymethods]
// impl PyBar {
//     #[new]
//     fn new(ts: i64, o: f64, h: f64, l: f64, c: f64, v: u64) -> PyBar {
//         let inner = Bar::new(ts, o, h, l, c, v);
//
//         Self { inner }
//     }
//
//     fn ts(&self) -> i64 {
//         self.inner.ts
//     }
//
//     fn o(&self) -> f64 {
//         self.inner.o
//     }
//
//     fn h(&self) -> f64 {
//         self.inner.h
//     }
//
//     fn l(&self) -> f64 {
//         self.inner.l
//     }
//
//     fn c(&self) -> f64 {
//         self.inner.c
//     }
//
//     fn v(&self) -> u64 {
//         self.inner.v
//     }
//
//     fn display(&self) -> String {
//         self.inner.to_string()
//     }
//
//     fn eq(&self, other: &Self) -> bool {
//         self.inner == other.inner
//     }
//
//     fn dt(&self) -> DateTime<Utc> {
//         self.inner.dt()
//     }
//
//     fn direction(&self) -> PyBarDirection {
//         PyBarDirection {
//             inner: self.inner.direction(),
//         }
//     }
//
//     fn is_bear(&self) -> bool {
//         self.inner.is_bear()
//     }
//
//     fn is_bull(&self) -> bool {
//         self.inner.is_bull()
//     }
//
//     fn is_neutral(&self) -> bool {
//         self.inner.is_neutral()
//     }
//
//     fn range(&self) -> PyPriceRange {
//         PyPriceRange {
//             inner: self.inner.range(),
//         }
//     }
//
//     fn body(&self) -> PyPriceRange {
//         PyPriceRange {
//             inner: self.inner.body(),
//         }
//     }
//
//     fn lower(&self) -> PyPriceRange {
//         PyPriceRange {
//             inner: self.inner.lower(),
//         }
//     }
//
//     fn upper(&self) -> PyPriceRange {
//         PyPriceRange {
//             inner: self.inner.upper(),
//         }
//     }
//
//     fn contains(&self, price: f64) -> bool {
//         self.inner.contains(price)
//     }
// }
