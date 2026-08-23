// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod error;
mod model;

use pyo3::prelude::*;

use model::{
    PyBar, PyBarDirection, PyCategory, PyChart, PyExchange, PyInstrumentId,
    PyInstrumentInfo, PyPriceRange, PyTicker, PyTimeFrame,
};

#[pymodule]
#[pyo3(name = "_native")]
fn avin_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyExchange>()?;
    m.add_class::<PyCategory>()?;
    m.add_class::<PyTicker>()?;
    m.add_class::<PyInstrumentId>()?;
    m.add_class::<PyInstrumentInfo>()?;

    m.add_class::<PyPriceRange>()?;
    m.add_class::<PyBarDirection>()?;
    m.add_class::<PyBar>()?;
    m.add_class::<PyTimeFrame>()?;
    m.add_class::<PyChart>()?;

    Ok(())
}
