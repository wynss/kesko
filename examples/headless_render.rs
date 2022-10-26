use bevy::prelude::*;
use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};

use kesko_core::interaction::groups::GroupDynamic;
use kesko_plugins::HeadlessRenderPlugins;
use kesko_physics::{
    rigid_body::RigidBody,
    collider::{ColliderShape, ColliderPhysicalProperties},
    gravity::GravityScale,
    force::Force,
    event::collision::GenerateCollisionEvents,
};
use kesko_object_interaction::InteractiveBundle;
use kesko_diagnostic::event::log_collision_events;


fn main() {
    App::new()
    .add_plugins(HeadlessRenderPlugins::default())
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_startup_system(setup)
    .add_system(log_collision_events)
    .run();
}


fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) { 

    // spawn sphere that will generate collision events
    commands.spawn_bundle(PbrBundle {
        material: materials.add(Color::PURPLE.into()),
        mesh: meshes.add(Mesh::from(shape::Icosphere {radius: 0.2, subdivisions: 5})),
        transform: Transform::from_translation(Vec3::new(0.0, 4.0, 0.0)),
        ..default()
    })
    .insert(RigidBody::Dynamic)
    .insert(ColliderShape::Sphere { radius: 0.2 })
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Force::default())
    .insert(GravityScale::default())
    .insert(GenerateCollisionEvents)
    .insert(ColliderPhysicalProperties {
        restitution: 0.8,
        ..default()
    });

    kesko_models::plane::spawn(
        &mut commands,
        materials.add(StandardMaterial {
            base_color: Color::rgba(1.0, 1.0, 1.0, 1.0),
            ..default()
        }), 
        &mut meshes,
    );
}
