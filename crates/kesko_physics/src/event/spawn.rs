use bevy::prelude::*;
use serde::Serialize;

use kesko_event::EventTrait;

use crate::{
    multibody::MultibodyRoot, 
    rigid_body::{RigidBody, RigidBodyName}
};


#[derive(Clone, Serialize)]
enum BodyType {
    Multibody,
    RigidBody
}

#[derive(Clone, Serialize)]
pub struct BodySpawnedEvent {
    name: String,
    body_type: BodyType
}

impl EventTrait for BodySpawnedEvent {
    fn test(&self) {
        
    }
}

pub(crate) fn send_spawned_events(
    mut event_writer: EventWriter<BodySpawnedEvent>,
    bodies: Query<(&RigidBodyName, Option<&MultibodyRoot>), Added<RigidBody>>
) {

    for (name, root) in bodies.iter() {
        
        if let Some(root) = root {
            event_writer.send(BodySpawnedEvent { name: root.name.clone(), body_type: BodyType::Multibody });
        } else {
            event_writer.send(BodySpawnedEvent { name: name.0.clone(), body_type: BodyType::RigidBody })
        }
    }
}
