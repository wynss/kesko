use pyo3::prelude::*;
use bevy::prelude::*;


#[pyfunction]
fn run_bevy() -> PyResult<()> {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();

    Ok(())
}

#[pymodule]
fn nora(_py: Python, m: &PyModule) -> PyResult<()> {

    m.add_function(wrap_pyfunction!(run_bevy, m)?)?;

    Ok(())
}
