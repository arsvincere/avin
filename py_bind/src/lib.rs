mod price_range;

use pyo3::prelude::*;

use price_range::PyPriceRange;

#[pymodule]
#[pyo3(name = "_native")]
fn avin_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPriceRange>()?;

    Ok(())
}
