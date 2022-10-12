pub mod car;
pub mod snake;
pub mod arena;
pub mod spider;
pub mod sphere;
pub mod wheely;
pub mod humanoid;
pub mod plane;

use bevy::prelude::*;
use serde::{Serialize, Deserialize};


pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::First, spawn_system);
    }
}


pub enum SpawnEvent {
    OpenWindow,
    Spawn {
        model: Model,
        transform: Transform,
        color: Color
    }
}


/// Description on how to manually control a robot
/// The text will be shown in the multibody ui
#[derive(Component)]
pub struct ControlDescription(pub String);


// Enum to represent each default model
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Model {
    Car,
    Snake,
    Spider,
    Sphere,
    Wheely,
    Humanoid,
    Arena,
    Plane
}

impl Model {

    /// Names to use for example in UI
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Car => "Car",
            Self::Snake => "Snake",
            Self::Spider => "Spider",
            Self::Sphere => "Sphere",
            Self::Wheely => "Wheely",
            Self::Humanoid => "Humanoid",
            Self::Arena => "Arena",
            Self::Plane => "Plane"
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
        if let SpawnEvent::Spawn { model, transform, color} = event {

            let material = materials.add(color.clone().into());

            match model {
                Model::Spider => spider::spawn(&mut commands, material, *transform, &mut meshes),
                Model::Snake => snake::spawn(&mut commands, material, *transform, &mut meshes),
                Model::Car => {
                    let wheel_material = materials.add(Color::DARK_GRAY.into());
                    car::Car::spawn(&mut commands, material, wheel_material, *transform, &mut meshes);
                },
                Model::Sphere => sphere::spawn(&mut commands, material, *transform, &mut meshes),
                Model::Wheely => {
                    let wheel_material = materials.add(Color::DARK_GRAY.into());
                    wheely::Wheely::spawn(&mut commands, material, wheel_material, *transform, &mut meshes);
                },
                Model::Humanoid => humanoid::Humanoid::spawn(&mut commands, material, *transform, &mut meshes),
                Model::Arena => arena::spawn(&mut commands, material, &mut meshes, 10.0, 10.0, 1.0),
                Model::Plane => plane::spawn(&mut commands, material, &mut meshes)
            }
        }
    }
}
