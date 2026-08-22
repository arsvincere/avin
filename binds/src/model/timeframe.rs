// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use std::str::FromStr;

use chrono::TimeDelta;
use pyo3::prelude::*;

use avin::TimeFrame;

use crate::error::avin_error_to_py;

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
        let inner = TimeFrame::from_str(s).map_err(avin_error_to_py)?;

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
