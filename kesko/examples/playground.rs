use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

use kesko_core::{
    bundle::MeshPhysicBodyBundle,
    interaction::groups::GroupDynamic,
    orbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin},
    shape::Shape,
};
use kesko_diagnostic::DiagnosticsPlugins;
use kesko_models as models;
use kesko_object_interaction::{InteractionPlugin, InteractiveBundle, InteractorBundle};
use kesko_physics::rigid_body::RigidBody;
use kesko_plugins::physics::DefaultPhysicsPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            DefaultPhysicsPlugin::default(),
            InteractionPlugin::<GroupDynamic>::default(),
            PanOrbitCameraPlugin,
            DiagnosticsPlugins,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .insert_resource(ClearColor(Color::hex("F5F5F5").unwrap()))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    models::arena::spawn(
        &mut commands,
        materials.add(Color::ALICE_BLUE.into()),
        &mut meshes,
        10.0,
        10.0,
        0.5,
    );

    models::car::Car::spawn(
        &mut commands,
        materials.add(Color::GOLD.into()),
        materials.add(Color::BLACK.into()),
        Transform::from_xyz(2.0, 1.0, 0.0),
        &mut meshes,
    );

    models::spider::spawn(
        &mut commands,
        materials.add(Color::ORANGE_RED.into()),
        Transform::from_xyz(0.0, 1.0, 2.0),
        &mut meshes,
    );

    models::snake::Snake::spawn(
        &mut commands,
        materials.add(Color::PINK.into()),
        Transform::from_xyz(0.0, 1.0, 0.0),
        &mut meshes,
    );

    // Spawn spheres
    let num_sphere = 5.0;
    for i in 0..(num_sphere as i32) {
        let i_f = i as f32;
        let sphere_radius = 0.05 * (1.0 + i_f);
        let z = -1.0 + 8.0 * sphere_radius;

        commands.spawn((
            MeshPhysicBodyBundle::from(
                RigidBody::Dynamic,
                Shape::Sphere {
                    radius: sphere_radius,
                    subdivisions: 5,
                },
                materials.add(Color::hex("66BB6A").unwrap().into()),
                Transform::from_xyz(-1.0, 4.0, z),
                &mut meshes,
            ),
            InteractiveBundle::<GroupDynamic>::default(),
        ));
    }

    // camera
    let camera_pos = Vec3::new(-9.0, 5.0, 9.0);
    let distance = camera_pos.length();
    let camera_transform = Transform::from_translation(camera_pos).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((
        Camera3dBundle {
            transform: camera_transform,
            ..Default::default()
        },
        PanOrbitCamera {
            dist_to_center: distance,
            ..Default::default()
        },
        InteractorBundle::<GroupDynamic>::default(),
    ));

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(20.0, 40.0, -40.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
