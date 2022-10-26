use bevy::{
    prelude::*, 
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}
};

use kesko_plugins::HeadlessRenderPlugins;
use kesko_tcp::TcpPlugin;


fn main() {
    App::new()
        .add_plugins(HeadlessRenderPlugins {
            initial_physic_state: kesko_physics::PhysicState::Stopped
        })
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(TcpPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands
) {
    // Light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn_bundle(DirectionalLightBundle {
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
