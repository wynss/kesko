use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    render::camera::Projection,
};

pub struct PanOrbitCameraPlugin;

impl Plugin for PanOrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(send_camera_mouse_events)
            .add_system(handle_camera_events)
            .add_event::<PanOrbitCameraEvents>();
    }
}

#[derive(Debug)]
enum PanOrbitCameraEvents {
    Orbit(Vec2),
    Pan(Vec2),
    Zoom(f32),
    Translate(Vec3),
}

#[derive(Component)]
pub struct PanOrbitCamera {
    /// center of the sphere the camera is orbiting around
    pub center: Vec3,
    /// distance to center for the sphere camera is orbiting around
    pub dist_to_center: f32,

    // orbit
    pub orbit_sensitivity: f32,
    pub orbit_button: MouseButton,
    pub orbit_vel: Vec2,
    pub orbit_max_vel: f32,
    pub orbit_acc: f32,
    pub orbit_friction: f32,

    // pan
    pub pan_sensitivity: f32,
    pub pan_button: MouseButton,
    pub pan_vel: Vec2,
    pub pan_acc: f32,
    pub pan_friction: f32,
    pub pan_max_vel: f32,

    // zoom
    pub zoom_sensitivity: f32,
    pub zoom_vel: f32,
    pub zoom_acc: f32,
    pub zoom_friction: f32,
    pub zoom_max_vel: f32,

    pub translation_vel: Vec3,
    pub translation_friction: f32,
    pub acceleration: f32,
    pub max_velocity: f32,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            center: Vec3::ZERO,
            dist_to_center: 1.0,

            orbit_sensitivity: 0.2,
            orbit_button: MouseButton::Right,
            orbit_vel: Vec2::ZERO,
            orbit_max_vel: 15.0,
            orbit_acc: 1.0,
            orbit_friction: 0.25,

            pan_sensitivity: 40.0,
            pan_button: MouseButton::Middle,
            pan_vel: Vec2::ZERO,
            pan_acc: 0.1,
            pan_friction: 0.2,
            pan_max_vel: 0.1,

            zoom_sensitivity: 10.0,
            zoom_vel: 0.0,
            zoom_acc: 0.7,
            zoom_friction: 0.08,
            zoom_max_vel: 1.5,

            translation_vel: Vec3::ZERO,
            translation_friction: 0.8,
            acceleration: 1.2,
            max_velocity: 20.0,
        }
    }
}

fn send_camera_mouse_events(
    mut events: EventWriter<PanOrbitCameraEvents>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_scroll_events: EventReader<MouseWheel>,
    mouse_button_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    query: Query<&PanOrbitCamera>,
) {
    if let Ok(camera) = query.get_single() {
        let mut translation = Vec3::ZERO;
        if key_input.pressed(KeyCode::Up) {
            translation.z += 1.0;
        }
        if key_input.pressed(KeyCode::Down) {
            translation.z -= 1.0;
        }
        if key_input.pressed(KeyCode::Right) {
            translation.x += 1.0;
        }
        if key_input.pressed(KeyCode::Left) {
            translation.x -= 1.0;
        }
        if key_input.pressed(KeyCode::Tab) {
            translation.y += 1.0;
        }
        if key_input.pressed(KeyCode::LShift) {
            translation.y -= 1.0;
        }

        if translation != Vec3::ZERO {
            events.send(PanOrbitCameraEvents::Translate(translation));
        }

        // mouse motion
        let mut motion_delta = Vec2::ZERO;
        for event in mouse_motion_events.iter() {
            motion_delta += event.delta;
        }
        if motion_delta != Vec2::ZERO {
            if mouse_button_input.pressed(camera.orbit_button) {
                events.send(PanOrbitCameraEvents::Orbit(motion_delta));
            } else if mouse_button_input.pressed(camera.pan_button) {
                events.send(PanOrbitCameraEvents::Pan(motion_delta));
            }
        }

        // mouse scroll
        let mut scroll_delta = 0.0;
        for event in mouse_scroll_events.iter() {
            scroll_delta += event.y;
        }
        if scroll_delta.abs() > 0.0 {
            events.send(PanOrbitCameraEvents::Zoom(scroll_delta));
        }
    }
}

