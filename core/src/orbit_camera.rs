use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;


pub struct PanOrbitCameraPlugin;

impl Plugin for PanOrbitCameraPlugin {
    fn build(&self, app: &mut App) {

        app.add_system(handle_camera_mouse_events)
            .add_system(handle_camera_events)
            .add_event::<PanOrbitCameraEvents>();
    }
}

#[derive(Debug)]
enum PanOrbitCameraEvents {
    Orbit(Vec2),
    Pan(Vec2),
    Zoom(f32)
}

#[derive(Component)]
struct PanOrbitCamera {
    pub center: Vec3,
    pub distance: f32,
    pub orbit_sensitivity: f32,
    pub pan_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub orbit_button: MouseButton,
    pub pan_button: MouseButton
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            center: Vec3::ZERO,
            distance: 5.0,
            orbit_sensitivity: 1.0,
            pan_sensitivity: 20.0,
            zoom_sensitivity: 0.05,
            orbit_button: MouseButton::Right,
            pan_button: MouseButton::Middle
        }
    }
}

pub fn spawn_camera(mut commands: Commands) {

    let camera_pos = Vec3::new(-2.0, 2.5, 5.0);
    let distance = camera_pos.length();

    let camera_transform = Transform::from_translation(camera_pos)
        .looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: camera_transform,
        ..Default::default()
    }).insert(PanOrbitCamera {
        distance,
        ..Default::default()
    });
}

fn handle_camera_mouse_events(
    mut events: EventWriter<PanOrbitCameraEvents>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_scroll_events: EventReader<MouseWheel>,
    mouse_button_input: Res<Input<MouseButton>>,
    query: Query<&PanOrbitCamera>
) {

    let mut motion_delta = Vec2::ZERO;
    let mut scroll_delta = 0.0;

    for event in mouse_motion_events.iter() {
        motion_delta += event.delta;
    }

    for event in mouse_scroll_events.iter() {
        scroll_delta += event.y;
    }

    if motion_delta.length() > 0.0 {
        for camera in query.iter() {
            if mouse_button_input.pressed(camera.orbit_button) {
                events.send(PanOrbitCameraEvents::Orbit(motion_delta));
            }
            else if mouse_button_input.pressed(camera.pan_button) {
                events.send(PanOrbitCameraEvents::Pan(motion_delta));
            }
        }
    }

    if scroll_delta.abs() > 0.0 {
        for _ in query.iter() {
            events.send(PanOrbitCameraEvents::Zoom(scroll_delta));
        }
    }

}

fn handle_camera_events(
    time: Res<Time>,
    windows: Res<Windows>,
    mut camera_events: EventReader<PanOrbitCameraEvents>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>
) {

    let window = windows.get_primary().unwrap();
    let window_size = Vec2::new(window.width(), window.height());

    for (mut camera, mut transform, projection) in query.iter_mut() {
        for event in camera_events.iter() {
            match event {
                PanOrbitCameraEvents::Orbit(mouse_move) => {

                    // get rotation that should be performed around x and y axes
                    let rot_around_y = mouse_move.x * camera.orbit_sensitivity * time.delta_seconds();
                    let rot_around_x = mouse_move.y * camera.orbit_sensitivity * time.delta_seconds();
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
                        * time.delta_seconds()
                        * camera.pan_sensitivity;

                    camera.center += translation;
                }
                PanOrbitCameraEvents::Zoom(scroll_move) => {
                    camera.distance -= camera.distance * camera.zoom_sensitivity * (*scroll_move);
                    camera.distance = f32::max(camera.distance, 0.05);
                }
            }

            // translate so camera always is on the sphere centered around camera.center with radius camera.distance
            transform.translation = camera.center + (transform.rotation * Vec3::Z) * camera.distance;
        }
    }
}
