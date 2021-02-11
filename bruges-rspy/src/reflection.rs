use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use libbruges_rs::reflection;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn critical_angle(vel1: f64, vel2: f64) -> PyResult<f64> {
    Ok(reflection::critical_angle(vel1, vel2))
}

pub fn init_reflection(module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(critical_angle, module)?)?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn reflection(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(critical_angle, m)?)?;

    Ok(())
}
