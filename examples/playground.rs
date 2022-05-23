use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};

use nora_core::orbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};
use nora_physics::{
    rigid_body::RigidBody,
    collider::{ColliderShape, ColliderPhysicalProperties}
};
use nora_object_interaction::{InteractionPlugin, InteractiveBundle, InteractorBundle};
use nora_core::plugins::physics::DefaultPhysicsPlugin;
use nora_physics::impulse::Impulse;
use nora_physics::gravity::GravityScale;


fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(DefaultPhysicsPlugin::default())
        .add_plugin(InteractionPlugin)
        .add_plugin(PanOrbitCameraPlugin)
        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system)
        .insert_resource(ClearColor(Color::hex("F5F5F5").unwrap()))
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // Spawn ground
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 200.0 })),
        material: materials.add(Color::ALICE_BLUE.into()),
        ..Default::default()
    })
        .insert(RigidBody::Fixed)
        .insert(ColliderShape::Cuboid {x_half: 100.0, y_half: 0.0, z_half: 100.0})
        .insert_bundle(InteractiveBundle::default());

    // Spawn sphere
    let num_sphere = 5.0;
    let radius = 4.0;
    for i in 0..(num_sphere as i32) {

        let i_f = i as f32;
        let y = radius * (2.0 * PI * i_f / num_sphere).sin();
        let x = radius * (2.0 * PI * i_f / num_sphere).cos();
        let sphere_radius = 0.5 * (1.0 + i_f);

        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: sphere_radius, subdivisions: 8})),
            material: materials.add(Color::hex("66BB6A").unwrap().into()),
            transform: Transform::from_xyz(x, 10.0, y),
            ..Default::default()
        })
            .insert(RigidBody::Dynamic)
            .insert(ColliderShape::Sphere { radius: sphere_radius })
            .insert(ColliderPhysicalProperties {
                restitution: 0.1*(i as f32),
                ..Default::default()
            })
            .insert(Impulse::default())
            .insert(GravityScale::default())
            .insert_bundle(InteractiveBundle::default());

    }

    // camera
    let camera_pos = Vec3::new(0.0, 8.0, 18.0);
    let distance = camera_pos.length();
    let camera_transform = Transform::from_translation(camera_pos)
        .looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: camera_transform,
        ..Default::default()
    })
        .insert(PanOrbitCamera {
        distance,
        ..Default::default()
    })
        .insert_bundle(InteractorBundle::default());

    // Light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 60000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 40.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}