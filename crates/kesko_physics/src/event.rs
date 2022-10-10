use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub mod collision;

use super::PhysicState;

pub enum PhysicStateEvent {
    Pause,
    Run,
    ToggleRunPause
}

pub(crate) fn change_physic_state(
    mut commands: Commands,
    curent_physic_state: Res<CurrentState<PhysicState>>,
    mut event_reader: EventReader<PhysicStateEvent>,
) {
    for event in event_reader.iter() {
        match event {
            PhysicStateEvent::Pause => commands.insert_resource(NextState(PhysicState::Pause)),
            PhysicStateEvent::Run => commands.insert_resource(NextState(PhysicState::Run)),
            PhysicStateEvent::ToggleRunPause => {
                match curent_physic_state.0 {
                    PhysicState::Pause => commands.insert_resource(NextState(PhysicState::Run)),
                    PhysicState::Run => commands.insert_resource(NextState(PhysicState::Pause)),
                }
            }
        }
    }
}
