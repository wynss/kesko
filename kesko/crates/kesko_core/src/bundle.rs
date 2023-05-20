use bevy::prelude::*;

use crate::shape::Shape;
use kesko_physics::collider::{ColliderPhysicalProperties, ColliderShape};
use kesko_physics::force::Force;
use kesko_physics::gravity::GravityScale;
use kesko_physics::impulse::Impulse;
use kesko_physics::rigid_body::{CanSleep, RigidBody};

#[derive(Bundle)]
pub struct PhysicBodyBundle {
    rigid_body: RigidBody,
    collider_shape: ColliderShape,
    collider_physical_properties: ColliderPhysicalProperties,
    impulse: Impulse,
    force: Force,
    /// How much the entity will be affected by gravity
    gravity_scale: GravityScale,
    can_sleep: CanSleep,

    transform_bundle: TransformBundle,
}

impl PhysicBodyBundle {
    pub fn from(body_type: RigidBody, shape: Shape, transform: Transform) -> Self {
        Self {
            rigid_body: body_type,
            collider_shape: shape.into_collider_shape(),
            collider_physical_properties: ColliderPhysicalProperties::default(),
            impulse: Impulse::default(),
            force: Force::default(),
            gravity_scale: GravityScale::default(),
            can_sleep: CanSleep(false),

            transform_bundle: TransformBundle {
                local: transform,
                ..Default::default()
            },
        }
    }
}

/// Component bundle for physic enabled Pbr entities
#[derive(Bundle)]
pub struct MeshPhysicBodyBundle {
    rigid_body: RigidBody,
    collider_shape: ColliderShape,
    collider_physical_properties: ColliderPhysicalProperties,
    impulse: Impulse,
    force: Force,
    /// How much the entity will be affected by gravity
    gravity_scale: GravityScale,
    can_sleep: CanSleep,

    pbr_bundle: PbrBundle,
}

impl MeshPhysicBodyBundle {
    pub fn from(
        body_type: RigidBody,
        shape: Shape,
        material: Handle<StandardMaterial>,
        transform: Transform,
        meshes: &mut Assets<Mesh>,
    ) -> Self {
        let mesh = shape.into_mesh().unwrap();
        let collider_shape = shape.into_collider_shape();

        Self {
            rigid_body: body_type,
            collider_shape,
            collider_physical_properties: ColliderPhysicalProperties::default(),
            impulse: Impulse::default(),
            force: Force::default(),
            gravity_scale: GravityScale::default(),
            can_sleep: CanSleep(false),

            pbr_bundle: PbrBundle {
                mesh: meshes.add(mesh),
                material,
                transform,
                ..Default::default()
            },
        }
    }
}
