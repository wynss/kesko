use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8};

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
        .add_startup_system(spawn_spider_system)
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
    let camera_pos = Vec3::new(-9.0, 5.0, 9.0);
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

    let origin = Transform::from_translation(Vec3::new(2.0, 1.0, 0.0));

    let frame_width = 0.5;
    let frame_height = 0.1;
    let frame_length = 1.0;

    let half_frame_length = frame_length / 2.0;
    let half_frame_width = frame_width / 2.0;
    let half_frame_height = frame_height / 2.0;

    let wheel_radius = 0.18;
    let wheel_width = 0.08;
    let wheel_base = frame_width + 0.2;
    let wbh = wheel_base / 2.0;

    let wall_thickness = 0.05;
    let wall_height = 0.2;
    let half_wall_height = wall_height / 2.0;
    let half_wall_thick = wall_thickness / 2.0;


    // Frame
    let frame = commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box {x_length: frame_width, y_length: frame_height, z_length: frame_length},
        materials.add(Color::GOLD.into()),
        origin,
        &mut meshes
    ))
        .insert_bundle(InteractiveBundle::default())
        .id();

    // front wall
    let parent_anchor = Transform::from_translation(Vec3::new(0.0, half_frame_height, half_frame_length - half_wall_thick));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_wall_height, 0.0));
    let world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box {x_length: frame_width, y_length: wall_height, z_length: wall_thickness},
        materials.add(Color::GOLD.into()),
        world_transform,
        &mut meshes
    ))
        .insert(Joint::new(frame, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::default());

    // back wall
    let parent_anchor = Transform::from_translation(Vec3::new(0.0, frame_height / 2.0, -(half_frame_length - half_wall_thick)));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_wall_height, 0.0));
    let world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box {x_length: frame_width, y_length: wall_height, z_length: wall_thickness},
        materials.add(Color::GOLD.into()),
        world_transform,
        &mut meshes
    ))
        .insert(Joint::new(frame, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::default());

    // left wall
    let parent_anchor = Transform::from_translation(Vec3::new(half_frame_width - half_wall_thick, frame_height / 2.0, 0.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_wall_height, 0.0));
    let world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box {x_length: wall_thickness, y_length: wall_height, z_length: frame_length - 2.0 * wall_thickness},
        materials.add(Color::GOLD.into()),
        world_transform,
        &mut meshes
    ))
        .insert(Joint::new(frame, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::default());

    // right wall
    let parent_anchor = Transform::from_translation(Vec3::new(-half_frame_width + half_wall_thick, frame_height / 2.0, 0.0));
    let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_wall_height, 0.0));
    let world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Box {x_length: wall_thickness, y_length: wall_height, z_length: frame_length - 2.0 * wall_thickness},
        materials.add(Color::GOLD.into()),
        world_transform,
        &mut meshes
    ))
        .insert(Joint::new(frame, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::default());

    // left front wheel
    let parent_anchor = Transform::from_translation(Vec3::new(wbh, 0.0, half_frame_length));
    let child_anchor = Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Cylinder { radius: wheel_radius, length: wheel_width},
        materials.add(Color::BLACK.into()),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        &mut meshes
    ))
        .insert(Joint::new(frame, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            ..default()
        }));

    // right front wheel
    let parent_anchor = Transform::from_translation(Vec3::new(-wbh, 0.0, half_frame_length));
    let child_anchor = Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Cylinder { radius: wheel_radius, length: wheel_width},
        materials.add(Color::BLACK.into()),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        &mut meshes
    ))
        .insert(Joint::new(frame, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            ..default()
        }));

    // left back wheel
    let parent_anchor = Transform::from_translation(Vec3::new(wbh, 0.0, -half_frame_length));
    let child_anchor = Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Cylinder { radius: wheel_radius, length: wheel_width},
        materials.add(Color::BLACK.into()),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        &mut meshes
    ))
        .insert(Joint::new(frame, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            ..default()
        }));

    // right back wheel
    let parent_anchor = Transform::from_translation(Vec3::new(-wbh, 0.0, -half_frame_length));
    let child_anchor = Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
    commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Cylinder { radius: wheel_radius, length: wheel_width},
        materials.add(Color::BLACK.into()),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        &mut meshes
    ))
        .insert(Joint::new(frame, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            ..default()
        }));
}


