use bevy::prelude::*;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub(crate) fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub(crate) fn transform(&self, transform: &Mat4) -> Self {
        Self {
            origin: transform.transform_point3(self.origin),
            direction: transform.transform_vector3(self.direction).normalize(),
        }
    }

    pub(crate) fn from_screen_space(
        camera: &Camera,
        camera_transform: &GlobalTransform,
        cursor_position: Vec2,
    ) -> Option<Self> {
        match camera.viewport_to_world(camera_transform, cursor_position) {
            Some(ray) => {
                let ray = Ray::new(ray.origin, ray.direction);
                Some(ray)
            }
            None => None,
        }
    }
}
