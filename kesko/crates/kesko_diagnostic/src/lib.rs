pub mod event;
pub mod fps;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

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
