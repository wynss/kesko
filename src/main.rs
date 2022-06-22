use bevy::prelude::*;
use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};

use nora_lib::models;
use nora_core::{
    plugins::CorePlugins,
    diagnostic::event::DebugEventPlugin,
    interaction::groups::GroupDynamic
};
use nora_physics::{
    rigid_body::RigidBody,
    force::Force,
    gravity::GravityScale,
    collider::ColliderShape,
    event::GenerateCollisionEvents
};
use nora_object_interaction::InteractiveBundle;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(CorePlugins)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin)
    .add_plugin(DebugEventPlugin)
    .add_startup_system(test_scene)
    .run();
}

fn test_scene(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    models::arena::spawn_arena(
        &mut commands,
        materials.add(Color::ALICE_BLUE.into()), 
        &mut meshes, 
        10.0, 10.0, 1.0
    );

    models::car::spawn_car(
        &mut commands,
        materials.add(Color::GOLD.into()),
        materials.add(Color::BLACK.into()),
        Transform::from_xyz(2.0, 1.0, 0.0),
        &mut meshes
    );

    models::spider::spawn_spider(
        &mut commands,
        materials.add(Color::ORANGE_RED.into()),
        Transform::from_xyz(0.0, 1.0, 0.0),
        &mut meshes
    );

    models::snake::spawn_snake(
        &mut commands,
        materials.add(Color::PINK.into()),
        Transform::from_xyz(0.0, 1.0, 0.0),
        &mut meshes
    );

    // spawn sphere that will generate collision events
    commands.spawn_bundle(PbrBundle {
        material: materials.add(Color::VIOLET.into()),
        mesh: meshes.add(Mesh::from(shape::Icosphere {radius: 0.5, subdivisions: 5})),
        transform: Transform::from_translation(Vec3::new(0.0, 1.0, 2.0)),
        ..default()
    })
    .insert(RigidBody::Dynamic)
    .insert(ColliderShape::Sphere { radius: 0.5 })
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Force::default())
    .insert(GravityScale::default())
    .insert(GenerateCollisionEvents);

    // Light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(20.0, 40.0, -40.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
