use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use kesko_ui::fps_component::FPSComponentEvent;

#[derive(Default)]
pub struct FPSPlugin;
impl Plugin for FPSPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::propagate_fps_system);
    }
}

impl FPSPlugin {
    fn propagate_fps_system(
        diagnostic: Res<Diagnostics>,
        mut fps_ew: EventWriter<FPSComponentEvent>,
    ) {
        if let Some(fps) = diagnostic.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(ave) = fps.average() {
                let history = fps
                    .values()
                    .cloned()
                    .map(|x| x as f32)
                    .collect::<Vec<f32>>();

                fps_ew.send(FPSComponentEvent::FPSData {
                    ave: ave as f32,
                    history,
                });
            }
        }
    }
}
