use bevy::prelude::*;
use heron::prelude::*;

mod camera;

use camera::pan_orbit_camera::{handle_camera, spawn_camera};

pub fn world() -> String {
    String::from("world")
}

pub fn start() {
    App::build()
        .insert_resource(ClearColor(Color::hex("FCE4EC").unwrap()))
        .insert_resource(WindowDescriptor {
            title: String::from("Nora"),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)))
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(handle_camera.system())
        .run();
}

fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    // Spaw ground
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::hex("37474F").unwrap().into()),
        ..Default::default()
    })
    .insert(RigidBody::Static)
    .insert(CollisionShape::Cuboid {
        half_extends: Vec3::new(2.5, 0.0, 2.5),
        border_radius: None
    });

    // Ball
    let radius = 0.2;
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius,
            subdivisions: 4
        })),
        material: materials.add(Color::hex("f44336").unwrap().into()),
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..Default::default()
    })
    .insert(RigidBody::Dynamic)
    .insert(CollisionShape::Sphere { radius });

    // Light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    spawn_camera(commands);

}
