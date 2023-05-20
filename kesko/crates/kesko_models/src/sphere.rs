use bevy::prelude::*;

use kesko_core::interaction::groups::GroupDynamic;
use kesko_object_interaction::InteractiveBundle;
use kesko_physics::{
    collider::{ColliderPhysicalProperties, ColliderShape},
    force::Force,
    gravity::GravityScale,
    rigid_body::RigidBody,
};

pub struct Sphere;
impl Sphere {
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
                    shape::Icosphere {
                        radius: 0.2,
                        subdivisions: 5,
                    }
                    .try_into()
                    .unwrap(),
                ),
                transform,
                ..default()
            },
            RigidBody::Dynamic,
            ColliderShape::Sphere { radius: 0.2 },
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
