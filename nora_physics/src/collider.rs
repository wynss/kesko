use bevy::prelude::*;
use rapier3d::prelude as rapier;
use fnv::FnvHashMap;

use super::rigid_body::RigidBodyHandleComp;


pub type EntityColliderHandleMap = FnvHashMap<Entity, rapier::ColliderHandle>;


#[derive(Component)]
pub enum ColliderComp {
    Cuboid {
        x_half: f32,
        y_half: f32,
        z_half: f32
    }
}

#[derive(Component)]
pub(crate) struct ColliderHandleComp(rapier::ColliderHandle);


pub(crate) fn add_collider_to_bodies(
    mut commands: Commands,
    mut entity_collider_map: ResMut<EntityColliderHandleMap>,
    mut collider_set: ResMut<rapier::ColliderSet>,
    mut rigid_body_set: ResMut<rapier::RigidBodySet>,
    query: Query<(Entity, &ColliderComp, &RigidBodyHandleComp), Without<ColliderHandleComp>>
) {
    for (entity, collider_comp, rigid_body_handle) in query.iter() {
        let collider = match collider_comp {
            ColliderComp::Cuboid {x_half, y_half, z_half} => {
                rapier::ColliderBuilder::cuboid(*x_half, *y_half, *z_half).build()
            }
        };

        let collider_handle = collider_set.insert_with_parent(
            collider,
            rigid_body_handle.0,
            &mut rigid_body_set
        );

        entity_collider_map.insert(entity, collider_handle);

        commands.entity(entity).insert(ColliderHandleComp(collider_handle));

    }
}
