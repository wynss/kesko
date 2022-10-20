use std::net::TcpStream;
use std::io::Write;

use bevy::prelude::*;

use kesko_core::event::SystemResponseEvent;
use kesko_physics::event::{
    PhysicResponseEvent,
    collision::CollisionEvent
};


pub(crate) fn handle_responses(
    mut tcp_stream: ResMut<TcpStream>,
    mut response_events:  EventReader<SystemResponseEvent>,
    mut physic_events:  EventReader<PhysicResponseEvent>,
    mut collision_events:  EventReader<CollisionEvent>
) {

    let mut responses: Vec<serde_traitobject::Box<dyn serde_traitobject::Any>> = Vec::new();

    for event in physic_events.iter() {
        responses.push(serde_traitobject::Box::new(event.clone()));
    }

    for event in collision_events.iter() {
        responses.push(serde_traitobject::Box::new(event.clone()));
    }

    for event in response_events.iter() {
        responses.push(serde_traitobject::Box::new(event.clone()));
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
                let _bytes_written = tcp_stream.write(response.as_bytes()).unwrap();
                tcp_stream.flush().unwrap();
            },
            Err(e) => error!("{:?}", e)
        }
    }
}
