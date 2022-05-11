use bevy::prelude::*;

use crate::RayCastSource;

#[derive(Component)]
pub(crate) struct DebugPointer;

pub(crate) fn spawn_debug_pointer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.06, subdivisions: 5})),
        material: materials.add(StandardMaterial {
            base_color: Color::CRIMSON,
            unlit: true,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..Default::default()
    }).insert(DebugPointer);
}

pub(crate) fn update_debug_pointer(
    query: Query<&RayCastSource>,
    mut pointer_query: Query<&mut Transform, With<DebugPointer>>
) {

    for source in query.iter() {
        if let Some(ray) = &source.ray {

            if let Ok(mut transform) = pointer_query.get_single_mut() {

                if let Some(intersection) = &source.intersection {

                    transform.translation = intersection.intersection;

                } else {

                    let translation = ray.origin + ray.direction * 5.0;
                    transform.translation = translation;

                }

            }

        }
    }
}
