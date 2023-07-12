use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

use kesko_core::{
    cursor_tracking::GrabablePlugin,
    interaction::groups::{GroupDynamic, GroupStatic},
    CorePlugin,
};
use kesko_models::ModelPlugin;
use kesko_physics::PhysicsPlugin;
use kesko_plugins::{main_camera::MainCameraPlugin, InteractionPlugin, UIPlugin};
use kesko_tcp::TcpPlugin;

fn main() {
    App::new()
        .add_plugins((
            CorePlugin::default(),
            UIPlugin,
            ModelPlugin,
            MainCameraPlugin,
            PhysicsPlugin {
                initial_state: kesko_physics::PhysicState::Stopped,
                ..default()
            },
            GrabablePlugin::<GroupDynamic>::default(),
            InteractionPlugin::<GroupDynamic>::default(),
            InteractionPlugin::<GroupStatic>::default(),
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
            TcpPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100_000.0,
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
