use bevy::prelude::*;
use kesko_object_interaction::InteractorBundle;

use kesko_core::{
    orbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera},
    interaction::groups::GroupDynamic
};


pub struct MainCameraPlugin;

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

    let camera_pos = Vec3::new(-4.0, 4.0, 8.0);
    let distance = camera_pos.length();

    let camera_transform = Transform::from_translation(camera_pos)
        .looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn_bundle(Camera3dBundle {
        transform: camera_transform,
        ..Default::default()
    }).insert(PanOrbitCamera {
        dist_to_center: distance,
        ..Default::default()
    }).insert_bundle(InteractorBundle::<GroupDynamic>::default());
}
