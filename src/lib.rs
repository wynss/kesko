use pyo3::prelude::*;
use bevy::prelude::*;


fn hello_world() {
    println!("Hello from bevy!")
}

#[pyfunction]
fn run_bevy() -> PyResult<()> {
    App::build()
        .add_startup_system(hello_world.system())
        .run();

    Ok(())
}

#[pymodule]
fn nora(_py: Python, m: &PyModule) -> PyResult<()> {

    m.add_function(wrap_pyfunction!(run_bevy, m)?)?;

    Ok(())
}
