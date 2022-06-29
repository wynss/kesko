use std::f32::consts::FRAC_PI_4;

use bevy::prelude::*;

use nora_physics::{
    rigid_body::RigidBody,
    joint::{
        Joint,
        revolute::RevoluteJoint
    }
};
use nora_core::{
    shape::Shape,
    bundle::MeshPhysicBodyBundle,
    transform::get_world_transform,
    interaction::groups::GroupDynamic
};
use nora_object_interaction::InteractiveBundle;


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

    let leg_stiffness = 1.0;
    let leg_damping = 0.0;

    // Frame
    let body = commands.spawn_bundle(MeshPhysicBodyBundle::from(RigidBody::Dynamic,
        Shape::Sphere { radius: body_radius, subdivisions: 7 }, material.clone(), origin, meshes
    ))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .id();


    // left front leg x
    let parent_anchor = Transform::from_translation((body_radius + leg_radius) * Vec3::new(1.0, 0.0, 1.0).normalize())
        .with_rotation(Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 1.0).normalize(), leg_angle));
    let child_anchor = Transform::default();
    let world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
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
        axis: Vec3::X,
        stiffness: leg_stiffness,
        damping: leg_damping,
        ..default()
    })).id();

    // left front leg z
    let parent_anchor = Transform::default();
    let child_anchor = Transform::from_translation((half_leg + leg_radius) * Vec3::Y);
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule { radius: leg_radius, length: leg_length },
        material.clone(),
        get_world_transform(&world_transform, &parent_anchor, &child_anchor),
        meshes
    )).insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Joint::new(left_front_x, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Vec3::Z,
        stiffness: leg_stiffness,
        damping: leg_damping,
        ..default()
    }));

    // right front leg x
    let parent_anchor = Transform::from_translation((body_radius + leg_radius) * Vec3::new(-1.0, 0.0, 1.0).normalize())
        .with_rotation(Quat::from_axis_angle(Vec3::new(1.0, 0.0, 1.0).normalize(), -leg_angle));
    let child_anchor = Transform::default();
    let world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
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
        axis: Vec3::X,
        stiffness: leg_stiffness,
        damping: leg_damping,
        ..default()
    })).id();
    // right front leg z
    let parent_anchor = Transform::default();
    let child_anchor = Transform::from_translation((half_leg + leg_radius) * Vec3::Y);
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule { radius: leg_radius, length: leg_length },
        material.clone(),
        get_world_transform(&world_transform, &parent_anchor, &child_anchor),
        meshes
    )).insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Joint::new(right_front_x, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Vec3::Z,
        stiffness: leg_stiffness,
        damping: leg_damping,
        ..default()
    }));

    // left back leg x
    let parent_anchor = Transform::from_translation((body_radius + leg_radius) * Vec3::new(1.0, 0.0, -1.0).normalize())
        .with_rotation(Quat::from_axis_angle(Vec3::new(1.0, 0.0, 1.0).normalize(), leg_angle));
    let child_anchor = Transform::default();
    let world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
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
        axis: Vec3::X,
        stiffness: leg_stiffness,
        damping: leg_damping,
        ..default()
    })).id();
    let parent_anchor = Transform::default();
    let child_anchor = Transform::from_translation((half_leg + leg_radius) * Vec3::Y);
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule { radius: leg_radius, length: leg_length },
        material.clone(),
        get_world_transform(&world_transform, &parent_anchor, &child_anchor),
        meshes
    )).insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Joint::new(left_rear_leg_x, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Vec3::Z,
        stiffness: leg_stiffness,
        damping: leg_damping,
        ..default()
    }));

    // right rear leg x
    let parent_anchor = Transform::from_translation((body_radius + leg_radius) * Vec3::new(-1.0, 0.0, -1.0).normalize())
        .with_rotation(Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 1.0).normalize(), -leg_angle));
    let child_anchor = Transform::default();
    let world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
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
        axis: Vec3::X,
        stiffness: leg_stiffness,
        damping: leg_damping,
        ..default()
    })).id();
    let parent_anchor = Transform::default();
    let child_anchor = Transform::from_translation((half_leg + leg_radius) * Vec3::Y);
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule { radius: leg_radius, length: leg_length },
        material.clone(),
        get_world_transform(&world_transform, &parent_anchor, &child_anchor),
        meshes
    )).insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Joint::new(right_rear_leg_x, RevoluteJoint {
        parent_anchor,
        child_anchor,
        axis: Vec3::Z,
        stiffness: leg_stiffness,
        damping: leg_damping,
        ..default()
    }));
}