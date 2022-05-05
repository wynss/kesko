use bevy::prelude::*;
use rapier3d::prelude as rapier;
use fnv::FnvHashMap;

use crate::conversions::IntoRapier;


pub type EntityBodyHandleMap = FnvHashMap<Entity, rapier::RigidBodyHandle>;

#[derive(Component)]
pub enum RigidBodyComp {
    Fixed,
    Dynamic
}

#[derive(Component)]
pub(crate) struct RigidBodyHandleComp(pub(crate) rapier::RigidBodyHandle);


pub(crate) fn add_rigid_bodies(
    mut rigid_body_set: ResMut<rapier::RigidBodySet>,
    mut entity_body_map: ResMut<EntityBodyHandleMap>,
    mut commands: Commands,
    query: Query<(Entity, &RigidBodyComp, &Transform), Without<RigidBodyHandleComp>>
) {

    for (entity, rigid_body_comp, transform) in query.iter() {

        let rigid_body = match rigid_body_comp {
            RigidBodyComp::Fixed => {
                rapier::RigidBodyBuilder::fixed()
                    .translation(transform.translation.into_rapier())
                    .build()

            }
            RigidBodyComp::Dynamic => {
                rapier::RigidBodyBuilder::dynamic()
                    .translation(transform.translation.into_rapier())
                    .build()
            }
        };

        let rigid_body_handle = rigid_body_set.insert(rigid_body);
        entity_body_map.insert(entity, rigid_body_handle);

        // Add the rigid body component to the entity
        commands.entity(entity).insert(RigidBodyHandleComp(rigid_body_handle));
    }
}
