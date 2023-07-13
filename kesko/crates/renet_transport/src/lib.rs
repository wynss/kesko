use bevy::prelude::*;
use bevy_renet::{
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport},
        ConnectionConfig, DefaultChannel, RenetClient,
    },
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};
use std::{net::UdpSocket, time::SystemTime};

fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let client = RenetClient::new(ConnectionConfig::default());

    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: 0,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    (client, transport)
}

pub struct RenetTransportPlugin;
impl Plugin for RenetTransportPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetClientPlugin);
        app.add_plugin(NetcodeClientPlugin);
        let (client, transport) = new_renet_client();
        app.insert_resource(client);
        app.insert_resource(transport);

        app.add_systems(
            (send_message_system, receive_message_system)
                .distributive_run_if(bevy_renet::transport::client_connected)
                .in_base_set(CoreSet::Update),
        );
    }

    fn name(&self) -> &str {
        "Renet Transport Plugin"
    }
}

// Systems
fn send_message_system(mut client: ResMut<RenetClient>) {
    // Send a text message to the server
    let msg = "msg from lib (client)".as_bytes().to_vec();
    client.send_message(
        DefaultChannel::ReliableOrdered,
        msg,
    );
}

fn receive_message_system(mut client: ResMut<RenetClient>) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        println!("Received message: {:?}", message);
    }
}
