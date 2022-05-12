pub(crate) mod ray;
pub(crate) mod intersect;
pub(crate) mod debug;
pub(crate) mod triangle;

use bevy::prelude::*;

use ray::Ray;


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
                .with_system(create_rays.label(RayCastSystems::CreateRays))
                .with_system(intersect::calc_intersections_system
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
    intersection: Option<intersect::Intersection>
}


impl RayCastSource {
    pub fn new(method: RayCastMethod) -> Self {
        Self {
            method,
            ray: None,
            intersection: None
        }
    }
}

/// An entity that are visible to the rays
#[derive(Component, Default)]
pub struct RayCastable;


fn create_rays(
    windows: Res<Windows>,
    mut ray_source_query: Query<(&mut RayCastSource, Option<&Camera>, Option<&GlobalTransform>)>
) {

    let window = windows.get_primary().unwrap();
    for (mut ray_source, camera, camera_transform) in ray_source_query.iter_mut() {

        // todo: Remove this and make a separate reset system that can be triggered
        ray_source.intersection = None;

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
                    ray_source.ray = Some(Ray::from_screen_space(
                        window, camera,
                        camera_transform,
                        cursor_position)
                    );
                }
            },
            RayCastMethod::WorldSpace => {
                todo!();
            }
        }
    }
}


#[cfg(test)]
mod tests {

}