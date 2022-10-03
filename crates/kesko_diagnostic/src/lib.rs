pub mod fps;
pub mod event;

use bevy::prelude::*;
use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};

use self::event::DebugEventPlugin;


pub struct DiagnosticsPlugins;
impl PluginGroup for DiagnosticsPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(FrameTimeDiagnosticsPlugin);
        group.add(LogDiagnosticsPlugin::default());
        group.add(fps::FPSPlugin);
        group.add(DebugEventPlugin);
    }
}
