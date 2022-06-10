mod response;
mod request;

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

use bevy::prelude::*;
use request::{SimRequest, SimAction, HttpRequest};

use kesko_core::event::SystemEvent;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum TCPConnectionState {
    NotConnected,
    Connected
}

pub struct TCPPlugin;

impl Plugin for TCPPlugin {
    fn build(&self, app: &mut App) {

        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        app.add_state(TCPConnectionState::NotConnected)
        .insert_resource(listener)
        .add_system_set(
            SystemSet::on_update(TCPConnectionState::NotConnected)
                .with_system(Self::handle_incoming_system)
        )
        .add_system_set(
            SystemSet::on_update(TCPConnectionState::Connected)
                .with_system(Self::communication_system)
        );
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
        info!("Waiting for connection...");
        match listener.accept() {
            Ok((stream, _)) => {
                info!("Connection established!");
                commands.insert_resource(stream);
                tcp_connection_state.set(TCPConnectionState::Connected).unwrap();
            },
            Err(e) => error!("{:?}", e)
        }
    }

    fn communication_system(
        mut tcp_connection_state: ResMut<State<TCPConnectionState>>,
        mut tcp_stream: ResMut<TcpStream>,
        mut system_event_writer: EventWriter<SystemEvent>
    ) {

        let mut buffer = [0; 1024];

        match tcp_stream.read(&mut buffer) {
            Ok(msg_len) => {

                let http_str = String::from_utf8_lossy(&buffer[..msg_len]).to_string();

                info!("Got http string: {}", http_str);

                if let Ok(http_req) = HttpRequest::parse(http_str) {
                    
                    let contents = "Hello from Nora";
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        contents.len(),
                        contents
                    );
                    tcp_stream.write(response.as_bytes()).unwrap();
                    tcp_stream.flush().unwrap();

                    if let Some(sim_req) = SimRequest::from_http_request(&http_req) {
                        Self::handle_request(sim_req, &mut system_event_writer);
                    }
                }
            },
            Err(e) => {
                error!("{}", e);
                if let Err(e) = tcp_connection_state.set(TCPConnectionState::NotConnected) {
                    error!("{}", e);
                }
            }
        }
    }

    fn handle_request(req: SimRequest, system_event_writer: &mut EventWriter<SystemEvent>) {

        info!("Got Request: {:?}", req.action);
        
        match req.action {
            SimAction::Close => { 
                system_event_writer.send(SystemEvent::Exit)
            },
            _ => {}
        }

    }
}
