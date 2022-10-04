mod response;
mod request;

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

use bevy::prelude::*;
use kesko_models::SpawnEvent;
use request::{SimHttpRequest, SimAction};

use kesko_core::event::SystemEvent;


const URL: &str = "127.0.0.1:8080";

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum TCPConnectionState {
    NotConnected,
    Connected
}

struct TcpBuffer{
    data: [u8; 2048]
}
impl TcpBuffer {
    fn new() -> Self {
        Self {
            data: [0; 2048]
        }
    }
}

/// Plugin for adding a tcp server that will progress the simulation only when getting a step request.
/// It is mainly used for the python API
pub struct TCPPlugin;
impl Plugin for TCPPlugin {
    fn build(&self, app: &mut App) {

        match TcpListener::bind(URL) {
            Ok(listener) => {
                app.add_state(TCPConnectionState::NotConnected)
                .insert_resource(listener)
                .insert_resource(TcpBuffer::new())
                .add_system_set(
                    SystemSet::on_update(TCPConnectionState::NotConnected)
                        .with_system(Self::handle_incoming_system)
                )
                .add_system_set(
                    SystemSet::on_update(TCPConnectionState::Connected)
                        .with_system(Self::communication_system)
                );
            },
            Err(e) => {
                error!("{}", e)
            }
        } 
    }

    fn name(&self) -> &str {
        "TCP Plugin"
    }
}

impl TCPPlugin {

    fn handle_incoming_system(
        mut commands: Commands,
        mut tcp_connection_state: ResMut<State<TCPConnectionState>>,
        listener: Res<TcpListener>
    ) {
        info!("Waiting for TCP connection...");
        match listener.accept() {
            Ok((stream, _)) => {
                info!("TCP connection established!");
                commands.insert_resource(stream);
                tcp_connection_state.set(TCPConnectionState::Connected).unwrap();
            },
            Err(e) => error!("{:?}", e)
        }
    }

    fn communication_system(
        mut tcp_connection_state: ResMut<State<TCPConnectionState>>,
        mut tcp_stream: ResMut<TcpStream>,
        mut tcp_buffer: ResMut<TcpBuffer>,
        mut system_event_writer: EventWriter<SystemEvent>,
        mut spawn_event_writer: EventWriter<SpawnEvent>
    ) {
        match tcp_stream.read(&mut tcp_buffer.data) {
            Ok(msg_len) => {

                let http_str = String::from_utf8_lossy(&tcp_buffer.data[..msg_len]).to_string();

                match SimHttpRequest::from_http_request(http_str) {
                    Ok(sim_req) => {

                        Self::handle_request(sim_req, &mut system_event_writer, &mut spawn_event_writer);

                        // create response
                        let contents = "Hello from Nora";
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                            contents.len(),
                            contents
                        );

                        // send response 
                        tcp_stream.write(response.as_bytes()).unwrap();
                        tcp_stream.flush().unwrap();
                    },
                    Err(e) => error!("{}", e)
                }
            },
            Err(e) => {
                error!("{}", e);

                // set state to not connected
                if let Err(e) = tcp_connection_state.set(TCPConnectionState::NotConnected) {
                    error!("{}", e);
                }
            }
        }
    }

    fn handle_request(
        mut req: SimHttpRequest, 
        system_event_writer: &mut EventWriter<SystemEvent>,
        spawn_event_writer: &mut EventWriter<SpawnEvent>
    ) {
        info!("Got Request: {:?}", req.actions);

        for action in req.actions.drain(..) {
            match action {
                SimAction::Close => { 
                    system_event_writer.send(SystemEvent::Exit)
                },
                SimAction::SpawnModel { model, position: pos, color } => {
                    info!("{:?}", color);
                    spawn_event_writer.send(SpawnEvent::Spawn { model, transform: Transform::from_translation(pos), color })
                }
                _ => {}
            }
        }
    }
}
