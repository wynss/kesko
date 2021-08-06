use pyo3::prelude::*;

use nora_lib::*;


#[pyfunction]
fn run_bevy() -> PyResult<()> {
    Ok(start())
}

#[pyfunction]
fn print_world() -> PyResult<String> {
    Ok(world())
}

#[pymodule]
fn pynora(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_bevy, m)?)?;
    m.add_function(wrap_pyfunction!(print_world, m)?)?;
    Ok(())
}
