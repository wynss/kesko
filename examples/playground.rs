use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

use bevy::prelude::*;
use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};

use nora_core::{
    bundle::PhysicBodyBundle,
    shape::Shape,
    orbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera},
    plugins::physics::DefaultPhysicsPlugin,
    diagnostic::FPSScreenPlugin
};
use nora_object_interaction::{InteractionPlugin, InteractiveBundle, InteractorBundle};
use nora_physics::{
    rigid_body::RigidBody,
    joint::{
        Joint,
        spherical::SphericalJoint,
        fixed::FixedJoint,
        revolute::RevoluteJoint
    },
};


fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(DefaultPhysicsPlugin::default())
        .add_plugin(InteractionPlugin)
        .add_plugin(PanOrbitCameraPlugin)
        .add_plugin(FPSScreenPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(spawn_car)
        .add_startup_system(spawn_spider)
        .add_system(bevy::input::system::exit_on_esc_system)
        .insert_resource(ClearColor(Color::hex("F5F5F5").unwrap()))
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    commands.spawn_batch(arena(&mut materials, &mut meshes));

    // Spawn spheres
    let num_sphere = 5.0;
    for i in 0..(num_sphere as i32) {

        let i_f = i as f32;
        let sphere_radius = 0.05 * (1.0 + i_f);
        let z = - 1.0 +  8.0 * sphere_radius;

        commands.spawn_bundle(PhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere {radius: sphere_radius, subdivisions: 5},
            materials.add(Color::hex("66BB6A").unwrap().into()),
            Transform::from_xyz(-1.0, 4.0, z),
            &mut meshes
        )).insert_bundle(InteractiveBundle::default());
    }

    // spawn multi body
    let mut root = commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule { radius: 0.1, length: 0.3},
        materials.add(Color::PINK.into()),
        Transform::from_xyz(0.0, 1.0, 0.0),
        &mut meshes
    ))
        .insert_bundle(InteractiveBundle::default())
        .id();

    for i in 1..4 {
        let child = commands.spawn_bundle( PhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: 0.1, length: 0.3},
            materials.add(Color::PINK.into()),
            Transform::from_xyz(0.0, 1.0 + 0.5*(i as f32), 0.0),
            &mut meshes
        ))
            .insert(Joint::new(root, SphericalJoint {
                parent_anchor: Transform::from_translation(Vec3::new(0.0, 0.25, 0.0)),
                child_anchor: Transform::from_translation(Vec3::new(0.0, -0.25, 0.0)),
                ..default()
            }))
            .insert_bundle(InteractiveBundle::default())
            .id();

        root = child;
    }

    // camera
    let camera_pos = Vec3::new(9.0, 5.0, 9.0);
    let distance = camera_pos.length();
    let camera_transform = Transform::from_translation(camera_pos)
        .looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: camera_transform,
        ..Default::default()
    })
        .insert(PanOrbitCamera {
        distance,
        ..Default::default()
    })
        .insert_bundle(InteractorBundle::default());

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

fn arena(materials: &mut ResMut<Assets<StandardMaterial>>, meshes: &mut ResMut<Assets<Mesh>>) -> Vec<PhysicBodyBundle> {

    // Spawn ground
    let ground = PhysicBodyBundle::from(
        RigidBody::Fixed,
        Shape::Box {x_length: 10.0, y_length: 0.5, z_length: 10.0},
        materials.add(Color::ALICE_BLUE.into()),
        Transform::from_xyz(0.0, -0.25, 0.0),
        meshes
    );

    // left wall
    let left_wall = PhysicBodyBundle::from(
        RigidBody::Fixed,
        Shape::Box {x_length: 0.5, y_length: 1.0, z_length: 10.0},
        materials.add(Color::ALICE_BLUE.into()),
        Transform::from_xyz(-4.75, 0.0, 0.0),
        meshes
    );

    // right wall
    let right_wall = PhysicBodyBundle::from(
        RigidBody::Fixed,
        Shape::Box {x_length: 0.5, y_length: 1.0, z_length: 10.0},
        materials.add(Color::ALICE_BLUE.into()),
        Transform::from_xyz(4.75, 0.0, 0.0),
        meshes
    );

    // back wall
    let back_wall = PhysicBodyBundle::from(
        RigidBody::Fixed,
        Shape::Box {x_length: 10.0, y_length: 1.0, z_length: 0.5},
        materials.add(Color::ALICE_BLUE.into()),
        Transform::from_xyz(0.0, 0.0, -4.75),
        meshes
    );

    // front wall
    let front_wall = PhysicBodyBundle::from(
        RigidBody::Fixed,
        Shape::Box {x_length: 10.0, y_length: 1.0, z_length: 0.5},
        materials.add(Color::ALICE_BLUE.into()),
        Transform::from_xyz(0.0, 0.0, 4.75),
        meshes
    );

    vec![ground, left_wall, right_wall, front_wall, back_wall]
}


