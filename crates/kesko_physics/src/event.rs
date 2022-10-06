use bevy::prelude::*;

pub mod collision;

use super::PhysicState;

pub enum PhysicStateEvent {
    Pause,
    Run,
    ToggleRunPause
}

pub(crate) fn change_physic_state(
    mut event_reader: EventReader<PhysicStateEvent>,
    mut physic_state: ResMut<State<PhysicState>>,
) {
    for event in event_reader.iter() {
        match event {
            PhysicStateEvent::Pause => set_state(PhysicState::Pause, &mut physic_state),
            PhysicStateEvent::Run => set_state(PhysicState::Run, &mut physic_state),
            PhysicStateEvent::ToggleRunPause => {
                match physic_state.current() {
                    PhysicState::Pause => set_state(PhysicState::Run, &mut physic_state),
                    PhysicState::Run => set_state(PhysicState::Pause, &mut physic_state)
                }
            }
        }
    }
}

fn set_state(state: PhysicState, current_state: &mut State<PhysicState>) {
    match current_state.set(state) {
        Ok(_) => {},
        Err(e) => error!("{}", e)
    }
}
