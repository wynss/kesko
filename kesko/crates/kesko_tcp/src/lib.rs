mod request;
mod response;

use std::net::TcpListener;

use bevy::prelude::*;

use kesko_types::resource::KeskoRes;
use kesko_core::HandleEventsSet;

const URL: &str = "127.0.0.1:8080";

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum TcpConnectionState {
    #[default]
    NotConnected,
    Connected,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, SystemSet)]
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
                    .configure_set(First, TcpSet::Request)
                    .configure_set(Last, TcpSet::Response.after(HandleEventsSet))
                    .add_systems(
                        First,
                        (handle_incoming_connections, apply_deferred)
                            .chain()
                            .run_if(in_state(TcpConnectionState::NotConnected))
                            .in_set(TcpSet::Request),
                    )
                    .add_systems(
                        First,
                        request::handle_requests
                            .run_if(in_state(TcpConnectionState::Connected))
                            .in_set(TcpSet::Request),
                    )
                    .add_systems(
                        Last,
                        response::handle_responses
                            .run_if(in_state(TcpConnectionState::Connected))
                            .in_set(TcpSet::Response),
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
