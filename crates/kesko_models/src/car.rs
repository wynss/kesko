use std::f32::consts::{FRAC_PI_2, FRAC_PI_6};

use bevy::prelude::*;

use kesko_physics::{
    rigid_body::{
        RigidBody,
        RigidBodyName
    },
    joint::{
        Joint,
        fixed::FixedJoint,
        revolute::RevoluteJoint,
        JointMotorEvent, MotorAction
    },
    multibody::MultibodyRoot, mass::Mass
};
use kesko_object_interaction::InteractiveBundle;
use kesko_core::{
    shape::Shape,
    bundle::{MeshPhysicBodyBundle, PhysicBodyBundle},
    interaction::{
        multibody_selection::MultibodySelectionEvent,
        groups::GroupDynamic
    },
    transform::get_world_transform,
};

use super::ControlDescription;


const NAME: &str = "car";
const RIGHT_FRONT_WHEEL_TURN: &str = "right_front_wheel_y";
const LEFT_FRONT_WHEEL_TURN: &str = "left_front_wheel_y";
const RIGHT_FRONT_WHEEL: &str = "right_front_wheel_x";
const LEFT_FRONT_WHEEL: &str = "left_front_wheel_x";
const RIGHT_REAR_WHEEL: &str = "right_rear_wheel_x";
const LEFT_REAR_WHEEL: &str = "left_rear_wheel_x";

pub struct CarPlugin;
impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<CarControlEvent>()
        .add_system(Car::enable_control_system)
        .add_system(Car::send_car_control_events)
        .add_system(Car::handle_car_control_system);
    }
}

