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
    PyBar, PyBarDirection, PyExchange, PyInstrumentId, PyInstrumentKind,
    PyPriceRange, PySymbol, PyTimeFrame,
};

#[pymodule]
#[pyo3(name = "_native")]
fn avin_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyExchange>()?;
    m.add_class::<PyInstrumentKind>()?;
    m.add_class::<PySymbol>()?;
    m.add_class::<PyInstrumentId>()?;

    m.add_class::<PyPriceRange>()?;
    m.add_class::<PyBarDirection>()?;
    m.add_class::<PyBar>()?;
    m.add_class::<PyTimeFrame>()?;

    Ok(())
}
