use bevy::prelude::*;
use heron::prelude::*;

use nora_core::plugins::CorePlugins;


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
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::hex("ECEFF1").unwrap().into()),
        ..Default::default()
    })
    .insert(RigidBody::Static)
    .insert(CollisionShape::Cuboid {
        half_extends: Vec3::new(2.5, 0.0, 2.5),
        border_radius: None
    });

    // Ball
    let radius = 0.2;
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius,
            subdivisions: 4
        })),
        material: materials.add(Color::hex("f44336").unwrap().into()),
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..Default::default()
    })
    .insert(RigidBody::Dynamic)
    .insert(CollisionShape::Sphere { radius })
    .insert(PhysicMaterial {
        restitution: 0.7,
        ..Default::default()
    });

    // Cube
    let size = 0.5;
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube {
            size,
        })),
        material: materials.add(Color::hex("f44336").unwrap().into()),
        transform: Transform::from_xyz(1.0, 2.0, 0.0),
        ..Default::default()
    })
    .insert(RigidBody::Dynamic)
    .insert(CollisionShape::Cuboid { 
        half_extends: Vec3::ONE * size / 2.0,
        border_radius: None,
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
