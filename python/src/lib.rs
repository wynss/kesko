use pyo3::prelude::*;

use nora_lib::*;


#[pyfunction]
fn run_bevy() -> PyResult<()> {
    Ok(start())
}

#[pymodule]
fn pynora(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_bevy, m)?)?;
    Ok(())
}
