mod model;

use pyo3::prelude::*;

use model::{PyBarDirection, PyPriceRange};

#[pymodule]
#[pyo3(name = "_native")]
fn avin_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPriceRange>()?;
    m.add_class::<PyBarDirection>()?;

    Ok(())
}
