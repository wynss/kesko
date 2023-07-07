use std::collections::HashMap;

use bevy::prelude::*;
use kesko_core::event::SimulatorRequestEvent;
extern crate flatbuffers;

#[allow(dead_code, unused_imports)]
#[path = "./urdf_generated.rs"]
mod urdf_generated;
pub use urdf_generated::kesko::urdf::root_as_spawn_urdf;

pub struct UrdfPlugin;

impl Plugin for UrdfPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_urdf);
    }

    fn name(&self) -> &str {
        "URDF Plugin"
    }
}

pub fn spawn_urdf(mut commands: Commands, mut system_requests: EventReader<SimulatorRequestEvent>) {
    for event in system_requests.iter() {
        warn!(">>>>>>>>>Event: ");
        match event {
            SimulatorRequestEvent::PublishFlatBuffers(data) => {
                if let Ok(spawn_urdf) = root_as_spawn_urdf(data.as_slice()) {
                    let urdf_path = spawn_urdf.urdf_path().unwrap_or("");
                    let position = spawn_urdf.position().unwrap();
                    let package_mapping = spawn_urdf.package_mappings().unwrap().iter().map(|mapping| {
                        (
                            mapping.package_name().unwrap_or(""),
                            mapping.package_path().unwrap_or(""),
                        )
                    }).collect::<HashMap<&str, &str>>();

                    warn!(
                        ">>>>>>>>>Name: {}, Position: ({}, {}, {}), mappings: {:?}",
                        urdf_path,
                        position.x(),
                        position.y(),
                        position.z(),
                        package_mapping
                    );
                }
            }
            _ => {}
        }
    }
}
