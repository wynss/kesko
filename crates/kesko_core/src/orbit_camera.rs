use bevy::prelude::*;
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    render::camera::Projection
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
    Translate
}

#[derive(Component)]
pub struct PanOrbitCamera {
    pub center: Vec3,
    pub distance: f32,
    pub orbit_sensitivity: f32,
    pub pan_sensitivity: f32,
    pub zoom_sensitivity: f32,

    pub orbit_button: MouseButton,
    pub pan_button: MouseButton,

    pub velocity: Vec3,
    pub velocity_friction: f32,
    pub acceleration: f32,
    pub max_velocity: f32
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            center: Vec3::ZERO,
            distance: 1.0,
            orbit_sensitivity: 0.2,
            pan_sensitivity: 40.0,
            zoom_sensitivity: 10.0,
            orbit_button: MouseButton::Right,
            pan_button: MouseButton::Middle,
            velocity: Vec3::ZERO,
            velocity_friction: 0.8,
            acceleration: 1.2,
            max_velocity: 20.0
        }
    }
}

fn send_camera_mouse_events(
    mut events: EventWriter<PanOrbitCameraEvents>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_scroll_events: EventReader<MouseWheel>,
    mouse_button_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    mut query: Query<&mut PanOrbitCamera>
) {

    if let Ok(mut camera) = query.get_single_mut() {

        let mut key_motion = Vec3::ZERO;
        if key_input.pressed(KeyCode::Up) {
            key_motion.z += 1.0;
        }
        if key_input.pressed(KeyCode::Down) {
            key_motion.z -= 1.0;
        }
        if key_input.pressed(KeyCode::Right) {
            key_motion.x += 1.0;
        }
        if key_input.pressed(KeyCode::Left) {
            key_motion.x -= 1.0;
        }

        let friction = camera.velocity_friction.clamp(0.0, 1.0);
        if key_motion != Vec3::ZERO {
            let acceleration = camera.acceleration;
            camera.velocity += key_motion.normalize() * acceleration;
            let new_velocity =camera.velocity.clamp_length_max(camera.max_velocity);
            camera.velocity = new_velocity;
        } else {
            camera.velocity *= 1.0 - friction;
            if camera.velocity.length_squared() < 1e-6 {
                camera.velocity = Vec3::ZERO;
            }
        }
        if camera.velocity != Vec3::ZERO {
            events.send(PanOrbitCameraEvents::Translate );
        }

        // mouse motion
        let mut motion_delta = Vec2::ZERO;
        for event in mouse_motion_events.iter() {
            motion_delta += event.delta;
        }
        if motion_delta != Vec2::ZERO {
            if mouse_button_input.pressed(camera.orbit_button) {
                events.send(PanOrbitCameraEvents::Orbit(motion_delta));
            }
            else if mouse_button_input.pressed(camera.pan_button) {
                events.send(PanOrbitCameraEvents::Pan(motion_delta));
            }
        }

        // mouse scroll
        let mut scroll_delta = 0.0;
        for event in mouse_scroll_events.iter() {
            scroll_delta += event.y;
        }

        if scroll_delta.abs() > 0.0 {
            for _ in query.iter() {
                events.send(PanOrbitCameraEvents::Zoom(scroll_delta));
            }
        }
    }
}

fn handle_camera_events(
    time: Res<Time>,
    windows: Res<Windows>,
    mut camera_events: EventReader<PanOrbitCameraEvents>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>
) {

    let window = windows.get_primary().unwrap();
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
        for event in camera_events.iter() {
            match event {
                PanOrbitCameraEvents::Orbit(mouse_move) => {

                    // get rotation that should be performed around x and y axes
                    let rot_around_y = mouse_move.x * camera.orbit_sensitivity * dt;
                    let rot_around_x = mouse_move.y * camera.orbit_sensitivity * dt;
                    transform.rotation = {
                        Quat::from_rotation_y(-rot_around_y)
                            * transform.rotation
                            * Quat::from_rotation_x(-rot_around_x)
                    };
                }
                PanOrbitCameraEvents::Pan(mut mouse_move) => {

                    // make panning distance independent of resolution and FOV
                    mouse_move *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window_size;

                    // translate by local camera axes
                    let pan_right = transform.rotation * Vec3::X * mouse_move.x;
                    let pan_up = transform.rotation * Vec3::Y * mouse_move.y;

                    let translation = (pan_up - pan_right)
                        * camera.distance  // make panning proportional to distance away from focus point
                        * dt
                        * camera.pan_sensitivity;

                    camera.center += translation;
                }
                PanOrbitCameraEvents::Zoom(scroll_move) => {
                    camera.distance -= camera.distance * camera.zoom_sensitivity * (*scroll_move) * dt;
                    camera.distance = f32::max(camera.distance, 0.05);
                },
                PanOrbitCameraEvents::Translate => {
                    let right = transform.right();
                    let forward = transform.forward();
                    let translation = camera.velocity.x * dt * right + camera.velocity.y * dt * Vec3::Y + camera.velocity.z * dt * forward;
                    camera.center += translation;
                }
            }

            // translate so camera always is on the sphere centered around camera.center with radius camera.distance
            transform.translation = camera.center + (transform.rotation * Vec3::Z) * camera.distance;
        }

    } else {
        error!("Can only have one pan orbit camera, found more than one");
    }
}
