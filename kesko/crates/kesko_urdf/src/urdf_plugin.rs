use bevy::{prelude::*, utils::hashbrown::HashMap};
use kesko_core::event::SimulatorRequestEvent;
use kesko_models::SpawnSet;
extern crate flatbuffers;

#[allow(dead_code, unused_imports)]
#[path = "./urdf_generated.rs"]
mod urdf_generated;
pub use urdf_generated::kesko::urdf::root_as_spawn_urdf;

use crate::{convert_urdf_to_components, UrdfAsset, UrdfAssetLoader, UrdfModel};

pub struct UrdfPlugin;

impl Plugin for UrdfPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<UrdfAsset>()
            .init_asset_loader::<UrdfAssetLoader>()
            .add_system(convert_urdf_to_components.in_base_set(SpawnSet::Spawn))
            .add_system(spawn_urdf);
    }

    fn name(&self) -> &str {
        "URDF Plugin"
    }
}

pub fn spawn_urdf(
    mut commands: Commands,
    mut system_requests: EventReader<SimulatorRequestEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in system_requests.iter() {
        match event {
            SimulatorRequestEvent::PublishFlatBuffers(data) => {
                if let Ok(spawn_urdf) = root_as_spawn_urdf(data.as_slice()) {
                    let urdf_path = spawn_urdf.urdf_path().unwrap_or("").to_string();
                    let position = spawn_urdf.position().unwrap();
                    let package_mapping = spawn_urdf
                        .package_mappings()
                        .unwrap()
                        .iter()
                        .map(|mapping| {
                            (
                                mapping.package_name().unwrap_or("").into(),
                                mapping.package_path().unwrap_or("").into(),
                            )
                        })
                        .collect::<HashMap<String, String>>();

                    let transform = Transform::from_xyz(position.x(), position.y(), position.z());
                    UrdfModel::spawn(
                        &mut commands,
                        urdf_path,
                        &package_mapping,
                        transform,
                        &asset_server,
                    );
                }
            }
            _ => {}
        }
    }
}
