use bevy::prelude::*;
use serde::{Serialize, Deserialize};


use crate::{
    multibody::MultibodyRoot, 
    rigid_body::{RigidBody, RigidBodyName}
};


#[derive(Clone, Serialize, Deserialize)]
pub enum BodySpawnedEvent {
    MultibodySpawned {
        id: u64,
        name: String,
        links: Vec<String>
    },
    RigidBodySpawned {
        id: u64,
        name: String
    }
}

pub(crate) fn send_spawned_events(
    mut event_writer: EventWriter<BodySpawnedEvent>,
    bodies: Query<(Entity, &RigidBodyName, Option<&MultibodyRoot>), Added<RigidBody>>
) {

    for (entity, name, root) in bodies.iter() {
        warn!("Sending spawn event"); 
        if let Some(root) = root {

            let mut links = root.child_map.keys().cloned().collect::<Vec<String>>();
            links.sort();
            event_writer.send(BodySpawnedEvent::MultibodySpawned{ 
                id: entity.to_bits(),
                name: root.name.clone(),
                links
            });
        } else {
            event_writer.send(BodySpawnedEvent::RigidBodySpawned{ 
                id: entity.to_bits(),
                name: name.0.clone() 
            })
        }
    }
}
