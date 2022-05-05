use bevy::prelude::*;
use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};

use nora_core::plugins::CorePlugins;
use nora_physics::{
    collider::ColliderComp,
    rigid_body::RigidBodyComp
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
        .insert(RigidBodyComp::Fixed)
        .insert(ColliderComp::Cuboid {x_half: 5.0, y_half: 0.0, z_half: 5.0});


    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0})),
        material: materials.add(Color::hex("FF0000").unwrap().into()),
        transform: Transform::from_xyz(0.0, 3.0, 0.0),
        ..Default::default()
    })
        .insert(RigidBodyComp::Dynamic)
        .insert(ColliderComp::Cuboid {x_half: 0.5, y_half: 0.5, z_half: 0.5});


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
