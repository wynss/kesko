use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub mod collision;
pub mod spawn;

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


pub enum PhysicEvent {
    PausePhysics,
    RunPhysics,
    TogglePhysics,
    DespawnBody(u64),
    DespawnAll
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
    mut event_reader: EventReader<PhysicEvent>,
    query: Query<(Entity, Option<&MultibodyRoot>), With<RigidBodyHandle>>
) {
    for event in event_reader.iter() {
        match event {
            PhysicEvent::PausePhysics => commands.insert_resource(NextState(PhysicState::Stopped)),
            PhysicEvent::RunPhysics => commands.insert_resource(NextState(PhysicState::Running)),
            PhysicEvent::TogglePhysics => {
                match current_physic_state.0 {
                    PhysicState::Stopped => commands.insert_resource(NextState(PhysicState::Running)),
                    PhysicState::Running => commands.insert_resource(NextState(PhysicState::Stopped)),
                }
            },
            PhysicEvent::DespawnBody(id) => {
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
            },
            PhysicEvent::DespawnAll => {
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