fn spawn_car(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    let frame_width = 0.5;
    let frame_height = 0.1;
    let frame_length = 1.0;
    let wheel_radius = 0.18;
    let wheel_width = 0.08;
    let wheel_base = frame_width + 0.2;

    let wbh = wheel_base / 2.0;
    let flh = frame_length / 2.0;


    // Frame
    let frame = commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box {x_length: frame_width, y_length: frame_height, z_length: frame_length},
        materials.add(Color::GOLD.into()),
        Transform::from_xyz(2.0, 0.2, 0.0),
        &mut meshes
    ))
        .insert_bundle(InteractiveBundle::default())
        .id();

    // front wall
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box {x_length: frame_width, y_length: 0.2, z_length: 0.1},
        materials.add(Color::GOLD.into()),
        Transform::from_xyz(2.0, 0.35, flh - 0.05),
        &mut meshes
    ))
        .insert(Joint::new(frame, FixedJoint {
            parent_anchor: Transform::from_translation(Vec3::new(0.0, frame_height / 2.0, flh - 0.05)),
            child_anchor: Transform::from_translation(Vec3::new(0.0, -0.1, 0.0)),
        }))
        .insert_bundle(InteractiveBundle::default());

    // back wall
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box {x_length: frame_width, y_length: 0.2, z_length: 0.1},
        materials.add(Color::GOLD.into()),
        Transform::from_xyz(2.0, 0.35, -flh + 0.05),
        &mut meshes
    ))
        .insert(Joint::new(frame, FixedJoint {
            parent_anchor: Transform::from_translation(Vec3::new(0.0, frame_height / 2.0, 0.05 - flh)),
            child_anchor: Transform::from_translation(Vec3::new(0.0, -0.1, 0.0)),
        }))
        .insert_bundle(InteractiveBundle::default());

    // left wall
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box {x_length: 0.1, y_length: 0.2, z_length: frame_length - 0.2},
        materials.add(Color::GOLD.into()),
        Transform::from_xyz(2.0 + frame_width / 2.0, 0.35, 0.0),
        &mut meshes
    ))
        .insert(Joint::new(frame, FixedJoint {
            parent_anchor: Transform::from_translation(Vec3::new(-frame_width / 2.0 + 0.05, frame_height / 2.0, 0.0)),
            child_anchor: Transform::from_translation(Vec3::new(0.0, -0.1, 0.0)),
        }))
        .insert_bundle(InteractiveBundle::default());

    // right wall
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box {x_length: 0.1, y_length: 0.2, z_length: frame_length - 0.2},
        materials.add(Color::GOLD.into()),
        Transform::from_xyz(2.0 - frame_width / 2.0, 0.4, 0.0),
        &mut meshes
    ))
        .insert(Joint::new(frame, FixedJoint {
            parent_anchor: Transform::from_translation(Vec3::new(frame_width / 2.0 - 0.05, frame_height / 2.0, 0.0)),
            child_anchor: Transform::from_translation(Vec3::new(0.0, -0.1, 0.0)),
        }))
        .insert_bundle(InteractiveBundle::default());

    // left front wheel
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Cylinder { radius: wheel_radius, length: wheel_width},
        materials.add(Color::BLACK.into()),
        Transform::from_xyz(2.0 + wbh, 0.0, flh),
        &mut meshes
    ))
        .insert(Joint::new(frame, RevoluteJoint {
            parent_anchor: Transform::from_translation(Vec3::new(wbh, 0.0, flh)),
            child_anchor: Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2)),
            axis: Vec3::Y,
            ..default()
        }));

    // right front wheel
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Cylinder { radius: 0.16, length: 0.08},
        materials.add(Color::BLACK.into()),
        Transform::from_xyz(2.0 - wbh, 0.0, flh),
        &mut meshes
    ))
        .insert(Joint::new(frame, RevoluteJoint {
            parent_anchor: Transform::from_translation(Vec3::new(-wbh, 0.0, flh)),
            child_anchor: Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2)),
            axis: Vec3::Y,
            ..default()
        }));

    // left back wheel
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Cylinder { radius: 0.16, length: 0.08},
        materials.add(Color::BLACK.into()),
        Transform::from_xyz(2.0 + wbh, 0.0, -flh),
        &mut meshes
    ))
        .insert(Joint::new(frame, RevoluteJoint {
            parent_anchor: Transform::from_translation(Vec3::new(wbh, 0.0, -flh)),
            child_anchor: Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2)),
            axis: Vec3::Y,
            ..default()
        }));

    // right back wheel
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Cylinder { radius: 0.16, length: 0.08},
        materials.add(Color::BLACK.into()),
        Transform::from_xyz(2.0 - wbh, 0.0, -flh),
        &mut meshes
    ))
        .insert(Joint::new(frame, RevoluteJoint {
            parent_anchor: Transform::from_translation(Vec3::new(-wbh, 0.0, -flh)),
            child_anchor: Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2)),
            axis: Vec3::Y,
            ..default()
        }));
}


