use bevy::prelude::*;

pub fn world() -> String {
    String::from("world")
}

pub fn start() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: String::from("Nora"),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}
