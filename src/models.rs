use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};
use bevy::prelude::*;

use nora_physics::{
    rigid_body::RigidBody,
    joint::{
        Joint,
        fixed::FixedJoint,
        revolute::RevoluteJoint,
        spherical::SphericalJoint
    }
};
use nora_core::{
    bundle::PhysicBodyBundle,
    shape::Shape,
    transform::get_world_transform
};
use nora_object_interaction::InteractiveBundle;


pub fn arena(
    material: Handle<StandardMaterial>, 
    meshes: &mut ResMut<Assets<Mesh>>,
    width: f32,
    length: f32,
    wall_height: f32
) -> Vec<PhysicBodyBundle> {

    let wall_width = 0.2;
    let half_wall_width = wall_width / 2.0;

    // Spawn ground
    let ground = PhysicBodyBundle::from(
        RigidBody::Fixed,
        Shape::Box {x_length: width, y_length: 0.5, z_length: length},
        material.clone(),
        Transform::from_xyz(0.0, -0.25, 0.0),
        meshes
    );

    // right wall
    let left_wall = PhysicBodyBundle::from(
        RigidBody::Fixed,
        Shape::Box {x_length: wall_width, y_length: wall_height, z_length: length},
        material.clone(),
        Transform::from_xyz(-(width / 2.0 + half_wall_width), wall_height / 2.0, 0.0),
        meshes
    );

    // left wall
    let right_wall = PhysicBodyBundle::from(
        RigidBody::Fixed,
        Shape::Box {x_length: wall_width, y_length: wall_height, z_length: length},
        material.clone(),
        Transform::from_xyz(width / 2.0 + half_wall_width, wall_height / 2.0, 0.0),
        meshes
    );

    // back wall
    let back_wall = PhysicBodyBundle::from(
        RigidBody::Fixed,
        Shape::Box {x_length: length, y_length: wall_height, z_length: wall_width},
        material.clone(),
        Transform::from_xyz(0.0, wall_height / 2.0, -(width / 2.0 + half_wall_width)),
        meshes
    );

    // front wall
    let front_wall = PhysicBodyBundle::from(
        RigidBody::Fixed,
        Shape::Box {x_length: length, y_length: wall_height, z_length: wall_width},
        material.clone(),
        Transform::from_xyz(0.0, wall_height / 2.0, width / 2.0 + half_wall_width),
        meshes
    );

    vec![ground, left_wall, right_wall, front_wall, back_wall]
}

pub fn spawn_spider(
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

pub fn spawn_worm(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    origin: Transform,
    meshes: &mut Assets<Mesh>
) {

    let radius = 0.1;
    let length = 0.3;
    let half_length = length / 2.0 + radius; 

    let mut new_origin = origin;

    let mut root = commands.spawn_bundle( PhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule {radius, length},
        material.clone(),
        new_origin,
        meshes
    ))
        .insert_bundle(InteractiveBundle::default())
        .id();

    for _ in 1..4 {

        let parent_anchor = Transform::from_translation(half_length * Vec3::Y);
        let child_anchor = Transform::from_translation(-half_length * Vec3::Y);
        let world_transform = get_world_transform(&new_origin, &parent_anchor, &child_anchor);

        let child = commands.spawn_bundle( PhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule {radius, length},
            material.clone(),
            world_transform,
            meshes
        ))
            .insert(Joint::new(root, SphericalJoint {
                parent_anchor,
                child_anchor,
                ..default()
            }))
            .insert_bundle(InteractiveBundle::default())
            .id();
        
        new_origin = world_transform;
        root = child;
    }
}

pub fn spawn_car(
    commands: &mut Commands,
    material_body: Handle<StandardMaterial>,
    material_wheel: Handle<StandardMaterial>,
    origin: Transform,
    meshes: &mut Assets<Mesh>
) {
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
        material_body.clone(),
        origin,
        meshes
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
        material_body.clone(),
        world_transform,
        meshes
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
        material_body.clone(),
        world_transform,
        meshes
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
        material_body.clone(),
        world_transform,
        meshes
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
        material_body.clone(),
        world_transform,
        meshes
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
        material_wheel.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        meshes
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
        material_wheel.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        meshes
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
        material_wheel.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        meshes
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
        material_wheel.clone(),
        get_world_transform(&origin, &parent_anchor, &child_anchor),
        meshes
    ))
        .insert(Joint::new(frame, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            ..default()
        }));
}