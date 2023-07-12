use std::f32::consts::{FRAC_PI_2, FRAC_PI_6};

use bevy::prelude::*;

use kesko_core::{
    bundle::{MeshPhysicBodyBundle, PhysicBodyBundle},
    interaction::{groups::GroupDynamic, multibody_selection::MultibodySelectionEvent},
    shape::Shape,
    transform::world_transform_from_joint_anchors,
};
use kesko_object_interaction::InteractiveBundle;
use kesko_physics::{
    joint::{fixed::FixedJoint, revolute::RevoluteJoint, JointMotorEvent, KeskoAxis, MotorCommand},
    mass::Mass,
    multibody::MultibodyRoot,
    rapier_extern::rapier::prelude as rapier,
    rigid_body::RigidBody,
};

use super::ControlDescription;

// entity names
const NAME: &str = "car";
const RIGHT_FRONT_WHEEL_TURN: &str = "right_front_wheel_y";
const LEFT_FRONT_WHEEL_TURN: &str = "left_front_wheel_y";
const RIGHT_FRONT_WHEEL: &str = "right_front_wheel_x";
const LEFT_FRONT_WHEEL: &str = "left_front_wheel_x";
const RIGHT_REAR_WHEEL: &str = "right_rear_wheel_x";
const LEFT_REAR_WHEEL: &str = "left_rear_wheel_x";
const LEFT_WALL: &str = "left_wall";
const RIGHT_WALL: &str = "right_wall";
const FRONT_WALL: &str = "front_wall";
const BACK_WALL: &str = "back_wall";

pub struct CarPlugin;
impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CarControlEvent>().add_systems(
            Update,
            (
                Car::enable_control_system,
                Car::send_car_control_events,
                Car::handle_car_control_system,
            ),
        );
    }
}

