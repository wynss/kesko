use std::marker::PhantomData;

use bevy::prelude::*;

use kesko_object_interaction::event::InteractionEvent;
use kesko_physics::mass::MultibodyMass;
use kesko_physics::{
    gravity::GravityScale,
    mass::Mass,
    force::Force
};
use kesko_raycast::RayCastSource;
use crate::orbit_camera::PanOrbitCamera;
use crate::controller::{PID, Controller};


/// constants for the PID-controller
const P: f32 = 10.0;
const I: f32 = 0.1;
const D: f32 = 5.0;

#[derive(Default)]
pub struct GrabablePlugin<T> {
    _phantom: PhantomData<fn() -> T>
}
impl<T> Plugin for GrabablePlugin<T>
where T: Component + Default
{
    fn build(&self, app: &mut App) {
       app
       .add_system(update_tracking_system::<T>.after(update_tracking_controller_system::<T>))
       .add_system(update_tracking_controller_system::<T>);
    }
}

/// Component that will keep track of quantities of object cursor tracking. The object tracking the cursor
/// will move on the plane perpendiculuar to the camera direction
#[derive(Component)]
pub(crate) struct CursorTrack {
    /// Point on the plane where the object will move
    plane_point: Vec3,
    /// Normal of the plane
    plane_normal: Vec3,
    /// PID controller
    controller: PID<Vec3>
}

/// Updates the object that should track the cursor and when it should stop
/// The object will track the cursor on a plane perpendicular to the camera direction places at the objects origin
/// While an object is tracking the cursor the gravity scale will be set to 0.0
pub(crate) fn update_tracking_system<T: Component + Default>(
    mut commands: Commands,
    mut interaction_events: EventReader<InteractionEvent>,
    mut entity_query: Query<(&mut GravityScale, &mut Force, &GlobalTransform)>,
    ray_query: Query<&GlobalTransform, (With<RayCastSource<T>>, With<PanOrbitCamera>)>
) {

    for event in interaction_events.iter() {
        if let InteractionEvent::DragStarted(entity) = event {
            if let Ok(camera_transform) = ray_query.get_single() {
                if let Ok((mut gravity_scale, _, entity_transform)) = entity_query.get_mut(*entity) {
                    let cursor_track = create_track_comp(
                        &camera_transform.compute_matrix(),
                        &entity_transform.translation()
                    );
                    commands.entity(*entity).insert(cursor_track);
                    gravity_scale.val = 0.0;
                }
            }
        } else if let InteractionEvent::DragStopped(entity) = event {
            if let Ok((mut gravity_scale, mut force, _)) = entity_query.get_mut(*entity) {
                gravity_scale.reset();
                force.reset();
            }
            commands.entity(*entity).remove::<CursorTrack>();
        }
    }
}

/// System that will update the controller of the object cursor tracking using a PD controller
#[allow(clippy::type_complexity)]
pub(crate) fn update_tracking_controller_system<T: Component + Default>(
    ray_query: Query<(&RayCastSource<T>, &Transform), With<PanOrbitCamera>>,
    mut track_query: Query<(&mut CursorTrack, &mut Force, &Mass, Option<&MultibodyMass>, &GlobalTransform), With<CursorTrack>>
) {
    if let Ok((ray_source, camera_transform)) = ray_query.get_single() {
        if let Some(ray) = &ray_source.ray {
            for (mut track, mut force, mass, multibody_mass, transform) in track_query.iter_mut() {

                let plane_normal = camera_transform.compute_matrix().transform_vector3(Vec3::Z).normalize();

                let distance = (track.plane_point - ray.origin).dot(plane_normal) / ray.direction.dot(plane_normal);
                let plane_intersection = ray.origin + distance * ray.direction;
                
                let pos_diff: Vec3 = plane_intersection - transform.translation();

                let mut control_output = track.controller.act(pos_diff);

                if let Some(mass) = multibody_mass {
                    control_output *= mass.val;
                } else {
                    control_output *= mass.val;
                }

                force.vec = control_output;
                track.plane_normal = plane_normal;
            }
        }
    }
}

fn create_track_comp(camera_view_transform: &Mat4, object_translation: &Vec3) -> CursorTrack {
    let plane_normal = camera_view_transform.transform_vector3(Vec3::Z).normalize();
    let pid = PID::<Vec3>::new(P, I, D);
    CursorTrack { plane_point: *object_translation, plane_normal, controller: pid}
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
        assert!(res.controller.prev_val.is_none());
    }
}