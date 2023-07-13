// use std::time::Duration;

use bevy::{prelude::*, app::ScheduleRunnerSettings};

use kesko_core::{interaction::groups::GroupDynamic, event::SimulatorRequestEvent};

use kesko_diagnostic::DiagnosticsPlugins;
use kesko_plugins::CorePlugins;

use kesko_models::{car::CarPlugin, wheely::WheelyPlugin};
use kesko_object_interaction::InteractiveBundle;
use kesko_physics::{
    collider::ColliderShape, event::collision::GenerateCollisionEvents, force::Force,
    gravity::GravityScale, rigid_body::RigidBody, joint::{ JointMotorEvent ,MotorCommand },
};

fn main() {
    App::new()
        .add_plugins(CorePlugins::default())
        // .add_plugins(DiagnosticsPlugins)
        .add_plugin(CarPlugin)
        .add_plugin(WheelyPlugin)
        .add_plugin(kesko_urdf::UrdfPlugin)
        .add_plugin(placeholder_box::PlaceholderBoxPlugin)
        .add_plugin(renet_transport::RenetTransportPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    kesko_models::arena::spawn(
        &mut commands,
        materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        }),
        &mut meshes,
        10.0,
        10.0,
        0.75,
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
        ..default()
    });
}
