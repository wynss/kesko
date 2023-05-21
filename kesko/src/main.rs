use bevy::pbr::CascadeShadowConfigBuilder;
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
            base_color: Color::DARK_GRAY,
            ..default()
        }),
        &mut meshes,
    );

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100_000.0,
            // Configure the projection to better fit the scene
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 10,
            ..default()
        }
        .build(),
        ..default()
    });
}
