use pyo3::prelude::*;

pub mod optimization;
pub mod options;

use options::volatility_py::implied_volatility_py;

#[pymodule]
fn rustyfin(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(implied_volatility_py, m)?)?;
    Ok(())
}