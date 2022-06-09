use bevy::prelude::*;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum TCPConnectionState {
    NotConnected,
    Connected
}

pub struct TCPPlugin;

impl Plugin for TCPPlugin {
    fn build(&self, app: &mut App) {

        let listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();

        app.add_state(TCPConnectionState::NotConnected)
        .insert_resource(listener)
        .add_system_set(
            SystemSet::on_update(TCPConnectionState::NotConnected)
                .with_system(Self::handle_incoming_system)
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
        listener: Res<std::net::TcpListener>
    ) {

        info!("Waiting for connection from python");
        match listener.accept() {
            Ok((stream, _)) => {
                info!("Connection established");
                commands.insert_resource(stream);
                tcp_connection_state.set(TCPConnectionState::Connected).unwrap();
            },
            Err(e) => error!("{:?}", e)
        }

    }
}
