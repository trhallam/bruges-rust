use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

pub mod reflection;

/// bruges_rs Python module
#[pymodule]
fn bruges_rspy(py: Python, module: &PyModule) -> PyResult<()> {
    // refleciton module
    let reflection_submod = PyModule::new(py, "reflection")?;
    reflection::init_reflection(reflection_submod)?;
    module.add_submodule(reflection_submod)?;
    Ok(())
}
