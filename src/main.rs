use bevy::prelude::*;

use nora_core::{
    plugins::CorePlugins,
    diagnostic::DiagnosticsPlugins,
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
use nora_core::{
    models,
    models::car::CarPlugin
};


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(CorePlugins)
    .add_plugins(DiagnosticsPlugins)
    .add_plugin(CarPlugin)
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
        materials.add(Color::hex("CFD8DC").unwrap().into()), 
        &mut meshes, 
        20.0, 20.0, 1.0
    );

    // spawn sphere that will generate collision events
    commands.spawn_bundle(PbrBundle {
        material: materials.add(Color::PURPLE.into()),
        mesh: meshes.add(Mesh::from(shape::Icosphere {radius: 0.2, subdivisions: 5})),
        transform: Transform::from_translation(Vec3::new(0.0, 1.0, 2.0)),
        ..default()
    })
    .insert(RigidBody::Dynamic)
    .insert(ColliderShape::Sphere { radius: 0.2 })
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
