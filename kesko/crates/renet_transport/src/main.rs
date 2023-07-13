use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    sync::mpsc::{self, Receiver, TryRecvError},
    thread,
    time::{Duration, Instant, SystemTime},
};

use renet::{
    transport::{
        ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication,
        ServerConfig, NETCODE_USER_DATA_BYTES,
    },
    ConnectionConfig, DefaultChannel, RenetClient, RenetServer, ServerEvent,
};

fn main() {
    let server_addr: SocketAddr = "127.0.0.1:5000".parse().unwrap();
    server(server_addr);
}

fn server(public_addr: SocketAddr) {
    let connection_config = ConnectionConfig::default();
    let mut server: RenetServer = RenetServer::new(connection_config);

    let socket: UdpSocket = UdpSocket::bind(public_addr).unwrap();
    let server_config = ServerConfig {
        max_clients: 64,
        protocol_id: 0,
        public_addr,
        authentication: ServerAuthentication::Unsecure,
    };
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let mut transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();

    let mut usernames: HashMap<u64, String> = HashMap::new();
    let mut received_messages: Vec<String> = vec![];
    let mut last_updated = Instant::now();

    loop {
        let now = Instant::now();
        let duration = now - last_updated;
        last_updated = now;

        server.update(duration);
        transport.update(duration, &mut server).unwrap();

        received_messages.clear();

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    let user_data = transport.user_data(client_id).unwrap();
                    println!("user_data: {:?}", user_data);
                    // let username = Username::from_user_data(&user_data);
                    // usernames.insert(client_id, username.0);
                    println!("Client {} connected.", client_id)
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!("Client {} disconnected: {}", client_id, reason);
                    usernames.remove_entry(&client_id);
                }
            }
        }

        for client_id in server.clients_id() {
            while let Some(message) =
                server.receive_message(client_id, DefaultChannel::ReliableOrdered)
            {
                // println!("Received message from client {}: {:?}", client_id, message);
                let text = String::from_utf8(message.into()).unwrap();
                // println!("Client ({}) sent text: {}", client_id, text);
                let text = format!("{}", text);
                received_messages.push(text);
            }
        }

        for text in received_messages.iter() {
            server.broadcast_message(
                DefaultChannel::ReliableOrdered,
                format!("response to {}", text).as_bytes().to_vec(),
            );
        }

        transport.send_packets(&mut server);
        thread::sleep(Duration::from_millis(50));
    }
}