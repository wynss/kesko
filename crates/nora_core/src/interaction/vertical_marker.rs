use bevy::prelude::*;
use nora_raycast::RayCastSource;
use nora_object_interaction::event::InteractionEvent;


#[derive(Component)]
pub(crate) struct VerticalMarker(Entity);

pub(crate) fn handle_vertical_marker_spawning<T: Component + Default>(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut event_reader: EventReader<InteractionEvent>,
    mut commands: Commands,
    marker_query: Query<(Entity, &VerticalMarker)>
) {

    for event in event_reader.iter() {
        match event {
            InteractionEvent::DragStarted(entity) => {            
                commands.spawn_bundle( PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgba(1.0, 0.0, 0.0, 0.3),
                        unlit: true,
                        alpha_mode: AlphaMode::Blend,
                        ..default()
                    }),
                    transform: Transform::from_scale(Vec3::ZERO),
                    ..default()
                }).insert(VerticalMarker(*entity));

                commands.entity(*entity).insert(RayCastSource::<T>::from_entity(-Vec3::Y));
            },
            InteractionEvent::DragStopped(entity) => {
                
                commands.entity(*entity).remove::<RayCastSource<T>>();

                for (marker_entity, marker) in marker_query.iter() {
                    if *entity == marker.0 {
                        commands.entity(marker_entity).despawn();
                    }
                }
            },
            _ => ()
        }
    }
}

pub(crate) fn update_vertical_marker_pos_system<T: Component + Default>(
    ray_query: Query<&RayCastSource<T>, Without<Camera>>,
    mut marker_query: Query<(&VerticalMarker, &mut Transform), With<VerticalMarker>>,
) {
    for (marker, mut transform) in marker_query.iter_mut() {
        if let Ok(ray_source) = ray_query.get(marker.0) {
            if let Some(hit) = &ray_source.ray_hit {
                if let Some(ray) = &ray_source.ray {

                    let pos = ray.origin + (hit.intersection.point - ray.origin) / 2.0;
                    *transform.translation = pos.into();
                    *transform.scale = Vec3::new(0.05, hit.intersection.distance, 0.05).into();
                }
            } else {
                *transform.scale = Vec3::ZERO.into();
            }
        }
    }
}