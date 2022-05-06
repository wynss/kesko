use bevy::prelude::*;
use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};

use nora_core::plugins::CorePlugins;
use nora_physics::{
    collider::{ColliderShape, ColliderPhysicalProperties},
    rigid_body::RigidBody
};


pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(CorePlugins)
        .add_startup_system(test_scene)
        .run();
}

fn test_scene(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // Spawn ground
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(Color::hex("ECEFF1").unwrap().into()),
        ..Default::default()
    })
        .insert(RigidBody::Fixed)
        .insert(ColliderShape::Cuboid {x_half: 5.0, y_half: 0.0, z_half: 5.0});

    // Spawn cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0})),
        material: materials.add(Color::hex("FF0000").unwrap().into()),
        transform: Transform::from_xyz(0.0, 3.0, 0.0),
        ..Default::default()
    })
        .insert(RigidBody::Dynamic)
        .insert(ColliderShape::Cuboid {x_half: 0.5, y_half: 0.5, z_half: 0.5});

    // Spawn sphere
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.5, subdivisions: 5})),
        material: materials.add(Color::hex("00FF00").unwrap().into()),
        transform: Transform::from_xyz(-2.0, 3.0, 0.0),
        ..Default::default()
    })
        .insert(RigidBody::Dynamic)
        .insert(ColliderShape::Sphere { radius: 0.5 })
        .insert(ColliderPhysicalProperties {
            restitution: 1.0,
            ..Default::default()
        });

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
