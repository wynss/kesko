use std::collections::BTreeMap;

use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::utils::hashbrown::HashMap;

use kesko_physics::{
    event::{
        PhysicEvent,
    },
    multibody::{
        MultibodyRoot,
        MultibodyChild,
        MultiBodyState,
        MultiBodyStates
    },
    joint::{
        Joint,
        JointState,
        JointMotorEvent, MotorAction,
    }
};


pub enum SystemRequestEvent {
    SpawnModel,
    Despawn {
        name: String
    },
    PausePhysics,
    StartPhysics,
    GetState,
    ExitApp,
    IsAlive,
    ApplyMotorCommand {
        body_name: String,
        command: HashMap<String, f32>
    }
}

pub enum SystemResponseEvent {
    State(MultiBodyStates),
    SpawnedModel(String),
    PausedPhysics,
    StartedPhysics,
    WillExitApp,
    Alive,
    Ok(String),
    Err(String)
}

pub fn handle_system_events(
    mut system_requests: EventReader<SystemRequestEvent>,
    mut system_response_writer: EventWriter<SystemResponseEvent>,
    mut app_exit_events: EventWriter<AppExit>,
    mut physics_events: EventWriter<PhysicEvent>,
) {
    for event in system_requests.iter() {
        match event {
            SystemRequestEvent::ExitApp => {
                app_exit_events.send_default();
                system_response_writer.send(SystemResponseEvent::WillExitApp);
            },
            SystemRequestEvent::PausePhysics => {
                physics_events.send(PhysicEvent::PausePhysics);
                system_response_writer.send(SystemResponseEvent::PausedPhysics);
            },
            SystemRequestEvent::StartPhysics => {
                physics_events.send(PhysicEvent::RunPhysics);
                system_response_writer.send(SystemResponseEvent::StartedPhysics);
            },
            SystemRequestEvent::IsAlive => system_response_writer.send(SystemResponseEvent::Alive),
            SystemRequestEvent::Despawn{ name } => {
                physics_events.send(PhysicEvent::DespawnMultibody{ name: name.clone() });
                system_response_writer.send(SystemResponseEvent::Ok(format!("Despawned: {}", name)));
            },
            _ => {}
        }
    }
}

pub fn handle_motor_command_requests(
    mut system_requests: EventReader<SystemRequestEvent>,
    mut motor_event_writer: EventWriter<JointMotorEvent>,
    multibody_root_query: Query<&MultibodyRoot>,
) {
    for event in system_requests.iter() {
        if let SystemRequestEvent::ApplyMotorCommand { body_name, command } = event {
            for root in multibody_root_query.iter() {
                if root.name == *body_name {
                    for (joint_name, val) in command.iter() {
                        if let Some(e) = root.child_map.get(joint_name) {
                            motor_event_writer.send(JointMotorEvent {
                                entity: *e,
                                action: MotorAction::PositionRevolute { position: *val, damping: 0.0, stiffness: 1.0 }
                            });
                        }
                    }
                }
            }
        }
    }
}

pub fn handle_serializable_state_request(
    mut system_requests: EventReader<SystemRequestEvent>,
    mut system_response_writer: EventWriter<SystemResponseEvent>,
    multibody_root_query: Query<(&MultibodyRoot, &Transform)>,
    multibody_child_query: Query<(&MultibodyChild, &Transform)>,
    joint_query: Query<&Joint>
) {

    for event in system_requests.iter() {
        if let SystemRequestEvent::GetState = event {
            let states = multibody_root_query.iter().map(|(root, transform)| {
                
                // get positions of all the child bodies
                let child_positions: BTreeMap<String, Vec3> = root.child_map.iter().map(|(name, entity)| {
                    let position = match multibody_child_query.get(*entity) {
                        Ok((_, transform)) => transform.translation,
                        Err(_) => Vec3::ZERO
                    };
                    (name.clone(), position)
                }).collect();

                // Get joint angles
                let joint_states: BTreeMap<String, Option<JointState>> = root.child_map.iter().map(|(name, e)| {
                    let orientation = match joint_query.get(*e) {
                        Ok(joint) => {
                            Some(joint.get_state())
                        }
                        Err(_) => None
                    };
                    (name.clone(), orientation)
                }).collect();

                MultiBodyState {
                    name: root.name.clone(),
                    global_position: transform.translation,
                    global_orientation: transform.rotation.to_euler(EulerRot::XYZ).into(),
                    global_angular_velocity: root.angvel,
                    relative_positions: Some(child_positions),
                    joint_states: Some(joint_states)
                }

            }).collect::<Vec<MultiBodyState>>();

            system_response_writer.send(SystemResponseEvent::State(MultiBodyStates { multibody_states: states }));
        }
    }
}
