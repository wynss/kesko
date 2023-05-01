use bevy::prelude::*;

use kesko::diagnostic::DiagnosticsPlugins;
use kesko::models::{car::CarPlugin, wheely::WheelyPlugin};
use kesko::plugins::CorePlugins;

fn main() {
    App::new()
        .add_plugins(CorePlugins)
        .add_plugins(DiagnosticsPlugins)
        .add_plugin(CarPlugin)
        .add_plugin(WheelyPlugin)
        .add_startup_system(test_scene)
        .run();
}

fn test_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    kesko_models::plane::spawn(
        &mut commands,
        materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        }),
        &mut meshes,
    );

    // Light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100_000.0,
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
}
