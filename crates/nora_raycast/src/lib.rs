extern crate core;

pub(crate) mod ray;
pub(crate) mod intersect;
pub(crate) mod debug;
pub(crate) mod triangle;
pub(crate) mod convert;

use bevy::prelude::*;

use ray::Ray;
use intersect::{RayHit, mesh_intersection};


#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub enum RayCastSystems {
    CreateRays,
    CalcIntersections,
    Debug
}

#[derive(Default)]
pub struct RayCastPlugin;
impl Plugin for RayCastPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(debug::spawn_debug_pointer)
            .add_system_set_to_stage(
            CoreStage::First,
            SystemSet::new()
                .with_system(create_rays_system.label(RayCastSystems::CreateRays))
                .with_system(calc_intersections_system
                    .label(RayCastSystems::CalcIntersections)
                    .after(RayCastSystems::CreateRays)
                )
                .with_system(debug::update_debug_pointer
                    .label(RayCastSystems::Debug)
                    .after(RayCastSystems::CalcIntersections)
                )
        );
    }
}

pub enum RayCastMethod {
    ScreenSpace,
    WorldSpace
}

/// Source that will cast rays
#[derive(Component)]
pub struct RayCastSource {
    method: RayCastMethod,
    ray: Option<Ray>,
    ray_hit: Option<RayHit>
}


impl RayCastSource {
    pub fn new(method: RayCastMethod) -> Self {
        Self {
            method,
            ray: None,
            ray_hit: None
        }
    }

    pub fn screen_space() -> Self {
        Self {
            method: RayCastMethod::ScreenSpace,
            ray: None,
            ray_hit: None
        }
    }
}

/// An entity that are visible to the rays
#[derive(Component, Default)]
pub struct RayCastable;


fn create_rays_system(
    windows: Res<Windows>,
    mut ray_source_query: Query<(&mut RayCastSource, Option<&Camera>, Option<&GlobalTransform>)>
) {

    let window = windows.get_primary().unwrap();
    for (mut ray_source, camera, camera_transform) in ray_source_query.iter_mut() {

        // todo: Remove this and make a separate reset system that can be triggered
        ray_source.ray_hit = None;

        match ray_source.method {
            RayCastMethod::ScreenSpace => {

                let camera = match camera {
                    Some(camera) => camera,
                    None => {
                        error!("Ray cast source with method screen space has no camera component");
                        return;
                    }
                };

                let camera_transform = match camera_transform {
                    Some(camera_transform) => camera_transform,
                    None => {
                        error!("Camera has not global transform");
                        return;
                    }
                };

                if let Some(cursor_position) = window.cursor_position() {
                    ray_source.ray = Some(Ray::from_screen_space(window, camera, camera_transform, cursor_position));
                }
            },
            RayCastMethod::WorldSpace => {
                todo!();
            }
        }
    }
}

pub(crate) fn calc_intersections_system(
    meshes: Res<Assets<Mesh>>,
    mut source_query: Query<&mut RayCastSource>,
    castable_query: Query<(&Handle<Mesh>, &GlobalTransform), With<RayCastable>>
) {
    for mut source in source_query.iter_mut() {
        if let Some(ray) = &source.ray {

            let mut ray_hits: Vec<RayHit> = Vec::new();
            for (mesh_handle, transform) in castable_query.iter() {

                if let Some(mesh) = meshes.get(mesh_handle) {

                    let mesh_to_world = transform.compute_matrix();
                    if let Some(intersection) = mesh_intersection(mesh, ray, &mesh_to_world) {
                        ray_hits.push(RayHit{intersection});
                    }

                } else {
                    error!("No mesh for mesh handle");
                    continue;
                }
            }

            // todo: improve this
            let min_distance = f32::MAX;
            let mut closest_hit: Option<RayHit> = None;

            for hit in ray_hits.iter() {
                if hit.intersection.distance < min_distance {
                    closest_hit = Some(hit.clone());
                }
            }

            if let Some(hit) = closest_hit {
                source.ray_hit = Some(hit)
            }

        };
    }
}


#[cfg(test)]
mod tests {

}