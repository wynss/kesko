use bevy::prelude::*;

use kesko_plugins::{
    main_camera::MainCameraPlugin,
    UIPlugin,
    core::CorePlugin,
    InteractionPlugin
};
use kesko_physics::PhysicsPlugin;
use kesko_core::{
    cursor_tracking::GrabablePlugin,
    interaction::groups::{GroupDynamic, GroupStatic}
};
use kesko_diagnostic::DiagnosticsPlugins;
use kesko_tcp::TcpPlugin;


fn main() {
    App::new()
        .add_plugin(UIPlugin)
        .add_plugin(CorePlugin)
        .add_plugin(MainCameraPlugin)
        .add_plugin(PhysicsPlugin {
            initial_state: kesko_physics::PhysicState::Pause,
            ..default()
        })
        .add_plugin(GrabablePlugin::<GroupDynamic>::default())
        .add_plugin(InteractionPlugin::<GroupDynamic>::default())
        .add_plugin(InteractionPlugin::<GroupStatic>::default())

        .add_plugins(DiagnosticsPlugins)
        .add_plugin(TcpPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(20.0, 40.0, -40.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}