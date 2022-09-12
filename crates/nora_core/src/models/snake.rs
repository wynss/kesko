use bevy::prelude::*;
use nora_physics::{
    rigid_body::{RigidBody, RigidBodyName},
    joint::{
        Joint,
        spherical::SphericalJoint
    }
};
use nora_object_interaction::InteractiveBundle;
use crate::{
    bundle::MeshPhysicBodyBundle,
    shape::Shape,
    transform::get_world_transform,
    interaction::groups::GroupDynamic
};


pub fn spawn_snake(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    transform: Transform,
    meshes: &mut Assets<Mesh>
) {

    let radius = 0.1;
    let length = 0.3;
    let half_length = length / 2.0 + radius;
    let margin = 0.02;

    let mut new_origin = transform;

    let mut root = commands.spawn_bundle( MeshPhysicBodyBundle::from(
        RigidBody::Dynamic,
        Shape::Capsule {radius, length},
        material.clone(),
        new_origin,
        meshes
    ))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(RigidBodyName("snake".to_owned()))
    .id();

    for i in 1..4 {

        let parent_anchor = Transform::from_translation((half_length + margin) * Vec3::Y);
        let child_anchor = Transform::from_translation(-(half_length + margin) * Vec3::Y);
        let world_transform = get_world_transform(&new_origin, &parent_anchor, &child_anchor);

        let child = commands.spawn_bundle( MeshPhysicBodyBundle::from(
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
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName(format!("joint {i}")))
        .id();
        
        new_origin = world_transform;
        root = child;
    }
}