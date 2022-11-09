use std::f32::consts::FRAC_PI_4;

use bevy::prelude::*;

use kesko_core::{
    bundle::MeshPhysicBodyBundle, interaction::groups::GroupDynamic, shape::Shape,
    transform::world_transform_from_joint_anchors,
};
use kesko_object_interaction::InteractiveBundle;
use kesko_physics::{
    event::collision::GenerateCollisionEvents,
    joint::{revolute::RevoluteJoint, KeskoAxis},
    mass::Mass,
    rigid_body::RigidBody,
};

// entity names
const NAME: &str = "spider";

const LEFT_FRONT_X: &str = "left front leg x";
const LEFT_FRONT_Z: &str = "left front leg z";
const RIGHT_FRONT_X: &str = "right front leg x";
const RIGHT_FRONT_Z: &str = "right front leg z";
const LEFT_REAR_X: &str = "left rear leg x";
const LEFT_REAR_Z: &str = "left rear leg z";
const RIGHT_REAR_X: &str = "right rear leg x";
const RIGHT_REAR_Z: &str = "right rear leg z";

pub fn spawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    transform: Transform,
    meshes: &mut Assets<Mesh>,
) {
    let body_radius = 0.2;
    let leg_length = 0.3;
    let leg_radius = 0.06;
    let leg_angle = FRAC_PI_4;

    // safety space
    let space = 0.01;
    let half_leg = leg_length / 2.0 + leg_radius;
    let dist_x = body_radius + leg_radius + space;
    let dist_z = half_leg + leg_radius + space;

    let mass_body = 1.0;
    let mass_leg = 0.2;

    let leg_stiffness = 1.0;
    let leg_damping = 0.2;
    let max_motor_force = 50.0;

    // Frame
    let body = commands
        .spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere {
                radius: body_radius,
                subdivisions: 7,
            },
            material.clone(),
            transform,
            meshes,
        ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Mass { val: mass_body })
        .insert(Name::new(NAME))
        .insert(GenerateCollisionEvents)
        .id();

    // left front leg x
    let parent_anchor =
        Transform::from_translation((dist_x) * Vec3::new(1.0, 0.0, 1.0).normalize()).with_rotation(
            Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 1.0).normalize(), leg_angle),
        );
    let child_anchor = Transform::default();
    let world_transform =
        world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
    let left_front_x = commands
        .spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere {
                radius: leg_radius,
                subdivisions: 5,
            },
            material.clone(),
            world_transform,
            meshes,
        ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(
            RevoluteJoint::attach_to(body)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::X)
                .with_motor_params(leg_stiffness, leg_damping)
                .with_limits(Vec2::new(-FRAC_PI_4, FRAC_PI_4))
                .with_max_motor_force(max_motor_force),
        )
        .insert(Mass { val: mass_leg })
        .insert(Name::new(LEFT_FRONT_X))
        .id();

    // left front leg z
    let parent_anchor = Transform::default();
    let child_anchor = Transform::from_translation((dist_z) * Vec3::Y);
    commands
        .spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule {
                radius: leg_radius,
                length: leg_length,
            },
            material.clone(),
            world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor),
            meshes,
        ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(
            RevoluteJoint::attach_to(left_front_x)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::Z)
                .with_motor_params(leg_stiffness, leg_damping)
                .with_limits(Vec2::new(-FRAC_PI_4, FRAC_PI_4))
                .with_max_motor_force(max_motor_force),
        )
        .insert(Mass { val: mass_leg })
        .insert(Name::new(LEFT_FRONT_Z));

    // right front leg x
    let parent_anchor =
        Transform::from_translation((dist_x) * Vec3::new(-1.0, 0.0, 1.0).normalize())
            .with_rotation(Quat::from_axis_angle(
                Vec3::new(1.0, 0.0, 1.0).normalize(),
                -leg_angle,
            ));
    let child_anchor = Transform::default();
    let world_transform =
        world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
    let right_front_x = commands
        .spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere {
                radius: leg_radius,
                subdivisions: 5,
            },
            material.clone(),
            world_transform,
            meshes,
        ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(
            RevoluteJoint::attach_to(body)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::X)
                .with_motor_params(leg_stiffness, leg_damping)
                .with_limits(Vec2::new(-FRAC_PI_4, FRAC_PI_4))
                .with_max_motor_force(max_motor_force),
        )
        .insert(Mass { val: mass_leg })
        .insert(Name::new(RIGHT_FRONT_X))
        .id();

    // right front leg z
    let parent_anchor = Transform::default();
    let child_anchor = Transform::from_translation((dist_z) * Vec3::Y);
    commands
        .spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule {
                radius: leg_radius,
                length: leg_length,
            },
            material.clone(),
            world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor),
            meshes,
        ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(
            RevoluteJoint::attach_to(right_front_x)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::Z)
                .with_motor_params(leg_stiffness, leg_damping)
                .with_limits(Vec2::new(-FRAC_PI_4, FRAC_PI_4))
                .with_max_motor_force(max_motor_force),
        )
        .insert(Mass { val: mass_leg })
        .insert(Name::new(RIGHT_FRONT_Z));

    // left rear leg x
    let parent_anchor =
        Transform::from_translation((dist_x) * Vec3::new(1.0, 0.0, -1.0).normalize())
            .with_rotation(Quat::from_axis_angle(
                Vec3::new(1.0, 0.0, 1.0).normalize(),
                leg_angle,
            ));
    let child_anchor = Transform::default();
    let world_transform =
        world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
    let left_rear_leg_x = commands
        .spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere {
                radius: leg_radius,
                subdivisions: 5,
            },
            material.clone(),
            world_transform,
            meshes,
        ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(
            RevoluteJoint::attach_to(body)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::X)
                .with_motor_params(leg_stiffness, leg_damping)
                .with_limits(Vec2::new(-FRAC_PI_4, FRAC_PI_4))
                .with_max_motor_force(max_motor_force),
        )
        .insert(Mass { val: mass_leg })
        .insert(Name::new(LEFT_REAR_X))
        .id();

    // left rear leg z
    let parent_anchor = Transform::default();
    let child_anchor = Transform::from_translation((dist_z) * Vec3::Y);
    commands
        .spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule {
                radius: leg_radius,
                length: leg_length,
            },
            material.clone(),
            world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor),
            meshes,
        ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(
            RevoluteJoint::attach_to(left_rear_leg_x)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::Z)
                .with_motor_params(leg_stiffness, leg_damping)
                .with_limits(Vec2::new(-FRAC_PI_4, FRAC_PI_4))
                .with_max_motor_force(max_motor_force),
        )
        .insert(Mass { val: mass_leg })
        .insert(Name::new(LEFT_REAR_Z));

    // right rear leg x
    let parent_anchor =
        Transform::from_translation((dist_x) * Vec3::new(-1.0, 0.0, -1.0).normalize())
            .with_rotation(Quat::from_axis_angle(
                Vec3::new(-1.0, 0.0, 1.0).normalize(),
                -leg_angle,
            ));
    let child_anchor = Transform::default();
    let world_transform =
        world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
    let right_rear_leg_x = commands
        .spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere {
                radius: leg_radius,
                subdivisions: 5,
            },
            material.clone(),
            world_transform,
            meshes,
        ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(
            RevoluteJoint::attach_to(body)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::X)
                .with_motor_params(leg_stiffness, leg_damping)
                .with_limits(Vec2::new(-FRAC_PI_4, FRAC_PI_4))
                .with_max_motor_force(max_motor_force),
        )
        .insert(Mass { val: mass_leg })
        .insert(Name::new(RIGHT_REAR_X))
        .id();

    // right rear leg z
    let parent_anchor = Transform::default();
    let child_anchor = Transform::from_translation((dist_z) * Vec3::Y);
    commands
        .spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule {
                radius: leg_radius,
                length: leg_length,
            },
            material,
            world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor),
            meshes,
        ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(
            RevoluteJoint::attach_to(right_rear_leg_x)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::Z)
                .with_motor_params(leg_stiffness, leg_damping)
                .with_limits(Vec2::new(-FRAC_PI_4, FRAC_PI_4))
                .with_max_motor_force(max_motor_force),
        )
        .insert(Mass { val: mass_leg })
        .insert(Name::new(RIGHT_REAR_Z));
}
