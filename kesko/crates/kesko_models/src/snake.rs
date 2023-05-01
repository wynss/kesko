use bevy::prelude::*;
use kesko_core::{
    bundle::MeshPhysicBodyBundle, interaction::groups::GroupDynamic, shape::Shape,
    transform::world_transform_from_joint_anchors,
};
use kesko_object_interaction::InteractiveBundle;
use kesko_physics::{joint::spherical::SphericalJoint, mass::Mass, rigid_body::RigidBody};

pub struct Snake;
impl Snake {
    pub fn spawn(
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        transform: Transform,
        meshes: &mut Assets<Mesh>,
    ) {
        let radius = 0.07;
        let length = 0.3;
        let half_length = length / 2.0 + radius;
        let margin = 0.02;

        let mut world_transform = transform;

        let mut root = commands
            .spawn((
                MeshPhysicBodyBundle::from(
                    RigidBody::Dynamic,
                    Shape::Capsule { radius, length },
                    material.clone(),
                    world_transform,
                    meshes,
                ),
                InteractiveBundle::<GroupDynamic>::default(),
                Mass { val: 0.2 },
                Name::new("snake"),
            ))
            .id();

        for i in 1..4 {
            let parent_anchor = Transform::from_translation((half_length + margin) * Vec3::Y);
            let child_anchor = Transform::from_translation(-(half_length + margin) * Vec3::Y);
            world_transform =
                world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);

            let child = commands
                .spawn((
                    MeshPhysicBodyBundle::from(
                        RigidBody::Dynamic,
                        Shape::Capsule { radius, length },
                        material.clone(),
                        world_transform,
                        meshes,
                    ),
                    SphericalJoint::attach_to(root)
                        .with_parent_anchor(parent_anchor)
                        .with_child_anchor(child_anchor),
                    InteractiveBundle::<GroupDynamic>::default(),
                    Mass { val: 0.2 },
                    Name::new(format!("joint {i}")),
                ))
                .id();

            root = child;
        }
    }
}
