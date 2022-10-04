use bevy::prelude::*;
use bevy::diagnostic::LogDiagnosticsPlugin;
use kesko_plugins::CorePlugins;
use kesko_tcp::TCPPlugin;


fn main() {
    App::new()
        .add_plugins(CorePlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(TCPPlugin)
        .run();
}