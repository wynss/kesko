use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use kesko_raycast::{RayCastMethod, RayCastPlugin, RayCastSource, RayVisible};

#[derive(Component, Default)]
struct RayCastGroup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(RayCastPlugin::<RayCastGroup>::default())
        .insert_resource(Msaa::Sample4)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ray-castable sphere
    commands.spawn((
        PbrBundle {
            //mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 1.0, subdivisions: 1})),
            mesh: meshes.add(
                shape::Icosphere {
                    radius: 2.0,
                    subdivisions: 5,
                }
                .try_into()
                .unwrap(),
            ),
            material: materials.add(Color::GOLD.into()),
            transform: Transform::from_xyz(0.0, 0.0, -3.0),
            ..Default::default()
        },
        RayVisible::<RayCastGroup>::default(),
    ));

    // camera that can cast rays from screen space
    let camera_pos = Vec3::new(0.0, 0.0, 5.0);
    let camera_transform = Transform::from_translation(camera_pos).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((
        Camera3dBundle {
            transform: camera_transform,
            ..Default::default()
        },
        RayCastSource::<RayCastGroup>::new(RayCastMethod::ScreenSpace),
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 4000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
