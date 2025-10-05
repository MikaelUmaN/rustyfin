use pyo3::prelude::*;
use crate::options::volatility::implied_volatility;

#[pyfunction]
pub fn implied_volatility_py(
    p: f64, s: f64, k: f64, t: f64, r: f64, is_call: bool
) -> PyResult<f64> {
    Ok(implied_volatility(p, s, k, t, r, is_call))
}