use std::net::TcpStream;
use std::io::Write;

use bevy::prelude::*;

use kesko_core::event::SystemResponseEvent;
use kesko_physics::event::PhysicResponseEvent;


pub(crate) fn handle_responses(
    mut tcp_stream: ResMut<TcpStream>,
    mut response_events:  EventReader<SystemResponseEvent>,
    mut spawn_event:  EventReader<PhysicResponseEvent>
) {

    let mut responses: Vec<serde_traitobject::Box<dyn serde_traitobject::Any>> = Vec::new();

    for event in spawn_event.iter() {
        responses.push(serde_traitobject::Box::new(event.clone()));
    }

    for event in response_events.iter() {
        match event {
            SystemResponseEvent::State(states) => {
                responses.push(serde_traitobject::Box::new(states.clone()));
            },
            SystemResponseEvent::Alive => {
                responses.push(serde_traitobject::Box::new("alive".to_owned()));
            }
            SystemResponseEvent::Ok(msg) => {
                responses.push(serde_traitobject::Box::new(msg.clone()));
            }
            _ => {
                responses.push(serde_traitobject::Box::new("OK".to_owned()));
            }
        }
    }

    if !responses.is_empty() {
        match serde_json::to_string_pretty(&responses) {
            Ok(json) =>  {
                // create http response
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n{}",
                    json.len(),
                    json
                );

                // send response 
                tcp_stream.write(response.as_bytes()).unwrap();
                tcp_stream.flush().unwrap();
            },
            Err(e) => error!("{:?}", e)
        }
    }
}
