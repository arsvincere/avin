// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::str::FromStr;

use chrono::TimeDelta;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use avin::{AvinError, TimeFrame};

#[pyclass(module = "avin._native")]
pub struct PyTimeFrame {
    pub(crate) inner: TimeFrame,
}

#[pymethods]
impl PyTimeFrame {
    #[classattr]
    const S1: Self = Self {
        inner: TimeFrame::S1,
    };
    #[classattr]
    const S5: Self = Self {
        inner: TimeFrame::S5,
    };
    #[classattr]
    const S10: Self = Self {
        inner: TimeFrame::S10,
    };
    #[classattr]
    const S15: Self = Self {
        inner: TimeFrame::S15,
    };

    #[classattr]
    const M1: Self = Self {
        inner: TimeFrame::M1,
    };
    #[classattr]
    const M5: Self = Self {
        inner: TimeFrame::M5,
    };
    #[classattr]
    const M10: Self = Self {
        inner: TimeFrame::M10,
    };
    #[classattr]
    const M15: Self = Self {
        inner: TimeFrame::M15,
    };

    #[classattr]
    const H1: Self = Self {
        inner: TimeFrame::H1,
    };
    #[classattr]
    const H4: Self = Self {
        inner: TimeFrame::H4,
    };

    #[classattr]
    #[allow(non_upper_case_globals)]
    const Day: Self = Self {
        inner: TimeFrame::Day,
    };
    #[classattr]
    #[allow(non_upper_case_globals)]
    const Week: Self = Self {
        inner: TimeFrame::Week,
    };
    #[classattr]
    #[allow(non_upper_case_globals)]
    const Month: Self = Self {
        inner: TimeFrame::Month,
    };

    #[staticmethod]
    fn all() -> Vec<Self> {
        TimeFrame::all()
            .iter()
            .copied()
            .map(|inner| Self { inner })
            .collect()
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        let inner = TimeFrame::from_str(s).map_err(|err| match err {
            AvinError::InvalidValue(msg) => PyValueError::new_err(msg),
        })?;

        Ok(Self { inner })
    }

    fn display(&self) -> String {
        self.inner.to_string()
    }

    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn nanos(&self) -> Option<u64> {
        self.inner.nanos()
    }

    fn seconds(&self) -> Option<u32> {
        self.inner.seconds()
    }

    fn timedelta(&self) -> Option<TimeDelta> {
        self.inner.timedelta()
    }

    fn begin_frame_ts(&self, ts: i64) -> i64 {
        self.inner.begin_frame_ts(ts)
    }

    fn end_frame_ts(&self, ts: i64) -> i64 {
        self.inner.end_frame_ts(ts)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn variants_delegation() {
        let cases = [
            (PyTimeFrame::S1.inner, TimeFrame::S1),
            (PyTimeFrame::S5.inner, TimeFrame::S5),
            (PyTimeFrame::S10.inner, TimeFrame::S10),
            (PyTimeFrame::S15.inner, TimeFrame::S15),
            (PyTimeFrame::M1.inner, TimeFrame::M1),
            (PyTimeFrame::M5.inner, TimeFrame::M5),
            (PyTimeFrame::M10.inner, TimeFrame::M10),
            (PyTimeFrame::M15.inner, TimeFrame::M15),
            (PyTimeFrame::H1.inner, TimeFrame::H1),
            (PyTimeFrame::H4.inner, TimeFrame::H4),
            (PyTimeFrame::Day.inner, TimeFrame::Day),
            (PyTimeFrame::Week.inner, TimeFrame::Week),
            (PyTimeFrame::Month.inner, TimeFrame::Month),
        ];
        assert_eq!(cases.len(), TimeFrame::all().len());

        for (bind, domain) in cases {
            assert_eq!(bind, domain);
        }
    }

    #[test]
    fn all_delegation() {
        let bind_all = PyTimeFrame::all();
        let domain_all = TimeFrame::all();
        assert_eq!(bind_all.len(), domain_all.len());

        for (bind, domain) in bind_all.iter().zip(domain_all) {
            assert_eq!(bind.inner, *domain);
        }
    }

    #[test]
    fn from_str_delegation() {
        let bind = PyTimeFrame::from_str("15M").unwrap();
        let domain = TimeFrame::from_str("15M").unwrap();

        assert_eq!(bind.inner, domain);
    }

    #[test]
    fn display_delegation() {
        let bind = PyTimeFrame {
            inner: TimeFrame::M15,
        };

        assert_eq!(bind.display(), bind.inner.to_string());
    }

    #[test]
    fn eq_delegation() {
        let a = PyTimeFrame {
            inner: TimeFrame::M15,
        };
        let b = PyTimeFrame {
            inner: TimeFrame::M15,
        };
        let c = PyTimeFrame {
            inner: TimeFrame::H1,
        };

        assert_eq!(a.eq(&b), a.inner == b.inner);
        assert_eq!(a.eq(&c), a.inner == c.inner);
    }

    #[test]
    fn instance_methods_delegation() {
        let bind = PyTimeFrame {
            inner: TimeFrame::M15,
        };
        assert_eq!(bind.nanos(), bind.inner.nanos());
        assert_eq!(bind.seconds(), bind.inner.seconds());
        assert_eq!(bind.timedelta(), bind.inner.timedelta());

        let ts = 1_691_000_123_456_789_000;
        assert_eq!(bind.begin_frame_ts(ts), bind.inner.begin_frame_ts(ts));
        assert_eq!(bind.end_frame_ts(ts), bind.inner.end_frame_ts(ts));
    }
}
