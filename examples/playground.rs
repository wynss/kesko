use bevy::prelude::*;

use nora_core::orbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};
use nora_physics::{
    PhysicsPlugin,
    rigid_body::RigidBody,
    collider::{ColliderShape, ColliderPhysicalProperties}
};
use nora_raycast::{RayCastable, RayCastSource, RayCastPlugin};
fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::gravity())
        .add_plugin(RayCastPlugin)
        .add_plugin(PanOrbitCameraPlugin)
        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // Spawn ground
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: materials.add(Color::ALICE_BLUE.into()),
        ..Default::default()
    })
        .insert(RigidBody::Fixed)
        .insert(ColliderShape::Cuboid {x_half: 5.0, y_half: 0.0, z_half: 5.0})
        .insert(RayCastable);

    // Spawn sphere
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.5, subdivisions: 5})),
        material: materials.add(Color::LIME_GREEN.into()),
        transform: Transform::from_xyz(-2.0, 30.0, 0.0),
        ..Default::default()
    })
        .insert(RigidBody::Dynamic)
        .insert(ColliderShape::Sphere { radius: 0.5 })
        .insert(ColliderPhysicalProperties {
            restitution: 1.0,
            ..Default::default()
        })
        .insert(RayCastable);

    // camera
    let camera_pos = Vec3::new(-18.0, 8.0, 18.0);
    let distance = camera_pos.length();
    let camera_transform = Transform::from_translation(camera_pos)
        .looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: camera_transform,
        ..Default::default()
    }).insert(PanOrbitCamera {
        distance,
        ..Default::default()
    })
        .insert(RayCastSource::screen_space());

    // Light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 4000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}