// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

use pyo3::prelude::*;

use avin_domain::{Bar, Chart};

use crate::error::avin_error_to_py;
use crate::model::{PyBar, PyInstrumentId, PyTicker, PyTimeFrame};

#[pyclass(module = "avin._native")]
pub struct PyChart {
    pub(crate) inner: Chart,
}

#[pymethods]
impl PyChart {
    #[new]
    fn new(
        iid: PyRef<'_, PyInstrumentId>,
        tf: PyRef<'_, PyTimeFrame>,
        bars: Vec<PyRef<'_, PyBar>>,
    ) -> Self {
        let bars = bars.into_iter().map(|bar| bar.inner).collect();
        let inner = Chart::new(iid.inner.clone(), tf.inner, bars);

        Self { inner }
    }

    #[staticmethod]
    fn empty(
        iid: PyRef<'_, PyInstrumentId>,
        tf: PyRef<'_, PyTimeFrame>,
    ) -> Self {
        let inner = Chart::empty(iid.inner.clone(), tf.inner);

        Self { inner }
    }

    fn iid(&self) -> PyInstrumentId {
        PyInstrumentId {
            inner: self.inner.iid().clone(),
        }
    }

    fn ticker(&self) -> PyTicker {
        PyTicker {
            inner: self.inner.ticker().clone(),
        }
    }

    fn tf(&self) -> PyTimeFrame {
        PyTimeFrame {
            inner: self.inner.tf(),
        }
    }

    // Avoid copying all bars just to get the chart length.
    fn len(&self) -> usize {
        self.inner.bars().len()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    // PERF: Сейчас каждый вызов копирует весь Rust Vec<Bar> в новый
    // Python list<PyBar>.
    // На больших графиках это дорого и может быть медленнее чистого Python API.
    // Возможные решения:
    // 1. вернуть ленивый view над Rust Vec<Bar> вместо list;
    // 2. материализовать только запрошенный slice;
    // 3. для bulk-доступа отдавать данные колонками, например в DataFrame.
    // Пока оставлено как простая временная реализация.
    fn bars(&self) -> Vec<PyBar> {
        py_bars(self.inner.bars())
    }

    fn bar(&self, index: isize) -> Option<PyBar> {
        self.inner.bar(index).copied().map(|inner| PyBar { inner })
    }

    fn first(&self) -> Option<PyBar> {
        self.inner.first().copied().map(|inner| PyBar { inner })
    }

    fn last(&self) -> Option<PyBar> {
        self.inner.last().copied().map(|inner| PyBar { inner })
    }

    fn last_price(&self) -> Option<f64> {
        self.inner.last_price()
    }

    fn select(&self, from_ts: i64, till_ts: i64) -> PyResult<Vec<PyBar>> {
        let bars = self
            .inner
            .select(from_ts, till_ts)
            .map_err(avin_error_to_py)?;

        Ok(py_bars(bars))
    }

    fn upsert(&mut self, bar: PyRef<'_, PyBar>) {
        self.inner.upsert(bar.inner);
    }
}

fn py_bars(bars: &[Bar]) -> Vec<PyBar> {
    bars.iter().copied().map(|inner| PyBar { inner }).collect()
}
