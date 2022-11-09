use bevy::prelude::*;

use crate::RayCastSource;

#[derive(Component)]
pub(crate) struct DebugPointer;

pub(crate) fn spawn_debug_pointer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.05,
                depth: 0.5,
                ..Default::default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::CRIMSON,
                unlit: true,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 1.0, 0.0).with_scale(Vec3::ZERO),
            ..Default::default()
        })
        .insert(DebugPointer);
}

pub(crate) fn update_debug_pointer<T: Component + Default>(
    query: Query<&RayCastSource<T>>,
    mut pointer_query: Query<&mut Transform, With<DebugPointer>>,
) {
    if let Ok(mut transform) = pointer_query.get_single_mut() {
        if query.is_empty() {
            transform.scale = Vec3::ZERO;
        }

        for source in query.iter() {
            match &source.ray_hit {
                None => {
                    transform.scale = Vec3::ZERO;
                }
                Some(ray_hit) => {
                    // make visible
                    transform.scale = Vec3::ONE;

                    // rotation
                    let up = Vec3::Y;
                    let rotation_axis = up.cross(ray_hit.intersection.normal).normalize();
                    let angle = up.dot(ray_hit.intersection.normal).acos();
                    let rotation = if angle.abs() > f32::EPSILON {
                        Quat::from_axis_angle(rotation_axis, angle)
                    } else {
                        Quat::default()
                    };

                    // Apply transformation
                    *transform = Transform::from_matrix(Mat4::from_rotation_translation(
                        rotation,
                        ray_hit.intersection.point,
                    ));
                }
            }
        }
    }
}
