use bevy::prelude::*;

use kesko_core::interaction::groups::GroupDynamic;
use kesko_object_interaction::InteractiveBundle;
use kesko_physics::{
    collider::{ColliderPhysicalProperties, ColliderShape},
    force::Force,
    gravity::GravityScale,
    rigid_body::RigidBody,
};

pub struct Cube;
impl Cube {
    pub fn spawn(
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        transform: Transform,
        meshes: &mut Assets<Mesh>,
    ) {
        commands.spawn((
            PbrBundle {
                material,
                mesh: meshes.add(
                    shape::Cube {
                        size: 1.0,
                    }
                    .try_into()
                    .unwrap(),
                ),
                transform,
                ..default()
            },
            RigidBody::Dynamic,
            ColliderShape::Cuboid { x_half: 0.5, y_half: 0.5, z_half: 0.5 },
            InteractiveBundle::<GroupDynamic>::default(),
            Force::default(),
            ColliderPhysicalProperties {
                restitution: 0.7,
                ..default()
            },
            GravityScale::default(),
        ));
    }
}
