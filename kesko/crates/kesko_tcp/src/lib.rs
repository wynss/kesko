mod request;
mod response;

use std::net::TcpListener;

use bevy::prelude::*;

use kesko_types::resource::KeskoRes;

const URL: &str = "127.0.0.1:8080";

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum TcpConnectionState {
    #[default]
    NotConnected,
    Connected,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, SystemSet)]
#[system_set(base)]
enum TcpSet {
    Request,
    Response,
}

struct TcpBuffer {
    data: [u8; 2048],
}
impl TcpBuffer {
    fn new() -> Self {
        Self { data: [0; 2048] }
    }
}

/// Plugin for adding a tcp server that will progress the simulation only when getting a step request.
/// It is mainly used for the python API
pub struct TcpPlugin;
impl Plugin for TcpPlugin {
    fn build(&self, app: &mut App) {
        match TcpListener::bind(URL) {
            Ok(listener) => {
                app.add_state::<TcpConnectionState>()
                    .insert_resource(KeskoRes(listener))
                    .insert_resource(KeskoRes(TcpBuffer::new()))
                    .configure_sets(
                        (
                            TcpSet::Request,
                            CoreSet::First,
                            CoreSet::LastFlush,
                            TcpSet::Response,
                        )
                            .chain(),
                    )
                    .add_system(apply_system_buffers.in_base_set(TcpSet::Request))
                    .add_system(apply_system_buffers.in_base_set(TcpSet::Response))
                    .add_system(
                        handle_incoming_connections
                            .run_if(in_state(TcpConnectionState::NotConnected))
                            .in_base_set(TcpSet::Request),
                    )
                    .add_system(
                        request::handle_requests
                            .run_if(in_state(TcpConnectionState::Connected))
                            .in_base_set(TcpSet::Request),
                    )
                    .add_system(
                        response::handle_responses
                            .run_if(in_state(TcpConnectionState::Connected))
                            .in_base_set(TcpSet::Response),
                    );
            }
            Err(e) => {
                error!("{}", e)
            }
        }
    }

    fn name(&self) -> &str {
        "TCP Plugin"
    }
}

pub(crate) fn handle_incoming_connections(
    mut next_connection_state: ResMut<NextState<TcpConnectionState>>,
    mut commands: Commands,
    listener: Res<KeskoRes<TcpListener>>,
) {
    info!("Waiting for TCP connection...");
    match listener.accept() {
        Ok((stream, _)) => {
            let ip = match stream.peer_addr() {
                Ok(ip) => ip.ip().to_string(),
                Err(_) => "Unknown".to_owned(),
            };

            info!("TCP connection established with {}!", ip);

            next_connection_state.set(TcpConnectionState::Connected);
            commands.insert_resource(KeskoRes(stream));
        }
        Err(e) => error!("{}", e),
    }
}
