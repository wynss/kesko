pub(crate) mod ray;

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::math as math;


use ray::Ray;

#[derive(Default)]
pub struct RayCastPlugin;
impl Plugin for RayCastPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::First,
            SystemSet::new()
                .with_system(create_rays)
        );
    }
}

pub enum RayCastMethod {
    Screenspace,
    WorldSpace
}

/// Source that will cast rays
#[derive(Component)]
pub struct RayCastSource {
    method: RayCastMethod,
    ray: Option<Ray>
}
impl RayCastSource {
    pub fn new(method: RayCastMethod) -> Self {
        Self {
            method,
            ray: None
        }
    }
}

/// An entity that are visible to the rays
#[derive(Component, Default)]
pub struct RayCastable {

}

struct RayIntersect;
impl RayIntersect {
    fn intersections() -> bool {
        todo!()
    }
}


fn create_rays(
    windows: Res<Windows>,
    mut ray_source_query: Query<(&mut RayCastSource, Option<&Camera>, Option<&GlobalTransform>)>
) {

    let window = windows.get_primary().unwrap();
    for (mut ray_source, camera, camera_transform) in ray_source_query.iter_mut() {

        match ray_source.method {
            RayCastMethod::Screenspace => {

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

fn calc_intersections(
    source_query: Query<&RayCastSource>,
    castable_query: Query<&RayCastable>
) {

    for source in source_query.iter() {
        if let Some(ray) = &source.ray {
            for castable in castable_query.iter() {
                todo!("Check for intersections here")
            }
        }
    }

}


#[cfg(test)]
mod tests {

}