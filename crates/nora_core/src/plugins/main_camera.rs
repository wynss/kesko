use bevy::prelude::*;
use nora_object_interaction::InteractorBundle;

use crate::orbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};


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

pub fn spawn_camera(mut commands: Commands) {

    let camera_pos = Vec3::new(-2.0, 2.5, 5.0);
    let distance = camera_pos.length();

    let camera_transform = Transform::from_translation(camera_pos)
        .looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: camera_transform,
        ..Default::default()
    }).insert(PanOrbitCamera {
        distance,
        ..Default::default()
    }).insert_bundle(InteractorBundle::default());
}
