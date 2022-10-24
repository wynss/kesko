use bevy::prelude::*;
use kesko_physics::{
    rigid_body::RigidBody,
    joint::{
        spherical::SphericalJoint
    }, mass::Mass
};
use kesko_object_interaction::InteractiveBundle;
use kesko_core::{
    bundle::MeshPhysicBodyBundle,
    shape::Shape,
    transform::world_transform_from_joint_anchors,
    interaction::groups::GroupDynamic
};

pub struct Snake;
impl Snake {
    pub fn spawn(
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        transform: Transform,
        meshes: &mut Assets<Mesh>
    ) {

        let radius = 0.07;
        let length = 0.3;
        let half_length = length / 2.0 + radius;
        let margin = 0.02;

        let mut world_transform = transform;

        let mut root = commands.spawn_bundle( MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule {radius, length},
            material.clone(),
            world_transform,
            meshes
        ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Mass{ val: 0.2 })
        .insert(Name::new("snake"))
        .id();

        for i in 1..4 {

            let parent_anchor = Transform::from_translation((half_length + margin) * Vec3::Y);
            let child_anchor = Transform::from_translation(-(half_length + margin) * Vec3::Y);
            world_transform = world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);

            let child = commands.spawn_bundle( MeshPhysicBodyBundle::from(
                RigidBody::Dynamic,
                Shape::Capsule {radius, length},
                material.clone(),
                world_transform,
                meshes
            ))
            .insert(
                SphericalJoint::attach_to(root)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
            )
            .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
            .insert(Mass{ val: 0.2 })
            .insert(Name::new(format!("joint {i}")))
            .id();
            
            root = child;
        }
    }
}