fn handle_camera_events(
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera_events: EventReader<PanOrbitCameraEvents>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
) {
    let window = windows.get_single().unwrap();
    let window_size = Vec2::new(window.width(), window.height());

    if let Ok((mut camera, mut transform, projection)) = query.get_single_mut() {
        let projection = match projection {
            Projection::Perspective(proj) => proj,
            _ => {
                error!("Could not get projection from main camera");
                return;
            }
        };

        let dt = time.delta_seconds();

        let mut orbit_updated = false;
        let mut zoom_updated = false;
        let mut pan_updated = false;
        let mut translation_updated = false;

        // update velocities based on
        for event in camera_events.iter() {
            match event {
                PanOrbitCameraEvents::Orbit(mouse_move) => {
                    // get rotation that should be performed around x and y axes
                    orbit_updated = true;
                    let acc = camera.orbit_acc;
                    camera.orbit_vel += mouse_move.normalize() * acc;
                    camera.orbit_vel = camera.orbit_vel.clamp_length_max(camera.orbit_max_vel);
                }
                PanOrbitCameraEvents::Pan(mut mouse_move) => {
                    pan_updated = true;
                    // make panning distance independent of resolution and FOV
                    mouse_move *=
                        Vec2::new(projection.fov * projection.aspect_ratio, projection.fov)
                            / window_size;
                    let acc = camera.pan_acc;
                    camera.pan_vel += mouse_move * acc;
                    camera.pan_vel = camera.pan_vel.clamp_length_max(camera.pan_max_vel);
                }
                PanOrbitCameraEvents::Zoom(scroll_move) => {
                    zoom_updated = true;
                    camera.zoom_vel += camera.zoom_acc * scroll_move.signum();
                    camera.zoom_vel = camera
                        .zoom_vel
                        .clamp(-camera.zoom_max_vel, camera.zoom_max_vel);
                }
                PanOrbitCameraEvents::Translate(translation) => {
                    translation_updated = true;
                    let acceleration = camera.acceleration;
                    camera.translation_vel += translation.normalize() * acceleration;
                    let new_velocity = camera.translation_vel.clamp_length_max(camera.max_velocity);
                    camera.translation_vel = new_velocity;
                }
            }
        }

        // handle orbit update
        if camera.orbit_vel.length() < 1e-6 {
            camera.orbit_vel = Vec2::ZERO;
        }

        if camera.orbit_vel != Vec2::ZERO {
            if !orbit_updated {
                let friction = camera.orbit_friction;
                camera.orbit_vel *= 1.0 - friction;
            }
            let rot_around_y = camera.orbit_sensitivity * dt * camera.orbit_vel.x;
            let rot_around_x = camera.orbit_sensitivity * dt * camera.orbit_vel.y;

            transform.rotation = {
                Quat::from_rotation_y(-rot_around_y)
                    * transform.rotation
                    * Quat::from_rotation_x(-rot_around_x)
            };
        }

        // handle zoom update
        if camera.zoom_vel.abs() < 1e-6 {
            camera.zoom_vel = 0.0;
        }
        if camera.zoom_vel != 0.0 {
            if !zoom_updated {
                let friction = camera.zoom_friction;
                camera.zoom_vel *= 1.0 - friction;
            }
            camera.dist_to_center -= camera.zoom_sensitivity * camera.zoom_vel * dt;
            camera.dist_to_center = f32::max(camera.dist_to_center, 0.05);
        }

        // handle pan update
        if camera.pan_vel.length() < 1e-6 {
            camera.pan_vel = Vec2::ZERO;
        }
        if camera.pan_vel != Vec2::ZERO {
            if !pan_updated {
                let friction = camera.pan_friction;
                camera.pan_vel *= 1.0 - friction;
            }
            // translate by local camera axes
            let pan_right = transform.rotation * Vec3::X * camera.pan_vel.x;
            let pan_up = transform.rotation * Vec3::Y * camera.pan_vel.y;

            let translation = (pan_up - pan_right)
                * camera.dist_to_center  // make panning proportional to distance away from focus point
                * dt
                * camera.pan_sensitivity;
            camera.center += translation;
        }

        // handle translation
        if camera.translation_vel.length_squared() < 1e-6 {
            camera.translation_vel = Vec3::ZERO;
        }
        if camera.translation_vel != Vec3::ZERO {
            if !translation_updated {
                let friction = camera.translation_friction;
                camera.translation_vel *= 1.0 - friction;
            }
            let right = transform.right();
            let forward = transform.forward();
            let translation = camera.translation_vel.x * dt * right
                + camera.translation_vel.y * dt * Vec3::Y
                + camera.translation_vel.z * dt * forward;
            camera.center += translation;
        }

        // translate so camera always is on the sphere centered around camera.center with radius camera.dist_to_center
        transform.translation =
            camera.center + (transform.rotation * Vec3::Z) * camera.dist_to_center;
    } else {
        error!("Can only have one pan orbit camera, found more than one");
    }
}
