use std::collections::BTreeMap;

use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::utils::hashbrown::HashMap;

use kesko_physics::{
    multibody::{
        MultibodyRoot,
        MultibodyChild,
        MultiBodyState,
        MultiBodyStates
    },
    joint::{
        JointState,
        JointMotorEvent, MotorCommand, revolute::RevoluteJoint, prismatic::PrismaticJoint,
    },
    rapier_extern::rapier::prelude as rapier
};
use serde::{Serialize, Deserialize};


pub enum SystemRequestEvent {
    GetState,
    ExitApp,
    IsAlive,
    ApplyMotorCommand {
        name: String,
        command: HashMap<String, f32>
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SystemResponseEvent {
    MultibodyStates(MultiBodyStates),
    WillExitApp,
    Alive,
    Ok(String),
    Err(String)
}

pub fn handle_system_events(
    mut system_requests: EventReader<SystemRequestEvent>,
    mut system_response_writer: EventWriter<SystemResponseEvent>,
    mut app_exit_events: EventWriter<AppExit>
) {
    for event in system_requests.iter() {
        match event {
            SystemRequestEvent::ExitApp => {
                app_exit_events.send_default();
                system_response_writer.send(SystemResponseEvent::WillExitApp);
            },
            SystemRequestEvent::IsAlive => system_response_writer.send(SystemResponseEvent::Alive),
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
        if let SystemRequestEvent::ApplyMotorCommand { name, command } = event {

            multibody_root_query.for_each(|root| {
                if root.name == *name {
                    for (joint_name, val) in command.iter() {
                        if let Some(e) = root.child_map.get(joint_name) {
                            motor_event_writer.send(JointMotorEvent {
                                entity: *e,
                                action: MotorCommand::PositionRevolute { position: *val as rapier::Real, stiffness: None, damping: None}
                            });
                        }
                    }
                }
            });
        }
    }
}

pub fn handle_serializable_state_request(
    mut system_requests: EventReader<SystemRequestEvent>,
    mut system_response_writer: EventWriter<SystemResponseEvent>,
    multibody_root_query: Query<(&MultibodyRoot, &Transform)>,
    multibody_child_query: Query<(&MultibodyChild, &Transform)>,
    revolute_joints: Query<&RevoluteJoint>,
    prismatic_joints: Query<&PrismaticJoint>
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

                    let state = if let Ok(joint) = revolute_joints.get(*e) {
                        Some(joint.state())
                    }
                    else if let Ok(joint) = prismatic_joints.get(*e) {
                        Some(joint.state())
                    }
                    else {
                        None
                    };

                    (name.clone(), state)
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

            system_response_writer.send(SystemResponseEvent::MultibodyStates(MultiBodyStates(states)));
        }
    }
}
