use bevy::prelude::*;
use nora_core::{
    diagnostic::event::DebugEventPlugin,
    plugins::CorePlugins,
};
use nora_lib::models;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(CorePlugins)
    .add_plugin(DebugEventPlugin)
    .add_startup_system(setup_scene)
    .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    models::arena::spawn_arena(
        &mut commands, 
        materials.add(Color::SALMON.into()), 
        &mut meshes, 
        20.0, 20.0, 1.0
    );

    models::spider::spawn_spider(
        &mut commands, 
        materials.add(Color::ALICE_BLUE.into()),
        Transform::from_xyz(0.0, 1.0, 0.0),
        &mut meshes, 
    );

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