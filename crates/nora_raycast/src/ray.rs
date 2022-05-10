use bevy::prelude::*;


pub(crate) struct Ray {
    origin: Vec3,
    direction: Vec3
}
impl Ray {

    fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize()
        }
    }

    pub(crate) fn from_screen_space(
        window: &Window,
        camera: &Camera,
        camera_transform: &GlobalTransform,
        cursor_position: Vec2
    ) -> Self
    {
        let window_size = Vec2::new(window.width(), window.height());
        let cursor_position_ndc = 2.0 * (cursor_position / window_size) - Vec2::ONE;

        // camera space -> NDC
        let projection = camera.projection_matrix;

        // camera space -> world
        let view = camera_transform.compute_matrix();

        // NDC -> camera space -> world
        let ndc_to_world = view * projection.inverse();

        // from near plane in camera space -> NDC
        let ndc_near = projection.project_point3(-Vec3::Z * camera.near).z;

        // cursor position on near plane
        let cursor_position_near = ndc_to_world.project_point3(cursor_position_ndc.extend(ndc_near));

        Self::new(cursor_position_near, cursor_position_near - camera_transform.translation)
    }

    fn from_world_space() -> Self {
        todo!()
    }
}