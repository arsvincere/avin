// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use chrono::{DateTime, Utc};
use pyo3::prelude::*;

use avin::Bar;

use crate::model::{PyBarDirection, PyPriceRange};

#[pyclass(module = "avin._native")]
pub struct PyBar {
    pub(crate) inner: Bar,
}

#[pymethods]
impl PyBar {
    #[new]
    fn new(ts: i64, o: f64, h: f64, l: f64, c: f64, v: u64) -> PyBar {
        let inner = Bar::new(ts, o, h, l, c, v);

        Self { inner }
    }

    fn ts(&self) -> i64 {
        self.inner.ts
    }

    fn o(&self) -> f64 {
        self.inner.o
    }

    fn h(&self) -> f64 {
        self.inner.h
    }

    fn l(&self) -> f64 {
        self.inner.l
    }

    fn c(&self) -> f64 {
        self.inner.c
    }

    fn v(&self) -> u64 {
        self.inner.v
    }

    fn display(&self) -> String {
        self.inner.to_string()
    }

    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn dt(&self) -> DateTime<Utc> {
        self.inner.dt()
    }

    fn direction(&self) -> PyBarDirection {
        PyBarDirection {
            inner: self.inner.direction(),
        }
    }

    fn is_bear(&self) -> bool {
        self.inner.is_bear()
    }

    fn is_bull(&self) -> bool {
        self.inner.is_bull()
    }

    fn is_neutral(&self) -> bool {
        self.inner.is_neutral()
    }

    fn range(&self) -> PyPriceRange {
        PyPriceRange {
            inner: self.inner.range(),
        }
    }

    fn body(&self) -> PyPriceRange {
        PyPriceRange {
            inner: self.inner.body(),
        }
    }

    fn lower(&self) -> PyPriceRange {
        PyPriceRange {
            inner: self.inner.lower(),
        }
    }

    fn upper(&self) -> PyPriceRange {
        PyPriceRange {
            inner: self.inner.upper(),
        }
    }

    fn contains(&self, price: f64) -> bool {
        self.inner.contains(price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_delegation() {
        let bind = PyBar::new(123_456_789, 10.0, 11.0, 9.9, 10.5, 5000);
        let domain = Bar::new(123_456_789, 10.0, 11.0, 9.9, 10.5, 5000);

        assert_eq!(bind.inner, domain);
    }

    #[test]
    fn fields_delegation() {
        let bind = PyBar::new(123_456_789, 10.0, 11.0, 9.9, 10.5, 5000);

        assert_eq!(bind.ts(), bind.inner.ts);
        assert_eq!(bind.o(), bind.inner.o);
        assert_eq!(bind.h(), bind.inner.h);
        assert_eq!(bind.l(), bind.inner.l);
        assert_eq!(bind.c(), bind.inner.c);
        assert_eq!(bind.v(), bind.inner.v);
    }

    #[test]
    fn display_delegation() {
        let bind = PyBar::new(123_456_789, 10.0, 11.0, 9.9, 10.5, 5000);

        assert_eq!(bind.display(), bind.inner.to_string());
    }

    #[test]
    fn eq_delegation() {
        let a = PyBar::new(123_456_789, 10.0, 11.0, 9.9, 10.5, 5000);
        let b = PyBar::new(123_456_789, 10.0, 11.0, 9.9, 10.5, 5000);
        let c = PyBar::new(987_654_321, 10.0, 11.0, 9.9, 10.5, 50);

        assert_eq!(a.eq(&b), a.inner == b.inner);
        assert_eq!(a.eq(&c), a.inner == c.inner);
    }

    #[test]
    fn instance_methods_delegation() {
        let bind = PyBar::new(123_456_789, 10.0, 11.0, 9.9, 10.5, 5000);

        assert_eq!(bind.dt(), bind.inner.dt());
        assert_eq!(bind.is_bear(), bind.inner.is_bear());
        assert_eq!(bind.is_bull(), bind.inner.is_bull());
        assert_eq!(bind.is_neutral(), bind.inner.is_neutral());

        let price = 10.3;
        assert_eq!(bind.contains(price), bind.inner.contains(price));

        assert_eq!(bind.direction().inner, bind.inner.direction());

        assert_eq!(bind.range().inner, bind.inner.range());
        assert_eq!(bind.body().inner, bind.inner.body());
        assert_eq!(bind.lower().inner, bind.inner.lower());
        assert_eq!(bind.upper().inner, bind.inner.upper());
    }
}
