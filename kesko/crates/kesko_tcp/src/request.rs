use std::io::Read;
use std::net::TcpStream;

use bevy::{prelude::*, utils::hashbrown::HashMap};
use serde::{Deserialize, Serialize};

use kesko_core::event::SimulatorRequestEvent;
use kesko_models::{Model, SpawnEvent};
use kesko_physics::event::PhysicRequestEvent;
use kesko_types::resource::KeskoRes;

use super::TcpBuffer;

/// Commands that can be requested by http clients
#[derive(Deserialize, Serialize, Debug)]
pub(crate) enum TcpCommand {
    Close,
    GetState,
    SpawnModel {
        model: Model,
        position: Vec3,
        color: Color,
        rotation: Vec3,
        scale: Vec3,
    },
    Despawn {
        id: u64,
    },
    DespawnAll,
    ApplyMotorCommand {
        id: u64,
        command: HashMap<u64, f32>,
    },
    PublishFlatBuffers {
        data: String,
    },
    PausePhysics,
    RunPhysics,
    IsAlive,
}

/// Holds parsed http requests
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct HttpRequest {
    pub(crate) commands: Vec<TcpCommand>,
}

impl HttpRequest {
    pub(crate) fn parse(request_str: String) -> Result<String, String> {
        for line in request_str.lines() {
            if line.starts_with('{') {
                return Ok(line.to_owned());
            }
        }
        Err("Failed to parse http request".to_owned())
    }

    pub(crate) fn from_http_str(req: String) -> Result<HttpRequest, String> {
        match Self::parse(req) {
            Ok(json) => match serde_json::from_str::<HttpRequest>(json.as_str()) {
                Ok(req) => Ok(req),
                Err(e) => Err(format!(
                    "Failed to convert http request to SimHttpRequest: {}",
                    e
                )),
            },
            Err(e) => Err(e),
        }
    }
}

pub(crate) fn handle_requests(
    mut tcp_stream: ResMut<KeskoRes<TcpStream>>,
    mut tcp_buffer: ResMut<KeskoRes<TcpBuffer>>,
    mut system_event_writer: EventWriter<SimulatorRequestEvent>,
    mut spawn_event_writer: EventWriter<SpawnEvent>,
    mut physic_event_writer: EventWriter<PhysicRequestEvent>,
) {
    info!("Waiting for request...");
    let mut got_msg = false;
    while !got_msg {
        match tcp_stream.read(&mut tcp_buffer.data) {
            Ok(msg_len) => {
                if msg_len == 0 {
                    return;
                }

                got_msg = true;

                let http_str = String::from_utf8_lossy(&tcp_buffer.data[..msg_len]).to_string();
                match HttpRequest::from_http_str(http_str) {
                    Ok(mut request) => {
                        info!("Got Request: {:?}", request.commands);

                        for command in request.commands.drain(..) {
                            match command {
                                TcpCommand::Close => {
                                    system_event_writer.send(SimulatorRequestEvent::ExitApp)
                                }
                                TcpCommand::SpawnModel {
                                    model,
                                    position,
                                    color,
                                    rotation,
                                    scale,
                                } => {
                                    spawn_event_writer.send(SpawnEvent::Spawn {
                                        model,
                                        transform: Transform::from_translation(position)
                                            .with_rotation(Quat::from_euler(
                                                EulerRot::XYZ,
                                                rotation.x,
                                                rotation.y,
                                                rotation.z,
                                            ))
                                            .with_scale(scale),
                                        color,
                                    });
                                }
                                TcpCommand::GetState => {
                                    system_event_writer.send(SimulatorRequestEvent::GetState)
                                }
                                TcpCommand::PausePhysics => {
                                    physic_event_writer.send(PhysicRequestEvent::PausePhysics)
                                }
                                TcpCommand::RunPhysics => {
                                    physic_event_writer.send(PhysicRequestEvent::RunPhysics)
                                }
                                TcpCommand::IsAlive => {
                                    system_event_writer.send(SimulatorRequestEvent::IsAlive)
                                }
                                TcpCommand::ApplyMotorCommand { id, command } => {
                                    system_event_writer.send(
                                        SimulatorRequestEvent::ApplyMotorCommand {
                                            entity: Entity::from_bits(id),
                                            command,
                                        },
                                    )
                                }
                                TcpCommand::PublishFlatBuffers { data } => {
                                    // TODO skip converting to base64 and just send the bytes
                                    let decoded = base64::decode(data).expect("Invalid base64 string");
                                    system_event_writer
                                        .send(SimulatorRequestEvent::PublishFlatBuffers(decoded))
                                }
                                TcpCommand::Despawn { id } => {
                                    physic_event_writer.send(PhysicRequestEvent::DespawnBody(id))
                                }
                                TcpCommand::DespawnAll => {
                                    physic_event_writer.send(PhysicRequestEvent::DespawnAll)
                                }
                            }
                        }
                    }
                    Err(e) => {
                        got_msg = false;
                        error!("{}", e)
                    }
                }
            }
            Err(e) => {
                error!("Could not read tcp stream: {}", e);
                system_event_writer.send(SimulatorRequestEvent::ExitApp);
                return;
            }
        }
    }
}