fn spawn_spider(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    let body_radius = 0.2;
    let arm_length = 0.3;
    let arm_radius = 0.06;
    let alh = arm_length / 2.0;
    let gap = 0.01;

    let origin = Transform::from_xyz(0.0, 2.0, 2.0);

    // Frame
    let body = commands.spawn_bundle(PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Sphere { radius: body_radius, subdivisions: 7 },
        materials.add(Color::ORANGE_RED.into()),
        origin,
        &mut meshes
    ))
        .insert_bundle(InteractiveBundle::default())
        .id();

    // left_front_leg
    commands.spawn_bundle(PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule { radius: arm_radius, length: arm_length },
        materials.add(Color::ORANGE_RED.into()),
        Transform::from_rotation(Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 1.0), -FRAC_PI_4))
            .with_translation(Vec3::new(1.0, 0.0, 1.0).normalize() + body_radius + alh + arm_radius + gap),
        &mut meshes
    ))
        .insert(Joint::new(body, FixedJoint {
            parent_anchor: Transform::from_translation(body_radius * Vec3::new(1.0, 0.0, 1.0).normalize()),
            child_anchor: Transform::from_translation(Vec3::new(0.0, alh + arm_radius + gap, 0.0))
                .with_rotation(Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 1.0), -FRAC_PI_4)),
        }))
        .insert_bundle(InteractiveBundle::default());

    // right_front_leg
    commands.spawn_bundle(PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule { radius: arm_radius, length: arm_length },
        materials.add(Color::ORANGE_RED.into()),
        Transform::from_rotation(Quat::from_axis_angle(Vec3::new(1.0, 0.0, 1.0), FRAC_PI_4))
            .with_translation(Vec3::new(-1.0, 0.0, 1.0).normalize() * (body_radius + alh + arm_radius + gap)),
        &mut meshes
    ))
        .insert(Joint::new(body, FixedJoint {
            parent_anchor: Transform::from_translation(body_radius * Vec3::new(-1.0, 0.0, 1.0).normalize()),
            child_anchor: Transform::from_translation(Vec3::new(0.0, alh + arm_radius + gap, 0.0)).with_rotation(Quat::from_axis_angle(Vec3::new(1.0, 0.0, 1.0), FRAC_PI_4)),
        }))
        .insert_bundle(InteractiveBundle::default());

    // right_back_leg
    commands.spawn_bundle(PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule { radius: arm_radius, length: arm_length },
        materials.add(Color::ORANGE_RED.into()),
        Transform::from_rotation(Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 1.0), FRAC_PI_4))
            .with_translation((body_radius + alh + arm_radius + gap) * Vec3::new(-1.0, 0.0, -1.0).normalize()),
        &mut meshes
    ))
        .insert(Joint::new(body, FixedJoint {
            parent_anchor: Transform::from_translation(body_radius * Vec3::new(-1.0, 0.0, -1.0).normalize()),
            child_anchor: Transform::from_translation(Vec3::new(0.0, alh + arm_radius + gap, 0.0)).with_rotation(Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 1.0), FRAC_PI_4)),
        }))
        .insert_bundle(InteractiveBundle::default());

    // left_back_leg
    commands.spawn_bundle(PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule { radius: arm_radius, length: arm_length },
        materials.add(Color::ORANGE_RED.into()),
        Transform::from_rotation(Quat::from_axis_angle(Vec3::new(1.0, 0.0, 1.0), -FRAC_PI_4))
            .with_translation((body_radius + alh + arm_radius + gap) * Vec3::new(1.0, 0.0, -1.0).normalize()),
        &mut meshes
    ))
        .insert(Joint::new(body, FixedJoint {
            parent_anchor: Transform::from_translation(body_radius * Vec3::new(1.0, 0.0, -1.0).normalize()),
            child_anchor: Transform::from_translation(Vec3::new(0.0, alh + arm_radius + gap, 0.0)).with_rotation(Quat::from_axis_angle(Vec3::new(1.0, 0.0, 1.0), -FRAC_PI_4)),
        }))
        .insert_bundle(InteractiveBundle::default());
}