use bevy::prelude::*;

use crate::{
    event::PhysicResponseEvent,
    multibody::MultibodyRoot, 
    rigid_body::{RigidBody, RigidBodyName}
};


pub(crate) fn send_spawned_events(
    mut event_writer: EventWriter<PhysicResponseEvent>,
    bodies: Query<(Entity, &RigidBodyName, Option<&MultibodyRoot>), Added<RigidBody>>
) {

    for (entity, name, root) in bodies.iter() {
        debug!("Sending spawn event"); 
        if let Some(root) = root {

            event_writer.send(PhysicResponseEvent::MultibodySpawned{ 
                id: entity.to_bits(),
                name: root.name.clone(),
                links: root.child_map.clone()
            });
        } else {
            event_writer.send(PhysicResponseEvent::RigidBodySpawned{ 
                id: entity.to_bits(),
                name: name.0.clone() 
            })
        }
    }
}
