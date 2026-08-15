mod bar_kind;
mod price_range;

use pyo3::prelude::*;

use bar_kind::PyBarKind;
use price_range::PyPriceRange;

#[pymodule]
#[pyo3(name = "_native")]
fn avin_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPriceRange>()?;
    m.add_class::<PyBarKind>()?;

    Ok(())
}
