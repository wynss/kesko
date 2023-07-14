use anyhow::{anyhow, Result};
use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_obj::ObjPlugin;
use kesko_core::event::SimulatorRequestEvent;
use kesko_models::SpawnSet;

use crate::{convert_sdf_to_components, SdfAsset, SdfAssetLoader, SdfModel};
extern crate flatbuffers;

#[allow(dead_code, unused_imports)]
#[path = "./spawn_generated.rs"]
mod spawn_generated;
pub use spawn_generated::kesko::sdf_model_loader as spawn_fbs;

pub struct SdfPlugin;

impl Plugin for SdfPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<SdfAsset>()
            .init_asset_loader::<SdfAssetLoader>()
            .add_plugin(ObjPlugin)
            .add_system(convert_sdf_to_components.in_base_set(SpawnSet::Spawn))
            .add_system(parse_spawn_message.pipe(spawn_sdf_model));
    }

    fn name(&self) -> &str {
        "Simulation Description Format loader Plugin"
    }
}

pub fn parse_spawn_message(
    mut system_requests: EventReader<SimulatorRequestEvent>,
) -> Result<Vec<SdfModel>> {
    let models = system_requests
        .iter()
        .map(|event| {
            if let SimulatorRequestEvent::PublishFlatBuffers(data) = event {
                if flatbuffers::buffer_has_identifier(
                    data,
                    spawn_fbs::SPAWN_SDF_MODEL_IDENTIFIER,
                    false,
                ) {
                    println!("SpawnSdfModel message received");
                    let sdf_model = spawn_fbs::root_as_spawn_sdf_model(data.as_slice()).expect(
                        format!(
                            "Failed to parse spawn placeholder box message ID: {}",
                            spawn_fbs::SPAWN_SDF_MODEL_IDENTIFIER
                        )
                        .as_str(),
                    );
                    let transform = sdf_model.transform().unwrap();
                    let sdf_path = sdf_model.sdf_path().unwrap().to_string();

                    let transform = Transform::from_xyz(
                        transform.translation().x(),
                        transform.translation().y(),
                        transform.translation().z(),
                    )
                    .with_rotation(Quat::from_euler(
                        EulerRot::XYZ,
                        transform.rotation().x(),
                        transform.rotation().y(),
                        transform.rotation().z(),
                    ))
                    .with_scale(Vec3::new(
                        transform.scale().x(),
                        transform.scale().y(),
                        transform.scale().z(),
                    ));

                    return Ok(SdfModel::new(sdf_path, transform));
                }
            }

            Err(anyhow!("Not a SpawnSdfModel message"))
        })
        .filter(|message| message.is_ok())
        .map(|message| message.unwrap())
        .collect::<Vec<SdfModel>>();

    Ok(models)
}

pub fn spawn_sdf_model(
    In(result): In<Result<Vec<SdfModel>>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Ok(models) = result {
        for model in models {
            model.spawn(&mut commands, &asset_server);
        }
    }
}
