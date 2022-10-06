pub mod orbit_camera;
pub mod cursor_tracking;
pub mod bundle;
pub mod shape;
pub mod transform;
pub mod interaction;
pub mod controller;
pub mod event;

use bevy::prelude::*;

use kesko_physics::event::PhysicStateEvent;


pub fn change_physic_state(
    mut keys: ResMut<Input<KeyCode>>,
    mut event_writer: EventWriter<PhysicStateEvent>
) {
    if keys.just_pressed(KeyCode::Space) {
        event_writer.send(PhysicStateEvent::ToggleRunPause);
        keys.reset(KeyCode::Space);
    }
}
