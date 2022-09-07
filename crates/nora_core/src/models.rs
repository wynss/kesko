pub mod car;
pub mod snake;
pub mod arena;
pub mod spider;
pub mod sphere;

use bevy::prelude::*;

use crate::ui::spawn_component::SpawnEvent;


// Enum to represent each default model
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Model {
    Car,
    Snake,
    Spider,
    Sphere
}

impl Model {

    /// Names to use for example in UI
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Car => "Car",
            Self::Snake => "Snake",
            Self::Spider => "Spider",
            Self::Sphere => "Sphere",
        }
    }
}

/// System to spawn a model given an spawn event
pub(crate) fn spawn_model_system(
    mut ui_event_reader: EventReader<SpawnEvent>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for event in ui_event_reader.iter() {
        if let SpawnEvent::Spawn { model, transform, color} = event {

            let material = materials.add(color.clone().into());

            match model {
                Model::Spider => spider::spawn_spider(&mut commands, material, *transform, &mut meshes),
                Model::Snake => snake::spawn_snake(&mut commands, material, *transform, &mut meshes),
                Model::Car => {
                    let wheel_material = materials.add(Color::DARK_GRAY.into());
                    car::spawn_car(&mut commands, material, wheel_material, *transform, &mut meshes);
                },
                Model::Sphere => sphere::spawn_sphere(&mut commands, material, *transform, &mut meshes)
            }
        }
    }
}
