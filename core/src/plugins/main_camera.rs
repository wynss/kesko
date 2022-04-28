use bevy::app::{App, Plugin};
use pan_orbit_camera::{handle_camera, spawn_camera};

pub(crate) struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(handle_camera);
    }

    fn name(&self) -> &str {
        "Main Camera Plugin"
    }
}

pub(super) mod pan_orbit_camera {

    use bevy::prelude::*;
    use bevy::input::mouse::{MouseMotion, MouseWheel};
    use bevy::render::camera::PerspectiveProjection;
    use crate::plugins::menu::OccupiedScreenSpaceByMenus;

    #[derive(Component)]
    pub struct PanOrbitCamera {
        pub focus: Vec3,
        pub radius: f32,
        pub upside_down: bool
    }

    impl Default for PanOrbitCamera {
        fn default() -> Self {
            PanOrbitCamera {
                focus: Vec3::ZERO,
                radius: 5.0,
                upside_down: false
            }
        }
    }

    pub fn spawn_camera(
        mut commands: Commands
    ) {

        let camera_pos = Vec3::new(-2.0, 2.5, 5.0);
        let radius = camera_pos.length();

        let camera_transform = Transform::from_translation(camera_pos)
            .looking_at(Vec3::ZERO, Vec3::Y);

        commands.spawn_bundle(PerspectiveCameraBundle {
            transform: camera_transform,
            ..Default::default()
        }).insert(PanOrbitCamera {
            radius,
            ..Default::default()
        });
    }

    pub fn handle_camera(
        windows: Res<Windows>,
        mut ev_motion: EventReader<MouseMotion>,
        mut ev_scroll: EventReader<MouseWheel>,
        input_mouse: Res<Input<MouseButton>>,
        occupied_screen_space_by_menus: Res<OccupiedScreenSpaceByMenus>,
        mut query: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>
    ) {

        let orbit_button = MouseButton::Right;
        let pan_button = MouseButton::Middle;

        let mut pan_move = Vec2::ZERO;
        let mut rotation_move = Vec2::ZERO;
        let mut zoom_move = 0.0;
        let mut orbit_button_changed = false;

        // gather all inputs
        if input_mouse.pressed(orbit_button) {
            for ev in ev_motion.iter() {
                rotation_move += ev.delta;
            }
        } else if input_mouse.pressed(pan_button) {
            for ev in ev_motion.iter() {
                pan_move += ev.delta;
            }
        }

        for ev in ev_scroll.iter() {
            zoom_move += ev.y;
        }

        if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
            orbit_button_changed = true;
        }

        for (mut pan_orbit, mut transform, projection) in query.iter_mut() {

            if orbit_button_changed {
                let up: Vec3 = transform.rotation * Vec3::Y;
                println!("{:?}", up);
                pan_orbit.upside_down = up.y <= 0.0;
            }

            let mut should_transform = false;
            if rotation_move.length_squared() > 0.0 {
                should_transform = true;
                let window = get_primary_window_size(&windows);
                let delta_x = {
                    let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                    if pan_orbit.upside_down { -delta } else { delta }
                };
                let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
                let yaw = Quat::from_rotation_y(-delta_x);
                let pitch = Quat::from_rotation_x(-delta_y);
                transform.rotation = yaw * transform.rotation; // rotate around global y axis
                transform.rotation = transform.rotation * pitch; // rotate around local x axis
            } else if pan_move.length_squared() > 0.0 {
                should_transform = true;
                // make panning distance independent of resolution and FOV,
                let window = get_primary_window_size(&windows);
                pan_move *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
                // translate by local axes
                let right = transform.rotation * Vec3::X * -pan_move.x;
                let up = transform.rotation * Vec3::Y * pan_move.y;
                // make panning proportional to distance away from focus point
                let translation = (right + up) * pan_orbit.radius;
                pan_orbit.focus += translation;
            } else if zoom_move.abs() > 0.0 {
                should_transform = true;
                pan_orbit.radius -= zoom_move * pan_orbit.radius * 0.2;
                // dont allow zoom to reach zero or you get stuck
                pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
            }

            if should_transform {
                // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
                // parent = x and y rotation
                // child = z-offset
                let rot_matrix = Mat3::from_quat(transform.rotation);
                transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
            }
        }
    }

    fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
        let window = windows.get_primary().unwrap();
        let window = Vec2::new(window.width() as f32, window.height() as f32);
        window
    }
}
