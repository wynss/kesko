mod response;
mod request;

use std::net::TcpListener;

use bevy::prelude::*;
use iyes_loopless::prelude::*;


const URL: &str = "127.0.0.1:8080";

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum TcpConnectionState {
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
pub struct TcpPlugin;
impl Plugin for TcpPlugin {
    fn build(&self, app: &mut App) {

        static TCP_REQUEST_STAGE: &str = "tcp_request_stage";
        static TCP_RESPONSE_STAGE: &str = "tcp_response_stage";

        match TcpListener::bind(URL) {
            Ok(listener) => {
                app.add_loopless_state(TcpConnectionState::NotConnected)
                    .insert_resource(listener)
                    .insert_resource(TcpBuffer::new())

                    .add_stage_before(CoreStage::First, TCP_REQUEST_STAGE, SystemStage::single_threaded())
                    .add_stage_after(CoreStage::Last, TCP_RESPONSE_STAGE, SystemStage::single_threaded())

                    .add_system_to_stage(
                        TCP_REQUEST_STAGE,
                        handle_incoming_connections.run_in_state(TcpConnectionState::NotConnected)
                    )
                    .add_system_to_stage(
                        TCP_REQUEST_STAGE,
                        request::handle_requests.run_in_state(TcpConnectionState::Connected)
                    )
                    .add_system_to_stage(
                        TCP_RESPONSE_STAGE,
                        response::handle_responses.run_in_state(TcpConnectionState::Connected)
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

pub(crate) fn handle_incoming_connections(
    mut commands: Commands,
    listener: Res<TcpListener>
) {
    info!("Waiting for TCP connection...");
    match listener.accept() {
        Ok((stream, _)) => {
            let ip = match stream.peer_addr() {
                Ok(ip) => ip.ip().to_string(),
                Err(_) => "Unknown".to_owned()
            };

            info!("TCP connection established with {}!", ip);

            commands.insert_resource(NextState(TcpConnectionState::Connected));
            commands.insert_resource(stream);
        },
        Err(e) => error!("{}", e)
    }
}
