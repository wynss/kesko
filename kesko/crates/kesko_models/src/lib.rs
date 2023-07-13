pub mod arena;
pub mod car;
pub mod cube;
pub mod humanoid;
pub mod plane;
pub mod snake;
pub mod sphere;
pub mod spider;
pub mod wheely;
// pub mod gltf_model;

use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
pub enum SpawnSet {
    Spawn,
    SpawnFlush,
}

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            (
                CoreSet::FirstFlush,
                SpawnSet::Spawn,
                SpawnSet::SpawnFlush,
                CoreSet::PreUpdate,
            )
                .chain(),
        )
        .add_plugin(bevy_stl::StlPlugin)
        .add_system(spawn_system.in_base_set(SpawnSet::Spawn))
        .add_system(apply_system_buffers.in_base_set(SpawnSet::SpawnFlush))
        .add_event::<SpawnEvent>();
    }
}

pub enum SpawnEvent {
    OpenWindow,
    Spawn {
        model: Model,
        transform: Transform,
        color: Color,
    },
    // SpawnAsset {
    //     asset_path: String,
    //     transform: Transform,
    // },
}

/// Description on how to manually control a robot
/// The text will be shown in the multibody ui
#[derive(Component)]
pub struct ControlDescription(pub String);

// Enum to represent each default model
#[cfg_attr(not(target_arch = "wasm32"), pyclass(get_all))]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Model {
    Car,
    Cube,
    Snake,
    Spider,
    Sphere,
    Wheely,
    Humanoid,
    Arena,
    Plane,
}

#[cfg_attr(not(target_arch = "wasm32"), pymethods)]
impl Model {
    /// Names to use for example in UI
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Car => "Car",
            Self::Cube => "Cube",
            Self::Snake => "Snake",
            Self::Spider => "Spider",
            Self::Sphere => "Sphere",
            Self::Wheely => "Wheely",
            Self::Humanoid => "Humanoid",
            Self::Arena => "Arena",
            Self::Plane => "Plane",
        }
    }
}

/// System to spawn a model given an spawn event
pub fn spawn_system(
    mut spawn_event_reader: EventReader<SpawnEvent>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for event in spawn_event_reader.iter() {
        if let SpawnEvent::Spawn {
            model,
            transform,
            color,
        } = event
        {
            debug!("Spawning model {:?}", model);

            let material = materials.add(color.clone().into());

            match model {
                Model::Spider => spider::spawn(&mut commands, material, *transform, &mut meshes),
                Model::Snake => {
                    snake::Snake::spawn(&mut commands, material, *transform, &mut meshes)
                }
                Model::Car => {
                    let wheel_material = materials.add(Color::DARK_GRAY.into());
                    car::Car::spawn(
                        &mut commands,
                        material,
                        wheel_material,
                        *transform,
                        &mut meshes,
                    );
                }
                Model::Cube => cube::Cube::spawn(&mut commands, material, *transform, &mut meshes),
                Model::Sphere => {
                    sphere::Sphere::spawn(&mut commands, material, *transform, &mut meshes)
                }
                Model::Wheely => {
                    let wheel_material = materials.add(Color::DARK_GRAY.into());
                    wheely::Wheely::spawn(
                        &mut commands,
                        material,
                        wheel_material,
                        *transform,
                        &mut meshes,
                    );
                }
                Model::Humanoid => {
                    humanoid::Humanoid::spawn(&mut commands, material, *transform, &mut meshes)
                }
                Model::Arena => arena::spawn(&mut commands, material, &mut meshes, 10.0, 10.0, 1.0),
                Model::Plane => plane::spawn(&mut commands, material, &mut meshes),
            }
        }
    }
}
