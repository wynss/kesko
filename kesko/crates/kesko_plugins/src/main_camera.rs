use bevy::prelude::*;
use kesko_object_interaction::InteractorBundle;

use kesko_core::{
    interaction::groups::GroupDynamic,
    orbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin},
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

    let camera_transform = Transform::from_translation(camera_pos).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn((
        Camera3dBundle {
            transform: camera_transform,
            ..Default::default()
        },
        PanOrbitCamera {
            dist_to_center: distance,
            ..Default::default()
        },
        InteractorBundle::<GroupDynamic>::default(),
    ));
}
