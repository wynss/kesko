pub mod event;
pub mod fps;

use bevy::app::PluginGroupBuilder;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

use self::event::DebugEventPlugin;

pub struct DiagnosticsPlugins;
impl PluginGroup for DiagnosticsPlugins {
    fn build() -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(FrameTimeDiagnosticsPlugin)
            .add(LogDiagnosticsPlugin::default())
            .add(fps::FPSPlugin)
            .add(DebugEventPlugin)
    }
}
