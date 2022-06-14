use bevy::prelude::*;
use nora_physics::{
    rigid_body::RigidBody,
    joint::{
        Joint,
        spherical::SphericalJoint
    }
};
use nora_core::{
    bundle::PhysicBodyBundle,
    shape::Shape,
    transform::get_world_transform,
    interaction::groups::GroupDynamic
};
use nora_object_interaction::InteractiveBundle;


pub fn spawn_snake(
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
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
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
            .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
            .id();
        
        new_origin = world_transform;
        root = child;
    }
}