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
        mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.1, subdivisions: 5})),
        material: materials.add(StandardMaterial {
            base_color: Color::CRIMSON,
            unlit: true,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 1.0, 0.0).with_scale(Vec3::ZERO),
        ..Default::default()
    }).insert(DebugPointer);
}

pub(crate) fn update_debug_pointer(
    query: Query<&RayCastSource>,
    mut pointer_query: Query<&mut Transform, With<DebugPointer>>
) {

    if let Ok(mut transform) = pointer_query.get_single_mut() {
        if query.is_empty() {
            transform.scale = Vec3::ZERO;
        }

        for source in query.iter() {
            if let Some(ray_hit) = &source.ray_hit {
                transform.scale = Vec3::ONE;
                transform.translation = ray_hit.intersection.point;

            } else {
                transform.scale = Vec3::ZERO;
            }
        }
    }
}
