use bevy::prelude::*;

use nora_object_interaction::event::{InteractionEvent, SelectEvent};
use nora_physics::{event::CollisionEvent, joint::JointMotorEvent};
use crate::interaction::multibody_selection::MultibodySelectionEvent;


pub struct DebugEventPlugin;
impl Plugin for DebugEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(debug_events);
    }
}

pub fn debug_events(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut select_event_reader: EventReader<SelectEvent>,
    mut interaction_event_reader: EventReader<InteractionEvent>,
    mut joint_event_reader: EventReader<JointMotorEvent>,
    mut multibody_selection_event_reader: EventReader<MultibodySelectionEvent>
) {
    for interaction_event in interaction_event_reader.iter() {
        info!("Interaction event {:?}", interaction_event);
    }
    for select_event in select_event_reader.iter() {
        info!("Select event {:?}", select_event);
    }
    for collision_event in collision_event_reader.iter() {
        info!("Collision event {:?}", collision_event);
    }
    for joint_event in joint_event_reader.iter() {
        info!("Joint event {:?}", joint_event);
    }
    for multibody_selection_event in multibody_selection_event_reader.iter() {
        info!("Multibody Selection event {:?}", multibody_selection_event);
    }
}