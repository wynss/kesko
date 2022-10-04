use bevy::prelude::*;
use bevy::app::AppExit;


pub enum SystemEvent {
    Exit = 0
}

pub fn handle_system_events(
    mut system_events: EventReader<SystemEvent>,
    mut app_exit_events: EventWriter<AppExit>
) {
    for event in system_events.iter() {
        match event {
            SystemEvent::Exit => {
                app_exit_events.send_default();
            }
        }
    }
}
