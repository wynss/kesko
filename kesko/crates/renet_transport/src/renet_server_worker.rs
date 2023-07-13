use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    sync::mpsc::{self, Receiver, Sender, TryRecvError},
    thread::{self, JoinHandle},
    time::{Duration, Instant, SystemTime},
};

use renet::{
    transport::{
        NetcodeServerTransport, ServerAuthentication,
        ServerConfig,
    },
    ConnectionConfig, DefaultChannel, RenetServer, ServerEvent,
};

pub struct RenetServerWorker {
    sender_to_worker: Sender<String>,
    receiver_from_worker: Receiver<String>,
    handle: Option<JoinHandle<()>>,
}

impl RenetServerWorker {
    pub fn new() -> Self {
        let public_addr: SocketAddr = "127.0.0.1:5000".parse().unwrap();

        let (main_send, worker_receive) = mpsc::channel();
        let (worker_send, main_receive) = mpsc::channel();

        let handle = thread::spawn(move || {
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
            let mut transport =
                NetcodeServerTransport::new(current_time, server_config, socket).unwrap();

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

                // Handle connection events
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

                // Receive messages
                for client_id in server.clients_id() {
                    while let Some(message) =
                        server.receive_message(client_id, DefaultChannel::ReliableOrdered)
                    {
                        // println!("Received message from client {}: {:?}", client_id, message);
                        let text = String::from_utf8(message.into()).unwrap();
                        // println!("Client ({}) sent text: {}", client_id, text);
                        let text = format!("{}", text);
                        worker_send.send(text).unwrap();
                    }
                }

                match worker_receive.try_recv() {
                    Ok(message) => {
                        server.broadcast_message(
                            DefaultChannel::ReliableOrdered,
                            format!("response to {}", message).as_bytes().to_vec(),
                        );
                    }
                    Err(TryRecvError::Empty) => {
                        continue;
                    }
                    Err(TryRecvError::Disconnected) => {
                        break;
                    }
                }

                transport.send_packets(&mut server);
                thread::sleep(Duration::from_millis(50));
            }
        });

        RenetServerWorker {
            sender_to_worker: main_send,
            receiver_from_worker: main_receive,
            handle: Some(handle),
        }
    }

    pub fn send(&self, message: String) {
        self.sender_to_worker.send(message).unwrap();
    }

    pub fn try_receive(&self) -> Option<String> {
        match self.receiver_from_worker.try_recv() {
            Ok(message) => {
                println!("Main thread received: {}", message);
                Some(message)
            }
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => None,
        }
    }

    pub fn join(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}