use bevy::prelude::*;
use nora_object_interaction::event::InteractionEvent;
use nora_physics::gravity::GravityScale;
use nora_physics::impulse::Impulse;
use nora_physics::mass::Mass;
use nora_raycast::RayCastSource;
use crate::orbit_camera::PanOrbitCamera;

/// P and D constants for the PD-controller
const P: f32 = 1.3;
const D: f32 = 0.30;

/// Component that will keep track of quantities of object cursor tracking. The object tracking the cursor
/// will move on the plane perpendicular to the camera direction
#[derive(Component, Default)]
pub(crate) struct CursorTrack {
    /// Point on the plane where the object will move
    plane_point: Vec3,
    /// Normal of the plane
    plane_normal: Vec3,
    /// Previous applied impulse vector
    prev_impulse: Option<Vec3>
}

/// Updates the object that should track the cursor and when it should stop
/// The object will track the cursor on a plane perpendicular to the camera direction places at the objects origin
/// While an object is tracking the cursor the gravity scale will be set to 0.0
pub(crate) fn update_tracking_system(
    mut commands: Commands,
    mut interaction_events: EventReader<InteractionEvent>,
    mut entity_query: Query<(&mut GravityScale, &GlobalTransform)>,
    ray_query: Query<&GlobalTransform, (With<RayCastSource>, With<PanOrbitCamera>)>
) {

    for event in interaction_events.iter() {
        if let InteractionEvent::DragStarted(entity) = event {
            if let Ok(camera_transform) = ray_query.get_single() {
                if let Ok((mut gravity_scale, entity_transform)) = entity_query.get_mut(*entity) {
                    let cursor_track = create_track_comp(
                        &camera_transform.compute_matrix(),
                        &entity_transform.translation
                    );
                    commands.entity(*entity).insert(cursor_track);
                    gravity_scale.val = 0.0;
                }
            }
        } else if let InteractionEvent::DragStopped(entity) = event {
            if let Ok((mut gravity_scale, _)) = entity_query.get_mut(*entity) {
                gravity_scale.val = 1.0;
            }
            commands.entity(*entity).remove::<CursorTrack>();
        }
    }
}

/// System that will update the controller of the object cursor tracking using a PD controller
pub(crate) fn update_tracking_controller_system(
    ray_query: Query<(&RayCastSource, &Transform), With<PanOrbitCamera>>,
    mut track_query: Query<(&mut CursorTrack, &mut Impulse, &Mass, &GlobalTransform)>
) {
    if let Ok((ray_source, camera_transform)) = ray_query.get_single() {
        if let Some(ray) = &ray_source.ray {
            for (mut track, mut impulse, mass, transform) in track_query.iter_mut() {

                let plane_normal = camera_transform.compute_matrix().transform_vector3(Vec3::Z).normalize();

                let distance = (track.plane_point - ray.origin).dot(plane_normal) / ray.direction.dot(plane_normal);
                let plane_intersection = ray.origin + distance * ray.direction;

                let impulse_vec: Vec3 = plane_intersection - transform.translation;

                if let Some(prev_impulse) = track.prev_impulse {
                    impulse.vec = (P * impulse_vec + D * (impulse_vec - prev_impulse) * 60.0) * mass.val;
                }

                track.plane_normal = plane_normal;
                track.prev_impulse = Some(impulse_vec);
            }
        }
    }
}

fn create_track_comp(camera_view_transform: &Mat4, object_translation: &Vec3) -> CursorTrack {
    let plane_normal = camera_view_transform.transform_vector3(Vec3::Z).normalize();
    CursorTrack { plane_point: *object_translation, plane_normal, prev_impulse: None}
}


#[cfg(test)]
mod tests {
    use bevy::math::Vec3;
    use bevy::prelude::Transform;
    use crate::cursor_tracking::create_track_comp;

    #[test]
    fn test_start_tracking() {

        let transform = Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y);
        let view_transform = transform.compute_matrix();

        let object_translation = Vec3::ZERO;

        let res = create_track_comp(&view_transform, &object_translation);

        assert_eq!(res.plane_point, object_translation);
        assert_eq!(res.plane_normal, Vec3::Z);
        assert!(res.prev_impulse.is_none());
    }
}