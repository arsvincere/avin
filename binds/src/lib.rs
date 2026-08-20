// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod model;

use pyo3::prelude::*;

use model::{PyBar, PyBarDirection, PyExchange, PyPriceRange, PyTimeFrame};

#[pymodule]
#[pyo3(name = "_native")]
fn avin_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBarDirection>()?;
    m.add_class::<PyExchange>()?;
    m.add_class::<PyPriceRange>()?;
    m.add_class::<PyTimeFrame>()?;
    m.add_class::<PyBar>()?;

    Ok(())
}
