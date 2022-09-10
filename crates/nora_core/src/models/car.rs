use std::f32::consts::{FRAC_PI_2, FRAC_PI_6};

use bevy::prelude::*;

use nora_physics::{
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
    multibody::MultibodyRoot
};
use nora_object_interaction::InteractiveBundle;
use crate::{
    shape::Shape,
    bundle::{MeshPhysicBodyBundle, PhysicBodyBundle},
    interaction::groups::GroupDynamic,
    transform::get_world_transform,
};


const RIGHT_FRONT_WHEEL: &str = "right_front_wheel";
const LEFT_FRONT_WHEEL: &str = "left_front_wheel";
const RIGHT_REAR_WHEEL: &str = "right_rear_wheel";
const LEFT_REAR_WHEEL: &str = "left_rear_wheel";

pub struct CarPlugin;
impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<CarEvent>()
        .add_startup_system(setup)
        .add_system(send_car_control_events)
        .add_system(handle_car_control_system);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    let root = spawn_car(
        &mut commands, 
        materials.add(Color::SEA_GREEN.into()), 
        materials.add(Color::DARK_GRAY.into()),
        Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
        &mut meshes
    );

    commands.entity(root).insert(CarController {
        max_velocity: 10.0,
        max_turn_angle: FRAC_PI_6,
        damping: 0.1,
        stiffness: 1.0
    });
}


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
    .insert(RigidBodyName("Car".to_owned()))
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
        stiffness: 1.0,
        ..default()
    }))
    .insert(RigidBodyName(LEFT_FRONT_WHEEL.to_owned()))
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
        stiffness: 1.0,
        ..default()
    }))
    .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
    .insert(RigidBodyName(RIGHT_FRONT_WHEEL.to_owned()))
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

/// Controller for the car model
/// Should hold the settings for sensitivity etc
#[derive(Component)]
struct CarController {
    max_velocity: f32,      // m/s
    max_turn_angle: f32,    // rad
    damping: f32,
    stiffness: f32
}

#[derive(Debug)]
enum CarEvent {
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

fn send_car_control_events(
    keys: Res<Input<KeyCode>>,
    mut car_event_writer: EventWriter<CarEvent>
) {
    if keys.just_pressed(KeyCode::W) {
        car_event_writer.send(CarEvent::Velocity(CarVelocity::Forward));
    } else if keys.just_pressed(KeyCode::S) {
        car_event_writer.send(CarEvent::Velocity(CarVelocity::Backward));
    } else if keys.just_released(KeyCode::S) || keys.just_released(KeyCode::W){
        car_event_writer.send(CarEvent::Velocity(CarVelocity::NoVelocity));
    } 
    
    if keys.just_pressed(KeyCode::A) || (keys.pressed(KeyCode::A) && keys.just_released(KeyCode::D)) {
        car_event_writer.send(CarEvent::Direction(CarDirection::Left));
    } else if keys.just_pressed(KeyCode::D) || (keys.pressed(KeyCode::D) && keys.just_released(KeyCode::A)) {
        car_event_writer.send(CarEvent::Direction(CarDirection::Right));
    } else if keys.just_released(KeyCode::D) || keys.just_released(KeyCode::A) {
        car_event_writer.send(CarEvent::Direction(CarDirection::NoAngle));
    }
}

#[allow(clippy::type_complexity)]
fn handle_car_control_system(
    mut car_event_reader: EventReader<CarEvent>,
    mut joint_event_writer: EventWriter<JointMotorEvent>,
    query: Query<(&MultibodyRoot, &CarController), (With<CarController>, With<MultibodyRoot>)>
) {

    if let Ok((car_body, controller)) = query.get_single() {
        for event in car_event_reader.iter() {
            match event {
                CarEvent::Velocity(velocity) => {

                    let (velocity, factor) = match velocity {
                        CarVelocity::Forward => (controller.max_velocity, controller.damping),
                        CarVelocity::Backward => (-controller.max_velocity, controller.damping),
                        CarVelocity::NoVelocity => (0.0, controller.damping)
                    };

                    if let Some(entity) = car_body.joints.get(RIGHT_REAR_WHEEL) {
                        joint_event_writer.send(JointMotorEvent {
                            entity: *entity,
                            action: MotorAction::VelocityRevolute { velocity: -velocity, factor }
                        });
                    }
                    if let Some(entity) = car_body.joints.get(LEFT_REAR_WHEEL) {
                        joint_event_writer.send(JointMotorEvent {
                            entity: *entity,
                            action: MotorAction::VelocityRevolute { velocity, factor }
                        });
                    }
                },
                CarEvent::Direction(direction) => {

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

                    if let Some(entity) = car_body.joints.get(LEFT_FRONT_WHEEL) {
                        joint_event_writer.send(JointMotorEvent { 
                            entity: *entity,
                            action: MotorAction::PositionRevolute { position: -position, damping, stiffness } })
                    } else {
                        error!("Could not get {}", LEFT_FRONT_WHEEL);
                    }

                    if let Some(entity) = car_body.joints.get(RIGHT_FRONT_WHEEL) {
                        joint_event_writer.send(JointMotorEvent { 
                            entity: *entity,
                            action: MotorAction::PositionRevolute { position, damping, stiffness } })
                    } else {
                        error!("Could not get {}", RIGHT_FRONT_WHEEL);
                    }

                } 
            }
        }
    }
}
