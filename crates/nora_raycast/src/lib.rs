extern crate core;

pub mod ray;
pub mod intersect;
pub(crate) mod debug;
pub(crate) mod triangle;
pub(crate) mod convert;

use std::{
    sync::{Arc, Mutex}, 
    marker::PhantomData
};

use bevy::prelude::*;
use bevy::render::{
    camera::Projection,
    primitives::Aabb
};

use ray::Ray;
use intersect::{RayHit, mesh_intersection, aabb_intersection};


#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub enum RayCastSystems {
    CreateRays,
    CalcIntersections,
    Debug
}

#[derive(Default)]
pub struct RayCastPlugin<T: Component> {
    debug: bool,
    _phantom: PhantomData<fn() -> T>
}

impl<T> RayCastPlugin<T> 
where T: Component 
{
    pub fn with_debug() -> Self {
        Self {
            debug: true,
            _phantom: PhantomData
        }
    }
}

impl<T> Plugin for RayCastPlugin<T> 
where T: Component + Default
{
    fn build(&self, app: &mut App) {

        app.add_system_set_to_stage(
            CoreStage::First,
            SystemSet::new()
                .with_system(create_rays_system::<T>
                    .label(RayCastSystems::CreateRays))
                .with_system(calc_intersections_system::<T>
                    .label(RayCastSystems::CalcIntersections)
                    .after(RayCastSystems::CreateRays)
                )
        );

        if self.debug {
            app.add_startup_system(debug::spawn_debug_pointer);
            app.add_system_to_stage(CoreStage::First, debug::update_debug_pointer::<T>);
        }
    }
}

pub enum RayCastMethod {
    ScreenSpace,
    FromEntity {
        direction: Vec3
    }
}

/// Source that will cast rays
#[derive(Component)]
pub struct RayCastSource<T> 
where T: Component + Default
{
    pub method: RayCastMethod,
    pub ray: Option<Ray>,
    pub ray_hit: Option<RayHit>,
    pub prev_ray_hit: Option<RayHit>,
    _phantom: PhantomData<fn() -> T>
}


impl<T> RayCastSource<T> 
where T: Component + Default
{
    pub fn new(method: RayCastMethod) -> Self {
        Self {
            method,
            ray: None,
            ray_hit: None,
            prev_ray_hit: None,
            _phantom: PhantomData
        }
    }

    pub fn from_entity(direction: Vec3) -> Self {
        Self {
            method: RayCastMethod::FromEntity { direction },
            ray: None,
            ray_hit: None,
            prev_ray_hit: None,
            _phantom: PhantomData
        }
    }

    pub fn screen_space() -> Self {
        Self {
            method: RayCastMethod::ScreenSpace,
            ray: None,
            ray_hit: None,
            prev_ray_hit: None,
            _phantom: PhantomData
        }
    }
}

/// Component that makes an entity visible to rays
#[derive(Component, Default)]
pub struct RayVisible<T> {
    _marker: PhantomData<fn() -> T>
}


#[allow(clippy::type_complexity)]
fn create_rays_system<T: Component + Default>(
    windows: Res<Windows>,
    mut ray_source_query: Query<(&mut RayCastSource<T>, Option<&Camera>, Option<&Projection>, Option<&GlobalTransform>)>
) {

    let window = windows.get_primary().unwrap();
    for (mut ray_source, camera, perspective_projection, transform) in ray_source_query.iter_mut() {

        // todo: Remove this and make a separate reset system that can be triggered
        if let Some(ray_hit) = &ray_source.ray_hit {
            ray_source.prev_ray_hit = Some(ray_hit.clone());
        } else {
            ray_source.prev_ray_hit = None;
        }
        ray_source.ray_hit = None;

        match transform {
            Some(transform) => {
                match ray_source.method {
                    RayCastMethod::ScreenSpace => {

                        let camera = match camera {
                            Some(camera) => camera,
                            None => {
                                error!("Ray cast source with method screen space has no camera component");
                                return;
                            }
                        };

                        let perspective_projection = match perspective_projection {
                            Some(perspective_projection) => match perspective_projection {
                                Projection::Perspective(perspective_proj) => perspective_proj,
                                _ => {
                                    error!("Got wrong projection for main camera");
                                    return;
                                }
                            },
                            None => {
                                error!("Could not get perspective projection for main camera");
                                return;
                            }
                        };

                        if let Some(cursor_position) = window.cursor_position() {
                            ray_source.ray = Some(Ray::from_screen_space(window, camera, perspective_projection, transform, cursor_position));
                        }
                    },
                    RayCastMethod::FromEntity { direction } => {
                        ray_source.ray = Some(Ray::from_world_space(transform.translation(), direction));
                    }
                }

            },
            None => {
                error!("Camera has no global transform");
                return;
            }
        };
    }
}

#[allow(clippy::type_complexity)]
fn calc_intersections_system<T: Component + Default> (
    meshes: Res<Assets<Mesh>>,
    culling_query: Query<(Entity, Option<&Aabb>, &Visibility, &ComputedVisibility, &GlobalTransform), With<RayVisible<T>>>,
    mut source_query: Query<&mut RayCastSource<T>>,
    visible_query: Query<(Entity, &Handle<Mesh>, &GlobalTransform), With<RayVisible<T>>>
) {


    for mut source in source_query.iter_mut() {

        if let Some(ray) = &source.ray {

            // first test intersection for the entities that has a aabb
            let culled_entities = culling_query.iter().filter_map(
                |(entity, aabb, visibility, computed_visibility, transform)| {

                    // check visibility, both user defined and computed
                    match source.method {
                        RayCastMethod::ScreenSpace => {
                            if !visibility.is_visible && !computed_visibility.is_visible() { return None }
                        },
                        _ => {
                            if !visibility.is_visible { return None }
                        }
                    }

                    let mesh_to_world = transform.compute_matrix();
                    match aabb {
                        Some(aabb) => {
                            if aabb_intersection(ray, aabb, &mesh_to_world) { Some(entity) }
                            else { None }
                        },
                        None => Some(entity)
                    }
            }).collect::<Vec<Entity>>();

            let ray_hits = Arc::new(Mutex::new(Vec::new()));

            visible_query.par_for_each(num_cpus::get(), 
                | (entity, mesh_handle, transform) | {
                    if culled_entities.contains(&entity) {
                        if let Some(mesh) = meshes.get(mesh_handle) {
                            let mesh_to_world = transform.compute_matrix();
                            if let Some(intersection) = mesh_intersection(mesh, ray, &mesh_to_world) {
                                ray_hits.lock().unwrap().push(RayHit{
                                    entity,
                                    intersection
                                });
                            }
                        } else {
                            error!("No mesh for mesh handle");
                        }
                    }
            });

            // todo: improve this
            let mut min_distance = f32::MAX;
            let mut closest_hit: Option<RayHit> = None;

            for hit in ray_hits.lock().unwrap().iter() {
                if hit.intersection.distance < min_distance {
                    closest_hit = Some(hit.clone());
                    min_distance = hit.intersection.distance;
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