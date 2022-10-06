mod response;
mod request;

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use kesko_physics::joint::JointState;
use serde_json;

use kesko_models::SpawnEvent;
use kesko_physics::{
    multibody::{MultibodyRoot, MultiBodyChild, MultiBodyState},
    joint::Joint
};
use kesko_core::event::SystemEvent;

use request::{SimHttpRequest, SimAction};
use response::Response;


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
                        .with_system(Self::handle_requests_system)
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

    fn handle_requests_system(
        mut tcp_connection_state: ResMut<State<TCPConnectionState>>,
        mut tcp_stream: ResMut<TcpStream>,
        mut tcp_buffer: ResMut<TcpBuffer>,
        mut system_event_writer: EventWriter<SystemEvent>,
        mut spawn_event_writer: EventWriter<SpawnEvent>,
        multibody_root_query: Query<(&MultibodyRoot, &Transform)>,
        multibody_child_query: Query<(&MultiBodyChild, &Transform)>,
        joint_query: Query<&Joint>
    ) {
        match tcp_stream.read(&mut tcp_buffer.data) {
            Ok(msg_len) => {

                let http_str = String::from_utf8_lossy(&tcp_buffer.data[..msg_len]).to_string();

                match SimHttpRequest::from_http_str(http_str) {
                    Ok(sim_req) => {

                        if let Some(response_content) = Self::handle_request(
                            sim_req, &mut system_event_writer, 
                            &mut spawn_event_writer, 
                            multibody_root_query,
                            multibody_child_query,
                            joint_query
                        ) {

                            // create response
                            let response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n{}",
                                response_content.len(),
                                response_content
                            );

                            // send response 
                            tcp_stream.write(response.as_bytes()).unwrap();
                            tcp_stream.flush().unwrap();
                        }
                    },
                    Err(e) => error!("{}", e)
                }
            },
            Err(e) => {
                error!("Could not read tcp stream: {}", e);

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
        spawn_event_writer: &mut EventWriter<SpawnEvent>,
        multibody_root_query: Query<(&MultibodyRoot, &Transform)>,
        multibody_child_query: Query<(&MultiBodyChild, &Transform)>,
        joint_query: Query<&Joint>
    ) -> Option<String> {

        info!("Got Request: {:?}", req.actions);

        let mut response = Response::new();

        for action in req.actions.drain(..) {
            match action {
                SimAction::Close => { 
                    system_event_writer.send(SystemEvent::Exit)
                },
                SimAction::SpawnModel { model, position: pos, color } => {
                    spawn_event_writer.send(SpawnEvent::Spawn { model, transform: Transform::from_translation(pos), color })
                }
                SimAction::GetState => {

                    let states = multibody_root_query.iter().map(|(root, transform)| {
                        
                        // get positions of all the child bodies
                        let child_positions: HashMap<String, Vec3> = root.joint_name_2_entity.iter().map(|(name, entity)| {
                            let position = match multibody_child_query.get(*entity) {
                                Ok((_, transform)) => transform.translation,
                                Err(_) => Vec3::ZERO
                            };
                            (name.clone(), position)
                        }).collect();

                        // Get joint angles
                        let joint_states: HashMap<String, Option<JointState>> = root.joint_name_2_entity.iter().map(|(name, e)| {
                            let orientation = match joint_query.get(*e) {
                                Ok(joint) => {
                                    Some(joint.get_state())
                                }
                                Err(_) => None
                            };
                            (name.clone(), orientation)
                        }).collect();

                        MultiBodyState {
                            name: root.name.clone(),
                            global_position: transform.translation,
                            global_orientation: transform.rotation.to_euler(EulerRot::XYZ).into(),
                            global_angular_velocity: root.angvel,
                            relative_positions: Some(child_positions),
                            joint_states: Some(joint_states)
                        }

                    }).collect::<Vec<MultiBodyState>>();
                    response.multibody_states = Some(states);
                },
                _ => {}
            }
        }

        Some(serde_json::to_string_pretty(&response).unwrap())
    }
}

