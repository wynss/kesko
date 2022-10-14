use std::f32::consts::FRAC_PI_4;

use bevy::prelude::*;

use kesko_physics::{
    rigid_body::{
        RigidBody, 
        RigidBodyName
    },
    joint::{
        Joint,
        Axis,
        revolute::RevoluteJoint
    }, 
    mass::Mass
};
use kesko_object_interaction::InteractiveBundle;
use kesko_core::{
    shape::Shape,
    bundle::MeshPhysicBodyBundle,
    transform::world_transform_from_joint_anchors,
    interaction::groups::GroupDynamic
};

pub fn spawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    transform: Transform,
    meshes: &mut Assets<Mesh>
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
    let leg_damping = 0.3;

    // Frame
    let body = commands.spawn_bundle(MeshPhysicBodyBundle::from(RigidBody::Dynamic,
        Shape::Sphere { radius: body_radius, subdivisions: 7 }, 
        material.clone(), 
        transform, 
        meshes
    ))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Mass{ val: mass_body})
    .insert(RigidBodyName("spider".to_owned()))
    .id();

    // left front leg x
    let parent_anchor = Transform::from_translation((dist_x) * Vec3::new(1.0, 0.0, 1.0).normalize())
        .with_rotation(Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 1.0).normalize(), leg_angle));
    let child_anchor = Transform::default();
    let world_transform = world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
    let left_front_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Sphere { radius: leg_radius, subdivisions: 5},
        material.clone(),
        world_transform,
        meshes
    )).insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Joint::new(body, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Axis::X,
        stiffness: leg_stiffness,
        damping: leg_damping,
        ..default()
    }))
    .insert(Mass{ val: mass_leg})
    .insert(RigidBodyName("left_front_x".to_owned()))
    .id();

    // left front leg z
    let parent_anchor = Transform::default();
    let child_anchor = Transform::from_translation((dist_z) * Vec3::Y);
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule { radius: leg_radius, length: leg_length },
        material.clone(),
        world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor),
        meshes
    )).insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Joint::new(left_front_x, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Axis::Z,
        stiffness: leg_stiffness,
        damping: leg_damping,
        ..default()
    }))
    .insert(Mass{ val: mass_leg})
    .insert(RigidBodyName("left_front_z".to_owned()));

    // right front leg x
    let parent_anchor = Transform::from_translation((dist_x) * Vec3::new(-1.0, 0.0, 1.0).normalize())
        .with_rotation(Quat::from_axis_angle(Vec3::new(1.0, 0.0, 1.0).normalize(), -leg_angle));
    let child_anchor = Transform::default();
    let world_transform = world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
    let right_front_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Sphere { radius: leg_radius, subdivisions: 5},
        material.clone(),
        world_transform,
        meshes
    )).insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Joint::new(body, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Axis::X,
        stiffness: leg_stiffness,
        damping: leg_damping,
        ..default()
    }))
    .insert(Mass{ val: mass_leg})
    .insert(RigidBodyName("right_front_x".to_owned()))
    .id();

    // right front leg z
    let parent_anchor = Transform::default();
    let child_anchor = Transform::from_translation((dist_z) * Vec3::Y);
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule { radius: leg_radius, length: leg_length },
        material.clone(),
        world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor),
        meshes
    )).insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Joint::new(right_front_x, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Axis::Z,
        stiffness: leg_stiffness,
        damping: leg_damping,
        ..default()
    }))
    .insert(Mass{ val: mass_leg})
    .insert(RigidBodyName("right_front_z".to_owned()));

    // left rear leg x
    let parent_anchor = Transform::from_translation((dist_x) * Vec3::new(1.0, 0.0, -1.0).normalize())
        .with_rotation(Quat::from_axis_angle(Vec3::new(1.0, 0.0, 1.0).normalize(), leg_angle));
    let child_anchor = Transform::default();
    let world_transform = world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
    let left_rear_leg_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Sphere { radius: leg_radius, subdivisions: 5},
        material.clone(),
        world_transform,
        meshes
    )).insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Joint::new(body, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Axis::X,
        stiffness: leg_stiffness,
        damping: leg_damping,
        ..default()
    }))
    .insert(Mass{ val: mass_leg})
    .insert(RigidBodyName("left_rear_x".to_owned()))
    .id();

    // left rear leg z
    let parent_anchor = Transform::default();
    let child_anchor = Transform::from_translation((dist_z) * Vec3::Y);
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule { radius: leg_radius, length: leg_length },
        material.clone(),
        world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor),
        meshes
    )).insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Joint::new(left_rear_leg_x, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Axis::Z,
        stiffness: leg_stiffness,
        damping: leg_damping,
        ..default()
    }))
    .insert(Mass{ val: mass_leg})
    .insert(RigidBodyName("left_rear_z".to_owned()));

    // right rear leg x
    let parent_anchor = Transform::from_translation((dist_x) * Vec3::new(-1.0, 0.0, -1.0).normalize())
        .with_rotation(Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 1.0).normalize(), -leg_angle));
    let child_anchor = Transform::default();
    let world_transform = world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
    let right_rear_leg_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Sphere { radius: leg_radius, subdivisions: 5},
        material.clone(),
        world_transform,
        meshes
    )).insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Joint::new(body, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Axis::X,
        stiffness: leg_stiffness,
        damping: leg_damping,
        ..default()
    }))
    .insert(Mass{ val: mass_leg})
    .insert(RigidBodyName("right_rear_x".to_owned()))
    .id();

    // right rear leg z
    let parent_anchor = Transform::default();
    let child_anchor = Transform::from_translation((dist_z) * Vec3::Y);
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule { radius: leg_radius, length: leg_length },
        material,
        world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor),
        meshes
    )).insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Joint::new(right_rear_leg_x, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Axis::Z,
        stiffness: leg_stiffness,
        damping: leg_damping,
        ..default()
    }))
    .insert(Mass{ val: mass_leg})
    .insert(RigidBodyName("right_rear_z".to_owned()));
}
