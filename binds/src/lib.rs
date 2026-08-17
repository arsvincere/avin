// ───────────────────────────────────────────────────────────────────────────
// AVIN
// Understand the market before trading it.
//
// https://avin.info
// ───────────────────────────────────────────────────────────────────────────

mod model;

use pyo3::prelude::*;

use model::{PyBarDirection, PyExchange, PyPriceRange};

#[pymodule]
#[pyo3(name = "_native")]
fn avin_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPriceRange>()?;
    m.add_class::<PyBarDirection>()?;
    m.add_class::<PyExchange>()?;

    Ok(())
}
