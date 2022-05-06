use bevy::prelude::*;
use rapier3d::prelude as rapier;
use fnv::FnvHashMap;

use super::rigid_body::RigidBodyHandle;


pub type EntityColliderHandleMap = FnvHashMap<Entity, rapier::ColliderHandle>;

#[derive(Component)]
pub enum ColliderShape {
    Cuboid {
        x_half: f32,
        y_half: f32,
        z_half: f32
    },
    Sphere {
        radius: f32
    }
}

/// Component for setting the physical material properties for a collider
#[derive(Component)]
pub struct ColliderPhysicalProperties {
    /// density of the collider
    pub density: f32,
    /// friction coefficient of the collider
    pub friction: f32,
    /// restitution coefficient, controls how elastic or bouncy the collider is
    pub restitution: f32
}

impl Default for ColliderPhysicalProperties {
    fn default() -> Self {
        Self {
            density: 1.0,
            friction: 0.5,
            restitution: 0.0
        }
    }
}

#[derive(Component)]
pub(crate) struct ColliderHandle(rapier::ColliderHandle);


pub(crate) fn add_collider_to_bodies(
    mut commands: Commands,
    mut entity_collider_map: ResMut<EntityColliderHandleMap>,
    mut collider_set: ResMut<rapier::ColliderSet>,
    mut rigid_body_set: ResMut<rapier::RigidBodySet>,
    query: Query<(Entity, &ColliderShape, &RigidBodyHandle, Option<&ColliderPhysicalProperties>), Without<ColliderHandle>>
) {
    for (entity, collider_comp, rigid_body_handle, material) in query.iter() {

        let collider_builder = match collider_comp {
            ColliderShape::Cuboid {x_half, y_half, z_half} => {
                rapier::ColliderBuilder::cuboid(*x_half, *y_half, *z_half)
            },
            ColliderShape::Sphere {radius} => rapier::ColliderBuilder::ball(*radius)
        };

        let collider = if let Some(material) = material {
            collider_builder
                .density(material.density)
                .friction(material.friction)
                .restitution(material.restitution)
                .build()
        } else {
            collider_builder.build()
        };

        let collider_handle = collider_set.insert_with_parent(
            collider,
            rigid_body_handle.0,
            &mut rigid_body_set
        );

        entity_collider_map.insert(entity, collider_handle);

        commands.entity(entity).insert(ColliderHandle(collider_handle));

    }
}
