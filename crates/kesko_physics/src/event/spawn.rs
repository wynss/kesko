use bevy::prelude::*;
use serde::{Serialize, Deserialize};


use crate::{
    multibody::MultibodyRoot, 
    rigid_body::{RigidBody, RigidBodyName}
};


#[derive(Clone, Serialize, Deserialize)]
pub enum BodySpawnedEvent {
    MultibodySpawned {
        name: String
    },
    RigidBodySpawned {
        name: String
    }
}

pub(crate) fn send_spawned_events(
    mut event_writer: EventWriter<BodySpawnedEvent>,
    bodies: Query<(&RigidBodyName, Option<&MultibodyRoot>), Added<RigidBody>>
) {

    for (name, root) in bodies.iter() {
        warn!("Sending spawn event"); 
        if let Some(root) = root {
            event_writer.send(BodySpawnedEvent::MultibodySpawned{ name: root.name.clone() });
        } else {
            event_writer.send(BodySpawnedEvent::RigidBodySpawned{ name: name.0.clone() })
        }
    }
}
