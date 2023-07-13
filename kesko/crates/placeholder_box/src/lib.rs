use bevy::prelude::*;
use kesko_core::event::SimulatorRequestEvent;
extern crate flatbuffers;

#[allow(dead_code, unused_imports)]
#[path = "./spawn_generated.rs"]
mod spawn_generated;
pub use spawn_generated::kesko::placeholder_box as spawn_fbs;

#[allow(dead_code, unused_imports)]
#[path = "./clear_generated.rs"]
mod clear_generated;
pub use clear_generated::kesko::placeholder_box as clear_fbs;

#[derive(Component)]
pub struct IsPlaceholderBox;

pub struct PlaceholderBoxPlugin;

impl Plugin for PlaceholderBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_placeholder_box);
    }

    fn name(&self) -> &str {
        "Placeholder Box Plugin"
    }
}

pub fn spawn_placeholder_box(
    mut commands: Commands,
    mut system_requests: EventReader<SimulatorRequestEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut placeholder_boxes: Query<Entity, With<IsPlaceholderBox>>,
) {
    for event in system_requests.iter() {
        match event {
            SimulatorRequestEvent::PublishFlatBuffers(data) => {
                if flatbuffers::buffer_has_identifier(
                    data,
                    spawn_fbs::SPAWN_PLACEHOLDER_BOX_IDENTIFIER,
                    false,
                ) {
                    let placeholder_box = spawn_fbs::root_as_spawn_placeholder_box(data.as_slice())
                        .expect(
                            format!(
                                "Failed to parse spawn placeholder box message ID: {}",
                                spawn_fbs::SPAWN_PLACEHOLDER_BOX_IDENTIFIER
                            )
                            .as_str(),
                        );
                    let transform = placeholder_box.transform().unwrap();
                    let color = placeholder_box.color().unwrap();
                    let name = placeholder_box.name().unwrap().to_string();

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
                    let color = Color::rgb(color.x(), color.y(), color.z());
                    // cube
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                            material: materials.add(color.into()),
                            transform,
                            ..default()
                        },
                        Name::new(name),
                        IsPlaceholderBox {},
                    ));
                } else if flatbuffers::buffer_has_identifier(
                    data,
                    clear_fbs::CLEAR_IDENTIFIER,
                    false,
                ) {
                    // remove all entities with IsPlaceholderBox component
                    for entity in placeholder_boxes.iter_mut() {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
            _ => {}
        }
    }
}
