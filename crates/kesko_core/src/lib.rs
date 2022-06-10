pub mod orbit_camera;
pub mod cursor_tracking;
pub mod bundle;
pub mod shape;
pub mod transform;
pub mod interaction;
pub mod controller;
pub mod event;

use bevy::prelude::*;

use kesko_physics::PhysicState;


pub fn change_physic_state(
    mut keys: ResMut<Input<KeyCode>>,
    mut physic_state: ResMut<State<PhysicState>>,
) {
    if keys.just_pressed(KeyCode::Space) {

        match physic_state.current() {
            PhysicState::Pause => physic_state.set(PhysicState::Run).unwrap(),
            PhysicState::Run => physic_state.set(PhysicState::Pause).unwrap(),
        }
        keys.reset(KeyCode::Space);
    }
}
