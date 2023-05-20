use std::io::Write;
use std::net::{Shutdown, TcpListener, TcpStream};

use bevy::prelude::*;

use kesko_core::event::SimulatorResponseEvent;
use kesko_physics::event::{collision::CollisionEvent, PhysicResponseEvent};
use kesko_types::resource::KeskoRes;

pub(crate) fn handle_responses(
    mut commands: Commands,
    mut tcp_stream: ResMut<KeskoRes<TcpStream>>,
    mut response_events: EventReader<SimulatorResponseEvent>,
    mut physic_events: EventReader<PhysicResponseEvent>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    let mut should_shutdown = false;
    let mut responses: Vec<serde_traitobject::Box<dyn serde_traitobject::Any>> = Vec::new();

    for event in physic_events.iter() {
        responses.push(serde_traitobject::Box::new(event.clone()));
    }

    for event in collision_events.iter() {
        responses.push(serde_traitobject::Box::new(event.clone()));
    }

    for event in response_events.iter() {
        if let SimulatorResponseEvent::WillExitApp = event {
            should_shutdown = true
        }
        responses.push(serde_traitobject::Box::new(event.clone()));
    }

    if !responses.is_empty() {
        match serde_json::to_string_pretty(&responses) {
            Ok(json) => {
                // create http response
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n{}",
                    json.len(),
                    json
                );

                // send response
                if let Err(e) = tcp_stream.write(response.as_bytes()) {
                    error!("{:?}", e);
                    return;
                }

                if let Err(e) = tcp_stream.flush() {
                    error!("{:?}", e);
                    return;
                }
            }
            Err(e) => error!("{:?}", e),
        }
    }

    if should_shutdown {
        info!("Shuting down TCP");
        tcp_stream
            .shutdown(Shutdown::Both)
            .expect("tcp stream shutdown failed");
        commands.remove_resource::<KeskoRes<TcpListener>>();
    }
}
