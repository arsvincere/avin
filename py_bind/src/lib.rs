mod range;

use pyo3::prelude::*;

use range::PyRange;

#[pymodule]
#[pyo3(name = "_native")]
fn avin_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyRange>()?;

    Ok(())
}
