use bevy::prelude::*;

use kesko_object_interaction::event::{InteractionEvent, SelectEvent};
use kesko_physics::{
    event::collision::CollisionEvent, 
    joint::JointMotorEvent
};
use kesko_core::interaction::multibody_selection::MultibodySelectionEvent;


pub struct DebugEventPlugin;
impl Plugin for DebugEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(log_multibody_selection_event)
            .add_system(log_joint_events)
            .add_system(log_select_events)
            .add_system(log_interaction_events)
            .add_system(log_collision_events);
    }
}

pub fn log_multibody_selection_event(
    mut multibody_selection_event_reader: EventReader<MultibodySelectionEvent>
) {
    for multibody_selection_event in multibody_selection_event_reader.iter() {
        info!("Multibody Selection event {:?}", multibody_selection_event);
    }
}

pub fn log_joint_events(
    mut joint_event_reader: EventReader<JointMotorEvent>,
) {
    for joint_event in joint_event_reader.iter() {
        info!("Joint event {:?}", joint_event);
    }
}

pub fn log_select_events(
    mut select_event_reader: EventReader<SelectEvent>,
) {
    for select_event in select_event_reader.iter() {
        info!("Select event {:?}", select_event);
    }
}

pub fn log_interaction_events(
    mut interaction_event_reader: EventReader<InteractionEvent>,
) {
    for interaction_event in interaction_event_reader.iter() {
        info!("Interaction event {:?}", interaction_event);
    }
}

pub fn log_collision_events(
    mut collision_event_reader: EventReader<CollisionEvent>,
) {
    for collision_event in collision_event_reader.iter() {
        info!("Collision event {:?}", collision_event);
    }
}