fn spawn_spider(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    origin: Transform,
    meshes: &mut Assets<Mesh>
) {

    let body_radius = 0.2;
    let leg_length = 0.3;
    let leg_radius = 0.06;
    let leg_angle = FRAC_PI_4;

    let half_leg = leg_length / 2.0 + leg_radius;

    // Frame
    let body = commands.spawn_bundle(PhysicBodyBundle::from(RigidBody::Dynamic,
        Shape::Sphere { radius: body_radius, subdivisions: 7 }, material.clone(), origin, meshes
    )).insert_bundle(InteractiveBundle::default()).id();

    // left_front_leg
    let parent_anchor = Transform::from_translation((body_radius + leg_radius) * Vec3::new(1.0, 0.0, 1.0).normalize());
    let child_anchor = Transform::from_translation(half_leg * Vec3::Y).with_rotation(Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 1.0).normalize(), -leg_angle));
    let leg_world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
    commands.spawn_bundle(PhysicBodyBundle::from(RigidBody::Dynamic,
        Shape::Capsule { radius: leg_radius, length: leg_length }, material.clone(),
        leg_world_transform,
        meshes
    )).insert_bundle(InteractiveBundle::default())
    .insert(Joint::new(body, FixedJoint {
        parent_anchor,
        child_anchor
    }));

    // right front leg
    let parent_anchor = Transform::from_translation((body_radius + leg_radius) * Vec3::new(-1.0, 0.0, 1.0).normalize());
    let child_anchor = Transform::from_translation(half_leg * Vec3::Y).with_rotation(Quat::from_axis_angle(Vec3::new(1.0, 0.0, 1.0).normalize(), leg_angle));
    let leg_world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
    commands.spawn_bundle(PhysicBodyBundle::from(RigidBody::Dynamic,
        Shape::Capsule { radius: leg_radius, length: leg_length }, material.clone(),
        leg_world_transform,
        meshes
    )).insert_bundle(InteractiveBundle::default())
    .insert(Joint::new(body, FixedJoint {
        parent_anchor,
        child_anchor
    }));

    // left back leg
    let parent_anchor = Transform::from_translation((body_radius + leg_radius) * Vec3::new(1.0, 0.0, -1.0).normalize());
    let child_anchor = Transform::from_translation(half_leg * Vec3::Y).with_rotation(Quat::from_axis_angle(Vec3::new(1.0, 0.0, 1.0).normalize(), -leg_angle));
    let leg_world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
    commands.spawn_bundle(PhysicBodyBundle::from(RigidBody::Dynamic,
        Shape::Capsule { radius: leg_radius, length: leg_length }, material.clone(),
        leg_world_transform,
        meshes
    )).insert_bundle(InteractiveBundle::default())
    .insert(Joint::new(body, FixedJoint {
        parent_anchor,
        child_anchor
    }));

    // right back leg
    let parent_anchor = Transform::from_translation((body_radius + leg_radius) * Vec3::new(-1.0, 0.0, -1.0).normalize());
    let child_anchor = Transform::from_translation(half_leg * Vec3::Y).with_rotation(Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 1.0).normalize(), leg_angle));
    let leg_world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
    commands.spawn_bundle(PhysicBodyBundle::from(RigidBody::Dynamic,
        Shape::Capsule { radius: leg_radius, length: leg_length }, material.clone(),
        leg_world_transform,
        meshes
    )).insert_bundle(InteractiveBundle::default())
    .insert(Joint::new(body, FixedJoint {
        parent_anchor,
        child_anchor
    }));

}


fn spawn_spider_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    let material = materials.add(Color::ORANGE_RED.into());
    let origin = Transform::from_xyz(0.0, 2.0, 2.0);
    spawn_spider(&mut commands, material, origin, &mut meshes);
}


fn get_world_transform(
    origin: &Transform, 
    parent_anchor: &Transform, 
    child_anchor: &Transform
) -> Transform {
    let translation = origin.translation + parent_anchor.translation - child_anchor.rotation.inverse().mul_vec3(child_anchor.translation);
    let transform = Transform::from_translation(translation).with_rotation(child_anchor.rotation.inverse());
    transform
}
