use bevy::prelude::*;
use bevy::diagnostic::LogDiagnosticsPlugin;
use kesko_plugins::CorePlugins;
use kesko_tcp::TCPPlugin;


fn main() {
    App::new()
        .add_plugins(CorePlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(TCPPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(20.0, 40.0, -40.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}