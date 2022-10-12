use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub mod collision;
pub mod spawn;

use super::{
    PhysicState,
    multibody::MultibodyRoot
};


pub enum PhysicEvent {
    PausePhysics,
    RunPhysics,
    TogglePhysics,
    DespawnMultibody {
        name: String
    }
}

pub(crate) fn handle_events(
    mut commands: Commands,
    current_physic_state: Res<CurrentState<PhysicState>>,
    mut event_reader: EventReader<PhysicEvent>,
    query: Query<(Entity, &MultibodyRoot)>
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
            PhysicEvent::DespawnMultibody{ name } => {
                for (entity, root) in query.iter() {
                    if root.name == *name {
                        commands.entity(entity).despawn();
                        for child in root.child_map.values() {
                            commands.entity(*child).despawn();
                        }
                    }
                }
            }
        }
    }
}
