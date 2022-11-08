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


pub enum SimulatorRequestEvent {
    GetState,
    ExitApp,
    IsAlive,
    ApplyMotorCommand {
        entity: Entity,
        command: HashMap<u64, f32>
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SimulatorResponseEvent {
    MultibodyStates(MultiBodyStates),
    WillExitApp,
    Alive,
    Ok(String),
    Err(String)
}

pub fn handle_system_events(
    mut system_requests: EventReader<SimulatorRequestEvent>,
    mut system_response_writer: EventWriter<SimulatorResponseEvent>,
    mut app_exit_events: EventWriter<AppExit>
) {
    for event in system_requests.iter() {
        match event {
            SimulatorRequestEvent::ExitApp => {
                app_exit_events.send_default();
                system_response_writer.send(SimulatorResponseEvent::WillExitApp);
            },
            SimulatorRequestEvent::IsAlive => system_response_writer.send(SimulatorResponseEvent::Alive),
            _ => {}
        }
    }
}

pub fn handle_motor_command_requests(
    mut system_requests: EventReader<SimulatorRequestEvent>,
    mut motor_event_writer: EventWriter<JointMotorEvent>,
) {
    for event in system_requests.iter() {
        if let SimulatorRequestEvent::ApplyMotorCommand { entity: _, command } = event {
            for (joint_id, val) in command.iter() {
                motor_event_writer.send(JointMotorEvent {
                    entity: Entity::from_bits(*joint_id),
                    command: MotorCommand::PositionRevolute { position: *val as rapier::Real, stiffness: None, damping: None}
                });
            }
        }
    }
}

pub fn handle_serializable_state_request(
    mut system_requests: EventReader<SimulatorRequestEvent>,
    mut system_response_writer: EventWriter<SimulatorResponseEvent>,
    multibody_root_query: Query<(Entity, &MultibodyRoot, &Transform)>,
    multibody_child_query: Query<(&MultibodyChild, &Transform)>,
    revolute_joints: Query<&RevoluteJoint>,
    prismatic_joints: Query<&PrismaticJoint>
) {

    for event in system_requests.iter() {
        if let SimulatorRequestEvent::GetState = event {
            let states = multibody_root_query.iter().map(|(e, root, transform)| {
                
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
                    id: e.to_bits(),
                    position: transform.translation,
                    orientation: transform.rotation,
                    velocity: root.linvel,
                    angular_velocity: root.angvel,
                    relative_positions: Some(child_positions),
                    joint_states: Some(joint_states)
                }

            }).collect::<Vec<MultiBodyState>>();

            system_response_writer.send(SimulatorResponseEvent::MultibodyStates(MultiBodyStates(states)));
        }
    }
}