pub struct Car;
impl Car {
    pub fn spawn_car(
        commands: &mut Commands,
        material_body: Handle<StandardMaterial>,
        material_wheel: Handle<StandardMaterial>,
        transform: Transform,
        meshes: &mut Assets<Mesh>
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
        let wbh = wheel_base / 2.0;

        let wall_thickness = 0.05;
        let wall_height = 0.2;
        let half_wall_height = wall_height / 2.0;
        let half_wall_thick = wall_thickness / 2.0;

        // Frame
        let frame = commands.spawn_bundle( MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box {x_length: frame_width, y_length: frame_height, z_length: frame_length},
            material_body.clone(),
            transform,
            meshes
        ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Mass { val: 1.0 })
        .insert(RigidBodyName(NAME.to_owned()))
        .insert(ControlDescription("Use the WASD keys to manoeuver the car".to_owned()))
        .id();
        
        // front wall
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, half_frame_height, half_frame_length - half_wall_thick));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_wall_height, 0.0));
        let world_transform = get_world_transform(&transform, &parent_anchor, &child_anchor);
        commands.spawn_bundle( MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box {x_length: frame_width, y_length: wall_height, z_length: wall_thickness},
            material_body.clone(),
            world_transform,
            meshes
        ))
        .insert(Joint::new(frame, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default());

        // back wall
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, half_frame_height, -(half_frame_length - half_wall_thick)));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_wall_height, 0.0));
        let world_transform = get_world_transform(&transform, &parent_anchor, &child_anchor);
        commands.spawn_bundle( MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box {x_length: frame_width, y_length: wall_height, z_length: wall_thickness},
            material_body.clone(),
            world_transform,
            meshes
        ))
        .insert(Joint::new(frame, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default());

        // left wall
        let parent_anchor = Transform::from_translation(Vec3::new(half_frame_width - half_wall_thick, half_frame_height, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_wall_height, 0.0));
        let world_transform = get_world_transform(&transform, &parent_anchor, &child_anchor);
        commands.spawn_bundle( MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box {x_length: wall_thickness, y_length: wall_height, z_length: frame_length - 2.0 * wall_thickness},
            material_body.clone(),
            world_transform,
            meshes
        ))
        .insert(Joint::new(frame, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default());

        // right wall
        let parent_anchor = Transform::from_translation(Vec3::new(-half_frame_width + half_wall_thick, half_frame_height, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_wall_height, 0.0));
        let world_transform = get_world_transform(&transform, &parent_anchor, &child_anchor);
        commands.spawn_bundle( MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box {x_length: wall_thickness, y_length: wall_height, z_length: frame_length - 2.0 * wall_thickness},
            material_body,
            world_transform,
            meshes
        ))
        .insert(Joint::new(frame, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default());


        // left front link
        let parent_anchor = Transform::from_translation(Vec3::new(wbh, 0.0, half_frame_length))
            .with_rotation(Quat::from_rotation_z(-FRAC_PI_2));
        let child_anchor = Transform::default();
        let left_front_link = commands.spawn_bundle(PhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: 0.01, subdivisions: 5},
            get_world_transform(&transform, &parent_anchor, &child_anchor),
        ))
        .insert(Joint::new(frame, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::X,
            limits: Some(Vec2::new(-FRAC_PI_6, FRAC_PI_6)),
            stiffness: 1.0,
            ..default()
        }))
        .insert(RigidBodyName(LEFT_FRONT_WHEEL_TURN.to_owned()))
        .id();

        // left front wheel
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.1, 0.0));
        let child_anchor = Transform::default();
        commands.spawn_bundle( MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Cylinder { radius: wheel_radius, length: wheel_width, resolution: 21},
            material_wheel.clone(),
            get_world_transform(&transform, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(left_front_link, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            ..default()
        }))
        .insert(RigidBodyName(LEFT_FRONT_WHEEL.to_owned()))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default());

        // right front link
        let parent_anchor = Transform::from_translation(Vec3::new(-wbh, 0.0, half_frame_length))
            .with_rotation(Quat::from_rotation_z(FRAC_PI_2));
        let child_anchor = Transform::default();
        let right_front_link = commands.spawn_bundle(PhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: 0.01, subdivisions: 5},
            get_world_transform(&transform, &parent_anchor, &child_anchor),
        ))
        .insert(Joint::new(frame, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::X,
            limits: Some(Vec2::new(-FRAC_PI_6, FRAC_PI_6)),
            stiffness: 1.0,
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName(RIGHT_FRONT_WHEEL_TURN.to_owned()))
        .id();

        // right front wheel
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.1, 0.0));
        let child_anchor = Transform::default();
        commands.spawn_bundle( MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Cylinder { radius: wheel_radius, length: wheel_width, resolution: 21},
            material_wheel.clone(),
            get_world_transform(&transform, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(right_front_link, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            ..default()
        }))
        .insert(RigidBodyName(RIGHT_FRONT_WHEEL.to_owned()))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default());

        // left back wheel
        let parent_anchor = Transform::from_translation(Vec3::new(wbh, 0.0, -half_frame_length))
            .with_rotation(Quat::from_rotation_z(-FRAC_PI_2));
        let child_anchor = Transform::default();
        commands.spawn_bundle( MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Cylinder { radius: wheel_radius, length: wheel_width, resolution: 21},        material_wheel.clone(),
            get_world_transform(&transform, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(frame, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName(LEFT_REAR_WHEEL.to_owned()));

        // right back wheel
        let parent_anchor = Transform::from_translation(Vec3::new(-wbh, 0.0, -half_frame_length))
            .with_rotation(Quat::from_rotation_z(FRAC_PI_2));
        let child_anchor = Transform::default();
        commands.spawn_bundle( MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Cylinder { radius: wheel_radius, length: wheel_width, resolution: 21},
            material_wheel,
            get_world_transform(&transform, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(frame, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName(RIGHT_REAR_WHEEL.to_owned()));

        frame

    }
    
    fn enable_control_system(
        mut commands: Commands,
        mut selection_event_reader: EventReader<MultibodySelectionEvent>,
        names: Query<&RigidBodyName>
    ) {
        for event in selection_event_reader.iter() {
            match event {
                MultibodySelectionEvent::Selected(entity) => {
                    if let Ok(name) = names.get(*entity) {
                        if name.0 == NAME {
                            commands.entity(*entity).insert(CarController::default());
                        }
                    }
                },
                MultibodySelectionEvent::Deselected(entity) => {
                    if let Ok(name) = names.get(*entity) {
                        if name.0 == NAME {
                            commands.entity(*entity).remove::<CarController>();
                        }
                    }
                }
            }
        }
    }
    
    fn send_car_control_events(
        keys: Res<Input<KeyCode>>,
        mut car_event_writer: EventWriter<CarControlEvent>
    ) {

        // velocity
        if keys.just_pressed(KeyCode::W) {
            car_event_writer.send(CarControlEvent::Velocity(CarVelocity::Forward));
        } else if keys.just_pressed(KeyCode::S) {
            car_event_writer.send(CarControlEvent::Velocity(CarVelocity::Backward));
        } else if keys.just_released(KeyCode::S) || keys.just_released(KeyCode::W){
            car_event_writer.send(CarControlEvent::Velocity(CarVelocity::NoVelocity));
        } 
        
        // steering 
        if keys.just_pressed(KeyCode::A) || (keys.pressed(KeyCode::A) && keys.just_released(KeyCode::D)) {
            car_event_writer.send(CarControlEvent::Direction(CarDirection::Left));
        } else if keys.just_pressed(KeyCode::D) || (keys.pressed(KeyCode::D) && keys.just_released(KeyCode::A)) {
            car_event_writer.send(CarControlEvent::Direction(CarDirection::Right));
        } else if keys.just_released(KeyCode::D) || keys.just_released(KeyCode::A) {
            car_event_writer.send(CarControlEvent::Direction(CarDirection::NoAngle));
        }
    }

    #[allow(clippy::type_complexity)]
    fn handle_car_control_system(
        mut car_event_reader: EventReader<CarControlEvent>,
        mut joint_event_writer: EventWriter<JointMotorEvent>,
        query: Query<(&MultibodyRoot, &CarController), (With<CarController>, With<MultibodyRoot>)>
    ) {

        if let Ok((car_body, controller)) = query.get_single() {
            for event in car_event_reader.iter() {
                match event {
                    CarControlEvent::Velocity(velocity) => {

                        let (velocity, factor) = match velocity {
                            CarVelocity::Forward => (controller.max_velocity, controller.damping),
                            CarVelocity::Backward => (-controller.max_velocity, controller.damping),
                            CarVelocity::NoVelocity => (0.0, controller.damping)
                        };

                        if let Some(entity) = car_body.joint_name_2_entity.get(RIGHT_REAR_WHEEL) {
                            joint_event_writer.send(JointMotorEvent {
                                entity: *entity,
                                action: MotorAction::VelocityRevolute { velocity: -velocity, factor }
                            });
                        }
                        if let Some(entity) = car_body.joint_name_2_entity.get(LEFT_REAR_WHEEL) {
                            joint_event_writer.send(JointMotorEvent {
                                entity: *entity,
                                action: MotorAction::VelocityRevolute { velocity, factor }
                            });
                        }
                    },
                    CarControlEvent::Direction(direction) => {

                        let (position, damping, stiffness) = match direction {
                            CarDirection::Left => {
                                (controller.max_turn_angle, controller.damping, controller.stiffness)
                            },
                            CarDirection::Right => {
                                (-controller.max_turn_angle, controller.damping, controller.stiffness)
                            },
                            CarDirection::NoAngle => {
                                (0.0, controller.damping, controller.stiffness)
                            }
                        };

                        if let Some(entity) = car_body.joint_name_2_entity.get(LEFT_FRONT_WHEEL_TURN) {
                            joint_event_writer.send(JointMotorEvent { 
                                entity: *entity,
                                action: MotorAction::PositionRevolute { position: -position, damping, stiffness } })
                        } else {
                            error!("Could not get {}", LEFT_FRONT_WHEEL_TURN);
                        }

                        if let Some(entity) = car_body.joint_name_2_entity.get(RIGHT_FRONT_WHEEL_TURN) {
                            joint_event_writer.send(JointMotorEvent { 
                                entity: *entity,
                                action: MotorAction::PositionRevolute { position, damping, stiffness } })
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
    max_velocity: f32,      // m/s
    max_turn_angle: f32,    // rad
    damping: f32,
    stiffness: f32
}

impl Default for CarController {
    fn default() -> Self {
       Self {
            max_velocity: 12.0,
            max_turn_angle: FRAC_PI_6,
            damping: 0.1,
            stiffness: 1.0
       } 
    }
}

#[derive(Debug)]
enum CarControlEvent {
    Velocity(CarVelocity),
    Direction(CarDirection)
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
    NoAngle
}
