use bevy::app::{App, Plugin};

use crate::orbit_camera::{PanOrbitCameraPlugin, spawn_camera};


pub(crate) struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PanOrbitCameraPlugin)
            .add_startup_system(spawn_camera);
    }

    fn name(&self) -> &str {
        "Main Camera Plugin"
    }
}
