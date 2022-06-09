use bevy::prelude::*;
use bevy::diagnostic::LogDiagnosticsPlugin;
use nora_core::plugins::CorePlugins;
use nora_tcp::TCPPlugin;
use nora_lib::test_scene;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CorePlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(TCPPlugin)
        .add_startup_system(test_scene)
        .run();
}