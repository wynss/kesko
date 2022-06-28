use std::f32::consts::FRAC_PI_4;

use bevy::prelude::*;

use nora_physics::{
    rigid_body::RigidBody,
    joint::{
        Joint,
        spherical::SphericalJoint
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

    let leg_stiffness = 100.0;
    let leg_damping = 100.0;

    // Frame
    let body = commands.spawn_bundle(MeshPhysicBodyBundle::from(RigidBody::Dynamic,
        Shape::Sphere { radius: body_radius, subdivisions: 7 }, material.clone(), origin, meshes
    ))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .id();

    // left_front_leg
    let parent_anchor = Transform::from_translation((body_radius + leg_radius) * Vec3::new(1.0, 0.0, 1.0).normalize())
        .with_rotation(Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 1.0).normalize(), leg_angle));
    let child_anchor = Transform::from_translation(half_leg * Vec3::Y);
    let leg_world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule { radius: leg_radius, length: leg_length }, 
        material.clone(),
        leg_world_transform,
        meshes
    )).insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Joint::new(body, SphericalJoint {
        parent_anchor,
        child_anchor,
        x_stiffness: leg_stiffness,
        y_stiffness: leg_stiffness,
        z_stiffness: leg_stiffness,
        x_damping: leg_damping,
        y_damping: leg_damping,
        z_damping: leg_damping,
        ..default()
    }));

    // right front leg
    let parent_anchor = Transform::from_translation((body_radius + leg_radius) * Vec3::new(-1.0, 0.0, 1.0).normalize())
        .with_rotation(Quat::from_axis_angle(Vec3::new(1.0, 0.0, 1.0).normalize(), -leg_angle));
    let child_anchor = Transform::from_translation(half_leg * Vec3::Y);
    let leg_world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule { radius: leg_radius, length: leg_length },
        material.clone(),
        leg_world_transform,
        meshes
    )).insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Joint::new(body, SphericalJoint {
        parent_anchor,
        child_anchor,
        x_stiffness: leg_stiffness,
        y_stiffness: leg_stiffness,
        z_stiffness: leg_stiffness,
        x_damping: leg_damping,
        y_damping: leg_damping,
        z_damping: leg_damping,
        ..default()
    }));

    // left back leg
    let parent_anchor = Transform::from_translation((body_radius + leg_radius) * Vec3::new(1.0, 0.0, -1.0).normalize())
        .with_rotation(Quat::from_axis_angle(Vec3::new(1.0, 0.0, 1.0).normalize(), leg_angle));
    let child_anchor = Transform::from_translation(half_leg * Vec3::Y);
    let leg_world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule { radius: leg_radius, length: leg_length }, 
        material.clone(),
        leg_world_transform,
        meshes
    )).insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Joint::new(body, SphericalJoint {
        parent_anchor,
        child_anchor,
        x_stiffness: leg_stiffness,
        y_stiffness: leg_stiffness,
        z_stiffness: leg_stiffness,
        x_damping: leg_damping,
        y_damping: leg_damping,
        z_damping: leg_damping,
        ..default()
    }));

    // right back leg
    let parent_anchor = Transform::from_translation((body_radius + leg_radius) * Vec3::new(-1.0, 0.0, -1.0).normalize())
        .with_rotation(Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 1.0).normalize(), -leg_angle));
    let child_anchor = Transform::from_translation(half_leg * Vec3::Y);
    let leg_world_transform = get_world_transform(&origin, &parent_anchor, &child_anchor);
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule { radius: leg_radius, length: leg_length }, 
        material.clone(),
        leg_world_transform,
        meshes
    )).insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(Joint::new(body, SphericalJoint {
        parent_anchor,
        child_anchor,
        x_stiffness: leg_stiffness,
        y_stiffness: leg_stiffness,
        z_stiffness: leg_stiffness,
        x_damping: leg_damping,
        y_damping: leg_damping,
        z_damping: leg_damping,
        ..default()
    }));

}