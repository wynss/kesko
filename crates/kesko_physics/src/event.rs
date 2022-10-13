pub mod collision;
pub mod spawn;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use serde::{Serialize, Deserialize};

use rapier3d::prelude as rapier;

use crate::{
    rigid_body::{
        Entity2BodyHandle, 
        RigidBodyHandle
    }
};

use super::{
    PhysicState,
    multibody::MultibodyRoot
};


pub enum PhysicRequestEvent {
    PausePhysics,
    RunPhysics,
    TogglePhysics,
    DespawnBody(u64),
    DespawnAll
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PhysicResponseEvent {
    StartedPhysics,
    StoppedPhysics,
    DespawnedBody(u64),
    DespawnedAllBodies,
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

pub(crate) fn handle_events(
    mut rigid_body_set: ResMut<rapier::RigidBodySet>,
    mut collider_set: ResMut<rapier::ColliderSet>,
    mut impulse_set: ResMut<rapier::ImpulseJointSet>,
    mut multibody_joint_set: ResMut<rapier::MultibodyJointSet>,
    mut islands: ResMut<rapier::IslandManager>,
    entity_2_body_handle: Res<Entity2BodyHandle>,
    mut commands: Commands,
    current_physic_state: Res<CurrentState<PhysicState>>,
    mut request_events: EventReader<PhysicRequestEvent>,
    mut response_events: EventWriter<PhysicResponseEvent>,
    query: Query<(Entity, Option<&MultibodyRoot>), With<RigidBodyHandle>>
) {
    for event in request_events.iter() {
        match event {
            PhysicRequestEvent::PausePhysics => {
                commands.insert_resource(NextState(PhysicState::Stopped));
                response_events.send(PhysicResponseEvent::StoppedPhysics);
            },
            PhysicRequestEvent::RunPhysics => {
                commands.insert_resource(NextState(PhysicState::Running));
                response_events.send(PhysicResponseEvent::StartedPhysics);
            },
            PhysicRequestEvent::TogglePhysics => {
                match current_physic_state.0 {
                    PhysicState::Stopped => {
                        commands.insert_resource(NextState(PhysicState::Running));
                        response_events.send(PhysicResponseEvent::StartedPhysics);
                    },
                    PhysicState::Running => {
                        commands.insert_resource(NextState(PhysicState::Stopped));
                        response_events.send(PhysicResponseEvent::StoppedPhysics);
                    },
                }
            },
            PhysicRequestEvent::DespawnBody(id) => {
                info!("Despawning body with id {:?}", id);
                let entity = Entity::from_bits(*id);

                match query.get(entity) {
                    Ok((_, root)) => {
                        let mut entities_to_remove = vec![entity];

                        if let Some(root) = root {
                            entities_to_remove.extend(root.child_map.values().cloned());
                        }

                        for entity in entities_to_remove.iter() {

                            info!("Despawning entity {:?}", entity);
                            commands.entity(*entity).despawn_recursive();

                            if let Some(body_handle) = entity_2_body_handle.get(entity) {
                                despawn_rapier_body(
                                    *body_handle, 
                                    &mut rigid_body_set,
                                    &mut islands, 
                                    &mut collider_set, 
                                    &mut impulse_set, 
                                    &mut multibody_joint_set
                                );
                            }
                        }
                    },
                    Err(e) => error!("Could not get body to remove {}", e)
                }
                response_events.send(PhysicResponseEvent::DespawnedBody(*id) )
            },
            PhysicRequestEvent::DespawnAll => {
                query.for_each(|(e, _)| {
                    commands.entity(e).despawn_recursive();
                    if let Some(body_handle) = entity_2_body_handle.get(&e) {
                        despawn_rapier_body(
                            *body_handle, 
                            &mut rigid_body_set,
                            &mut islands, 
                            &mut collider_set, 
                            &mut impulse_set, 
                            &mut multibody_joint_set
                        );
                    }
                });
                response_events.send(PhysicResponseEvent::DespawnedAllBodies);
            }
        }
    }
}

fn despawn_rapier_body(
    handle: rapier::RigidBodyHandle, 
    rigid_body_set: &mut rapier::RigidBodySet,
    islands: &mut rapier::IslandManager, 
    colliders: &mut rapier::ColliderSet, 
    impulse_joints: &mut rapier::ImpulseJointSet,
    multibody_joints: &mut rapier::MultibodyJointSet
) {
    multibody_joints.remove_multibody_articulations(handle, true);
    rigid_body_set.remove(
        handle, 
        islands, 
        colliders, 
        impulse_joints, 
        multibody_joints, 
        true
    );
}
