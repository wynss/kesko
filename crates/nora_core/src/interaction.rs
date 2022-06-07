use bevy::prelude::*;
use nora_raycast::RayCastSource;
use nora_object_interaction::event::InteractionEvent;


#[derive(Component)]
pub(crate) struct VerticalMarker(Entity);

pub(crate) fn handle_vertical_marker_spawning(
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<StandardMaterial>>,
    mut event_reader: EventReader<InteractionEvent>,
    mut commands: Commands,
    source_query: Query<&RayCastSource, Without<Camera>>,
    marker_query: Query<(Entity, &VerticalMarker)>
) {

    for event in event_reader.iter() {
        match event {
            InteractionEvent::DragStarted(entity) => {            
                if let Ok(source) = source_query.get(*entity) {

                    let marker_entity = commands.spawn_bundle( PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
                        ..default()
                    }).insert(VerticalMarker(*entity)).id();
                }
            },
            InteractionEvent::DragStopped(entity) => {
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

pub(crate) fn update_vertical_marker_pos_system(
    ray_query: Query<&RayCastSource, Without<Camera>>,
    mut marker_query: Query<(&VerticalMarker, &mut Transform), With<VerticalMarker>>,
) {
    for (marker, mut transform) in marker_query.iter_mut() {
        if let Ok(ray_source) = ray_query.get(marker.0) {
            if let Some(hit) = &ray_source.ray_hit {
                if let Some(ray) = &ray_source.ray {

                    let pos = ray.origin + (ray.origin - hit.intersection.point) / 2.0;
                    *transform.translation = pos.into();
                    *transform.scale = Vec3::new(1.0, hit.intersection.distance, 1.0).into();
                }
            }
        }
    }
}