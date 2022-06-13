use bevy::{ecs::entity::Entity, prelude::error};
use rapier3d::{
    geometry::CollisionEventFlags, 
    pipeline::EventHandler
};


pub enum CollisionEvent {
    Started {
        data: CollisionData
    },
    Stopped {
        data: CollisionData
    }
}

pub struct CollisionData {
    entity1: Entity,
    entity2: Entity,
    flag: CollisionEventFlags
}


struct CollisionEventHandler {
    collision_recv: crossbeam::channel::Receiver<rapier3d::geometry::CollisionEvent>,
    collision_send: crossbeam::channel::Sender<rapier3d::geometry::CollisionEvent>
}

impl EventHandler for CollisionEventHandler {
    fn handle_collision_event(
        &self,
        bodies: &rapier3d::prelude::RigidBodySet,
        colliders: &rapier3d::prelude::ColliderSet,
        event: rapier3d::prelude::CollisionEvent,
        contact_pair: Option<&rapier3d::prelude::ContactPair>
    ) {
        if let Err(e) = self.collision_send.send(event) {
            error!("Failed to propagate collision event: {e}");
        }
    }
}