pub struct Car;
impl Car {
    pub fn spawn(
        commands: &mut Commands,
        material_body: Handle<StandardMaterial>,
        material_wheel: Handle<StandardMaterial>,
        transform: Transform,
        meshes: &mut Assets<Mesh>,
    ) -> Entity {
        let frame_width = 0.5;
        let frame_height = 0.1;
        let frame_length = 1.0;

        let half_frame_length = frame_length / 2.0;
        let half_frame_width = frame_width / 2.0;
        let half_frame_height = frame_height / 2.0;

        let wheel_radius = 0.18;
        let wheel_width = 0.08;
        let wheel_base = frame_width + 0.1;
        let wbh = wheel_base / 2.0 + 0.01;

        let wall_thickness = 0.05;
        let wall_height = 0.2;
        let half_wall_height = wall_height / 2.0;
        let half_wall_thick = wall_thickness / 2.0;

        let stiffness = 10.0;
        let damping = 10.0;

        let mass = 0.5;

        // Frame
        let frame = commands
            .spawn((
                MeshPhysicBodyBundle::from(
                    RigidBody::Dynamic,
                    Shape::Box {
                        x_length: frame_width,
                        y_length: frame_height,
                        z_length: frame_length,
                    },
                    material_body.clone(),
                    transform,
                    meshes,
                ),
                InteractiveBundle::<GroupDynamic>::default(),
                Mass { val: mass },
                Name::new(NAME),
                ControlDescription("Use the WASD keys to manoeuver the car".to_owned()),
            ))
            .id();

        // front wall
        let parent_anchor = Transform::from_translation(Vec3::new(
            0.0,
            half_frame_height,
            half_frame_length - half_wall_thick,
        ));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_wall_height, 0.0));
        let world_transform =
            world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
        commands.spawn((
            MeshPhysicBodyBundle::from(
                RigidBody::Dynamic,
                Shape::Box {
                    x_length: frame_width,
                    y_length: wall_height,
                    z_length: wall_thickness,
                },
                material_body.clone(),
                world_transform,
                meshes,
            ),
            FixedJoint::attach_to(frame)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor),
            Name::new(FRONT_WALL),
            InteractiveBundle::<GroupDynamic>::default(),
        ));

        // back wall
        let parent_anchor = Transform::from_translation(Vec3::new(
            0.0,
            half_frame_height,
            -(half_frame_length - half_wall_thick),
        ));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_wall_height, 0.0));
        let world_transform =
            world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
        commands.spawn((
            MeshPhysicBodyBundle::from(
                RigidBody::Dynamic,
                Shape::Box {
                    x_length: frame_width,
                    y_length: wall_height,
                    z_length: wall_thickness,
                },
                material_body.clone(),
                world_transform,
                meshes,
            ),
            FixedJoint::attach_to(frame)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor),
            Name::new(BACK_WALL),
            Mass { val: mass },
            InteractiveBundle::<GroupDynamic>::default(),
        ));

        // left wall
        let parent_anchor = Transform::from_translation(Vec3::new(
            half_frame_width - half_wall_thick,
            half_frame_height,
            0.0,
        ));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_wall_height, 0.0));
        let world_transform =
            world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
        commands.spawn((
            MeshPhysicBodyBundle::from(
                RigidBody::Dynamic,
                Shape::Box {
                    x_length: wall_thickness,
                    y_length: wall_height,
                    z_length: frame_length - 2.0 * wall_thickness,
                },
                material_body.clone(),
                world_transform,
                meshes,
            ),
            FixedJoint::attach_to(frame)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor),
            Name::new(LEFT_WALL),
            Mass { val: mass },
            InteractiveBundle::<GroupDynamic>::default(),
        ));

        // right wall
        let parent_anchor = Transform::from_translation(Vec3::new(
            -half_frame_width + half_wall_thick,
            half_frame_height,
            0.0,
        ));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_wall_height, 0.0));
        let world_transform =
            world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
        commands.spawn((
            MeshPhysicBodyBundle::from(
                RigidBody::Dynamic,
                Shape::Box {
                    x_length: wall_thickness,
                    y_length: wall_height,
                    z_length: frame_length - 2.0 * wall_thickness,
                },
                material_body,
                world_transform,
                meshes,
            ),
            FixedJoint::attach_to(frame)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor),
            Name::new(RIGHT_WALL),
            Mass { val: mass },
            InteractiveBundle::<GroupDynamic>::default(),
        ));

        // left front link
        let parent_anchor = Transform::from_translation(Vec3::new(wbh, 0.0, half_frame_length))
            .with_rotation(Quat::from_rotation_z(-FRAC_PI_2));
        let child_anchor = Transform::default();
        let world_transform =
            world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
        let left_front_link = commands
            .spawn((
                PhysicBodyBundle::from(
                    RigidBody::Dynamic,
                    Shape::Sphere {
                        radius: 0.01,
                        subdivisions: 5,
                    },
                    world_transform,
                ),
                RevoluteJoint::attach_to(frame)
                    .with_parent_anchor(parent_anchor)
                    .with_child_anchor(child_anchor)
                    .with_axis(KeskoAxis::X)
                    .with_motor_params(stiffness, 0.1)
                    .with_limits(Vec2::new(-FRAC_PI_6, FRAC_PI_6)),
                Mass { val: mass },
                Name::new(LEFT_FRONT_WHEEL_TURN),
            ))
            .id();

        // left front wheel
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.11, 0.0));
        let child_anchor = Transform::default();
        let world_transform =
            world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        commands.spawn((
            MeshPhysicBodyBundle::from(
                RigidBody::Dynamic,
                Shape::Cylinder {
                    radius: wheel_radius,
                    length: wheel_width,
                    resolution: 42,
                },
                material_wheel.clone(),
                world_transform,
                meshes,
            ),
            RevoluteJoint::attach_to(left_front_link)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::Y),
            Mass { val: mass },
            Name::new(LEFT_FRONT_WHEEL),
            InteractiveBundle::<GroupDynamic>::default(),
        ));

        // right front link
        let parent_anchor = Transform::from_translation(Vec3::new(-wbh, 0.0, half_frame_length))
            .with_rotation(Quat::from_rotation_z(FRAC_PI_2));
        let child_anchor = Transform::default();
        let world_transform =
            world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
        let right_front_link = commands
            .spawn((
                PhysicBodyBundle::from(
                    RigidBody::Dynamic,
                    Shape::Sphere {
                        radius: 0.01,
                        subdivisions: 5,
                    },
                    world_transform,
                ),
                RevoluteJoint::attach_to(frame)
                    .with_parent_anchor(parent_anchor)
                    .with_child_anchor(child_anchor)
                    .with_axis(KeskoAxis::X)
                    .with_motor_params(stiffness, 0.1)
                    .with_limits(Vec2::new(-FRAC_PI_6, FRAC_PI_6)),
                Mass { val: mass },
                InteractiveBundle::<GroupDynamic>::default(),
                Name::new(RIGHT_FRONT_WHEEL_TURN),
            ))
            .id();

        // right front wheel
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.11, 0.0));
        let child_anchor = Transform::default();
        let world_transform =
            world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        commands.spawn((
            MeshPhysicBodyBundle::from(
                RigidBody::Dynamic,
                Shape::Cylinder {
                    radius: wheel_radius,
                    length: wheel_width,
                    resolution: 42,
                },
                material_wheel.clone(),
                world_transform,
                meshes,
            ),
            RevoluteJoint::attach_to(right_front_link)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::Y),
            Mass { val: mass },
            Name::new(RIGHT_FRONT_WHEEL),
            InteractiveBundle::<GroupDynamic>::default(),
        ));

        // left back wheel
        let parent_anchor = Transform::from_translation(Vec3::new(wbh, 0.0, -half_frame_length))
            .with_rotation(Quat::from_rotation_z(-FRAC_PI_2));
        let child_anchor = Transform::default();
        commands.spawn((
            MeshPhysicBodyBundle::from(
                RigidBody::Dynamic,
                Shape::Cylinder {
                    radius: wheel_radius,
                    length: wheel_width,
                    resolution: 42,
                },
                material_wheel.clone(),
                world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor),
                meshes,
            ),
            RevoluteJoint::attach_to(frame)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::Y)
                .with_motor_params(0.0, damping),
            Mass { val: mass },
            InteractiveBundle::<GroupDynamic>::default(),
            Name::new(LEFT_REAR_WHEEL),
        ));

        // right back wheel
        let parent_anchor = Transform::from_translation(Vec3::new(-wbh, 0.0, -half_frame_length))
            .with_rotation(Quat::from_rotation_z(FRAC_PI_2));
        let child_anchor = Transform::default();
        commands.spawn((
            MeshPhysicBodyBundle::from(
                RigidBody::Dynamic,
                Shape::Cylinder {
                    radius: wheel_radius,
                    length: wheel_width,
                    resolution: 42,
                },
                material_wheel,
                world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor),
                meshes,
            ),
            RevoluteJoint::attach_to(frame)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::Y)
                .with_motor_params(0.0, damping),
            Mass { val: mass },
            InteractiveBundle::<GroupDynamic>::default(),
            Name::new(RIGHT_REAR_WHEEL),
        ));

        frame
    }

    fn enable_control_system(
        mut commands: Commands,
        mut selection_event_reader: EventReader<MultibodySelectionEvent>,
        names: Query<&Name>,
    ) {
        for event in selection_event_reader.iter() {
            match event {
                MultibodySelectionEvent::Selected(entity) => {
                    if let Ok(name) = names.get(*entity) {
                        if name.as_str() == NAME {
                            commands.entity(*entity).insert(CarController::default());
                        }
                    }
                }
                MultibodySelectionEvent::Deselected(entity) => {
                    if let Ok(name) = names.get(*entity) {
                        if name.as_str() == NAME {
                            commands.entity(*entity).remove::<CarController>();
                        }
                    }
                }
            }
        }
    }

    fn send_car_control_events(
        keys: Res<Input<KeyCode>>,
        mut car_event_writer: EventWriter<CarControlEvent>,
    ) {
        // velocity
        if keys.just_pressed(KeyCode::W) {
            car_event_writer.send(CarControlEvent::Velocity(CarVelocity::Forward));
        } else if keys.just_pressed(KeyCode::S) {
            car_event_writer.send(CarControlEvent::Velocity(CarVelocity::Backward));
        } else if keys.just_released(KeyCode::S) || keys.just_released(KeyCode::W) {
            car_event_writer.send(CarControlEvent::Velocity(CarVelocity::NoVelocity));
        }

        // steering
        if keys.just_pressed(KeyCode::A)
            || (keys.pressed(KeyCode::A) && keys.just_released(KeyCode::D))
        {
            car_event_writer.send(CarControlEvent::Direction(CarDirection::Left));
        } else if keys.just_pressed(KeyCode::D)
            || (keys.pressed(KeyCode::D) && keys.just_released(KeyCode::A))
        {
            car_event_writer.send(CarControlEvent::Direction(CarDirection::Right));
        } else if keys.just_released(KeyCode::D) || keys.just_released(KeyCode::A) {
            car_event_writer.send(CarControlEvent::Direction(CarDirection::NoAngle));
        }
    }

    #[allow(clippy::type_complexity)]
    fn handle_car_control_system(
        mut car_event_reader: EventReader<CarControlEvent>,
        mut joint_event_writer: EventWriter<JointMotorEvent>,
        query: Query<(&MultibodyRoot, &CarController), (With<CarController>, With<MultibodyRoot>)>,
    ) {
        if let Ok((car_body, controller)) = query.get_single() {
            for event in car_event_reader.iter() {
                match event {
                    CarControlEvent::Velocity(velocity) => {
                        let velocity = match velocity {
                            CarVelocity::Forward => controller.max_velocity,
                            CarVelocity::Backward => -controller.max_velocity,
                            CarVelocity::NoVelocity => 0.0,
                        };

                        if let Some(entity) = car_body.child_map.get(RIGHT_REAR_WHEEL) {
                            joint_event_writer.send(JointMotorEvent {
                                entity: *entity,
                                command: MotorCommand::VelocityRevolute {
                                    velocity: -velocity,
                                    damping: None,
                                },
                            });
                        }
                        if let Some(entity) = car_body.child_map.get(LEFT_REAR_WHEEL) {
                            joint_event_writer.send(JointMotorEvent {
                                entity: *entity,
                                command: MotorCommand::VelocityRevolute {
                                    velocity,
                                    damping: None,
                                },
                            });
                        }
                    }
                    CarControlEvent::Direction(direction) => {
                        let position = match direction {
                            CarDirection::Left => controller.max_turn_angle,
                            CarDirection::Right => -controller.max_turn_angle,
                            CarDirection::NoAngle => 0.0,
                        };

                        if let Some(entity) = car_body.child_map.get(LEFT_FRONT_WHEEL_TURN) {
                            joint_event_writer.send(JointMotorEvent {
                                entity: *entity,
                                command: MotorCommand::PositionRevolute {
                                    position: -position,
                                    stiffness: None,
                                    damping: None,
                                },
                            })
                        } else {
                            error!("Could not get {}", LEFT_FRONT_WHEEL_TURN);
                        }

                        if let Some(entity) = car_body.child_map.get(RIGHT_FRONT_WHEEL_TURN) {
                            joint_event_writer.send(JointMotorEvent {
                                entity: *entity,
                                command: MotorCommand::PositionRevolute {
                                    position,
                                    stiffness: None,
                                    damping: None,
                                },
                            })
                        } else {
                            error!("Could not get {}", RIGHT_FRONT_WHEEL_TURN);
                        }
                    }
                }
            }
        }
    }
}

/// Controller for the car model
/// Should hold the settings for sensitivity etc
#[derive(Component)]
struct CarController {
    max_velocity: rapier::Real,   // m/s
    max_turn_angle: rapier::Real, // rad
}

impl Default for CarController {
    fn default() -> Self {
        Self {
            max_velocity: 12.0,
            max_turn_angle: FRAC_PI_6 as rapier::Real,
        }
    }
}

#[derive(Debug)]
enum CarControlEvent {
    Velocity(CarVelocity),
    Direction(CarDirection),
}

#[derive(Debug)]
enum CarVelocity {
    Forward,
    Backward,
    NoVelocity,
}

#[derive(Debug)]
enum CarDirection {
    Left,
    Right,
    NoAngle,
}
