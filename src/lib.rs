use bevy::prelude::*;

use nora_core::plugins::CorePlugins;
use nora_physics::{
    collider::{ColliderShape, ColliderPhysicalProperties},
    rigid_body::RigidBody,
    gravity::GravityScale,
    impulse::Impulse
};
use nora_object_interaction::{
    InteractiveBundle
};


pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins)
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
        mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: materials.add(Color::hex("ECEFF1").unwrap().into()),
        ..Default::default()
    })
        .insert(RigidBody::Fixed)
        .insert(ColliderShape::Cuboid {x_half: 5.0, y_half: 0.0, z_half: 5.0});

    // Spawn cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0})),
        material: materials.add(Color::hex("FF9800").unwrap().into()),
        transform: Transform::from_xyz(0.0, 3.0, 0.0),
        ..Default::default()
    })
        .insert(RigidBody::Dynamic)
        .insert(ColliderShape::Cuboid {x_half: 0.5, y_half: 0.5, z_half: 0.5})
        .insert(GravityScale::default())
        .insert(Impulse::default())
        .insert_bundle(InteractiveBundle::default());

    // Spawn sphere
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.5, subdivisions: 5})),
        material: materials.add(Color::hex("4CAF50").unwrap().into()),
        transform: Transform::from_xyz(-2.0, 3.0, 0.0),
        ..Default::default()
    })
        .insert(RigidBody::Dynamic)
        .insert(ColliderShape::Sphere { radius: 0.5 })
        .insert(ColliderPhysicalProperties {
            restitution: 1.0,
            ..Default::default()
        })
        .insert(GravityScale::default())
        .insert(Impulse::default())
        .insert_bundle(InteractiveBundle::default());

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 0.1,
            depth: 2.0,
            ..Default::default()
        })),
        material: materials.add(Color::hex("E91E63").unwrap().into()),
        transform: Transform::from_xyz(2.0, 3.0, 0.0).with_rotation(Quat::from_rotation_x(0.1)),
        ..Default::default()
    })
        .insert(RigidBody::Dynamic)
        .insert(ColliderShape::CapsuleY { half_length: 1.0, radius: 0.1 })
        .insert(ColliderPhysicalProperties {
            restitution: 0.7,
            ..Default::default()
        })
        .insert(GravityScale::default())
        .insert(Impulse::default())
        .insert_bundle(InteractiveBundle::default());

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
