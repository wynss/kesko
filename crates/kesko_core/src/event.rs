use bevy::prelude::*;
use bevy::app::AppExit;
use kesko_physics::event::PhysicStateEvent;


pub enum SystemEvent {
    SpawnModel,
    PausePhysics,
    RunPhysics,
    Exit
}

pub fn handle_system_events(
    mut system_events: EventReader<SystemEvent>,
    mut app_exit_events: EventWriter<AppExit>,
    mut physics_events: EventWriter<PhysicStateEvent>,
) {
    for event in system_events.iter() {
        match event {
            SystemEvent::Exit => {
                app_exit_events.send_default();
            },
            SystemEvent::PausePhysics => physics_events.send(PhysicStateEvent::Pause),
            SystemEvent::RunPhysics => physics_events.send(PhysicStateEvent::Run),
            _ => {}
        }
    }
}
