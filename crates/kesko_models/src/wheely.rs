use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use kesko_physics::{
    rigid_body::RigidBody,
    joint::{
        JointMotorEvent,
        MotorCommand,
        KeskoAxis,
        revolute::RevoluteJoint, 
        prismatic::PrismaticJoint, 
        fixed::FixedJoint
    },
    multibody::MultibodyRoot, mass::Mass,
    rapier_extern::rapier::prelude as rapier,
};
use kesko_object_interaction::InteractiveBundle;
use kesko_core::{
    shape::Shape,
    bundle::{
        MeshPhysicBodyBundle, PhysicBodyBundle
    },
    interaction::{
        groups::GroupDynamic, 
        multibody_selection::MultibodySelectionEvent
    },
    transform::world_transform_from_joint_anchors,
};

use super::ControlDescription;

// entity names
const NAME: &str = "wheely";

const LEFT_WHEEL: &str = "left_wheel";
const RIGHT_WHEEL: &str = "right_wheel";
const REAR_WHEEL_TURN: &str = "rear wheel turn";
const REAR_WHEEL: &str = "rear wheel x";

const ARM_BASE: &str = "arm_base";
const ARM_LINK_1: &str = "arm_joint_1";
const ARM_LINK_2: &str = "arm_joint_2";

// dimensions
const BODY_RADIUS: f32 = 0.3;
const BODY_HEIGHT: f32 = 0.3;
const WHEEL_RADIUS: f32 = 0.20;
const WHEEL_WIDTH: f32 = 0.08;
const BACK_WHEEL_RADIUS: f32 = 0.07;
const BACK_WHEEL_WIDTH: f32 = 0.04;


pub struct WheelyPlugin;
impl Plugin for WheelyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<WheelyControlEvent>()
        .add_system(Wheely::enable_control_system)
        .add_system(Wheely::send_control_events)
        .add_system(Wheely::handle_control_system);
    }
}


#[derive(Component)]
struct WheelyController {
    wheel_velocity: rapier::Real,
    arm_link1_velocity: rapier::Real,
    arm_link2_velocity: rapier::Real
}

impl Default for WheelyController {
    fn default() -> Self {
        Self {
            wheel_velocity: 6.0,
            arm_link1_velocity: 3.0,
            arm_link2_velocity: 1.0
        }
    }
}

enum WheelyControlEvent {
    LeftWheelForward,
    LeftWheelBackward,
    LeftWheelStop,

    RightWheelForward,
    RightWheelBackward,
    RightWheelStop,

    ArmLink1Pos,
    ArmLink1Neg,
    ArmLink1Stop,

    ArmLink2Pos,
    ArmLink2Neg,
    ArmLink2Stop,
}

pub struct Wheely;
impl Wheely {

    pub fn spawn(
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        wheel_material: Handle<StandardMaterial>,
        transform: Transform,
        meshes: &mut Assets<Mesh>
    ) {

        let hh = BODY_HEIGHT / 2.0;
        let rh = BODY_RADIUS / 2.0 + 0.2;
        let wheel_offset = WHEEL_RADIUS / 4.0;

        let damping = 3.0;
        let stiffness = 0.0;

        let body = commands.spawn_bundle(MeshPhysicBodyBundle::from(RigidBody::Dynamic,
            Shape::Cylinder { radius: BODY_RADIUS, length: BODY_HEIGHT, resolution: 21 } , 
            material.clone(), 
            transform, 
            meshes
        ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Mass { val: 0.5 })
        .insert(Name::new(NAME))
        .insert(ControlDescription("Use following keys\nRight wheel: E-D\tLeft wheel: Q-A\tArm joint 1: R-F\tArm joint 2: T-G".to_owned()))
        .id();

        // left wheel
        let parent_anchor = Transform::from_translation(Vec3::new(rh, -hh + wheel_offset, 0.15))
            .with_rotation(Quat::from_rotation_z(-FRAC_PI_2));
        let child_anchor = Transform::default();
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Cylinder { radius: WHEEL_RADIUS, length: WHEEL_WIDTH, resolution: 42},
            wheel_material.clone(),
            world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(body)
            .with_parent_anchor(parent_anchor)
            .with_child_anchor(child_anchor)
            .with_axis(KeskoAxis::Y)
            .with_motor_params(stiffness, damping)
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Mass { val: 1.5 })
        .insert(Name::new(LEFT_WHEEL));

        // right wheel
        let parent_anchor = Transform::from_translation(Vec3::new(-rh, -hh + wheel_offset, 0.15))
            .with_rotation(Quat::from_rotation_z(-FRAC_PI_2));
        let child_anchor = Transform::default();
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Cylinder { radius: WHEEL_RADIUS, length: WHEEL_WIDTH, resolution: 42},
            wheel_material.clone(),
            world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(body)
            .with_parent_anchor(parent_anchor)
            .with_child_anchor(child_anchor)
            .with_axis(KeskoAxis::Y)
            .with_motor_params(stiffness, damping)
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Mass { val: 1.5 })
        .insert(Name::new(RIGHT_WHEEL));
        
        // back wheel turn link
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -hh, -rh));
        let child_anchor = Transform::default();
        let world_transform = world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
        let back_wheel_turn = commands.spawn_bundle(PhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: 0.01, subdivisions: 5 },
            world_transform,
        ))
        .insert(
            RevoluteJoint::attach_to(body)
            .with_parent_anchor(parent_anchor)
            .with_child_anchor(child_anchor)
            .with_axis(KeskoAxis::Y)
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Mass { val: 0.5 })
        .insert(Name::new(REAR_WHEEL_TURN))
        .id();
        
        // back wheel
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -BACK_WHEEL_RADIUS - 0.01, -BACK_WHEEL_RADIUS))
            .with_rotation(Quat::from_rotation_z(FRAC_PI_2));
        let child_anchor = Transform::default();
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Cylinder { radius: BACK_WHEEL_RADIUS, length: BACK_WHEEL_WIDTH, resolution: 21},
            wheel_material,
            world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(back_wheel_turn)
            .with_parent_anchor(parent_anchor)
            .with_child_anchor(child_anchor)
            .with_axis(KeskoAxis::Y)
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Mass{ val: 0.2 })
        .insert(Name::new(REAR_WHEEL));

        Self::build_arm(body, commands, material, transform, meshes);

    }

    fn build_arm(
        body: Entity,
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        transform: Transform,
        meshes: &mut Assets<Mesh>
    ) {

        let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.2, -BODY_RADIUS - 0.015));
        let child_anchor = Transform::default();
        let world_transform = world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
        let arm_base = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box { x_length: 0.03, y_length: 0.5, z_length: 0.03},
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            FixedJoint::attach_to(body)
            .with_parent_anchor(parent_anchor)
            .with_child_anchor(child_anchor)
        )
        .insert(Mass { val: 0.1 })
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(ARM_BASE)).id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.25, -0.015));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, 0.25, 0.015));
        let world_transform = world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        let arm_link_1 = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box { x_length: 0.03, y_length: 0.5, z_length: 0.03},
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(arm_base)
            .with_parent_anchor(parent_anchor)
            .with_child_anchor(child_anchor)
            .with_axis(KeskoAxis::X)
            .with_motor_params(10.0, 0.0)
            .with_limits(Vec2::new(0.0, PI - 0.001))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Mass { val: 0.1 })
        .insert(Name::new(ARM_LINK_1))
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.25, -0.015));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, 0.25, 0.015));
        let world_transform = world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box { x_length: 0.03, y_length: 0.5, z_length: 0.03},
            material,
            world_transform,
            meshes
        ))
        .insert(
            PrismaticJoint::attach_to(arm_link_1)
            .with_parent_anchor(parent_anchor)
            .with_child_anchor(child_anchor)
            .with_axis(KeskoAxis::NegY)
            .with_motor_params(10.0, 0.0)
            .with_limits(Vec2::new(0.0, 0.45))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Mass { val: 0.1 })
        .insert(Name::new(ARM_LINK_2));

    }

    fn enable_control_system(
        mut commands: Commands,
        mut selection_event_reader: EventReader<MultibodySelectionEvent>,
        names: Query<&Name>
    ) {
        for event in selection_event_reader.iter() {
            match event {
                MultibodySelectionEvent::Selected(entity) => {
                    if let Ok(name) = names.get(*entity) {
                        if name.as_str() == NAME {
                            commands.entity(*entity).insert(WheelyController::default());
                        }
                    }
                },
                MultibodySelectionEvent::Deselected(entity) => {
                    if let Ok(name) = names.get(*entity) {
                        if name.as_str() == NAME {
                            commands.entity(*entity).remove::<WheelyController>();
                        }
                    }
                }
            }
        }
    }

    fn send_control_events(
        keys: Res<Input<KeyCode>>,
        mut wheely_event_writer: EventWriter<WheelyControlEvent>
    ) {

        // left wheel
        if keys.just_pressed(KeyCode::Q) {
            wheely_event_writer.send(WheelyControlEvent::LeftWheelForward);
        } else if keys.just_pressed(KeyCode::A) {
            wheely_event_writer.send(WheelyControlEvent::LeftWheelBackward);
        } else if keys.just_released(KeyCode::Q) || keys.just_released(KeyCode::A){
            wheely_event_writer.send(WheelyControlEvent::LeftWheelStop);
        } 
        
        // right wheel
        if keys.just_pressed(KeyCode::E) {
            wheely_event_writer.send(WheelyControlEvent::RightWheelForward);
        } else if keys.just_pressed(KeyCode::D) {
            wheely_event_writer.send(WheelyControlEvent::RightWheelBackward);
        } else if keys.just_released(KeyCode::E) || keys.just_released(KeyCode::D){
            wheely_event_writer.send(WheelyControlEvent::RightWheelStop);
        } 

        // arm link 1
        if keys.just_pressed(KeyCode::R) {
            wheely_event_writer.send(WheelyControlEvent::ArmLink1Pos);
        } else if keys.just_pressed(KeyCode::F) {
            wheely_event_writer.send(WheelyControlEvent::ArmLink1Neg);
        } else if keys.just_released(KeyCode::R) || keys.just_released(KeyCode::F){
            wheely_event_writer.send(WheelyControlEvent::ArmLink1Stop);
        } 
        
        // arm link 2
        if keys.just_pressed(KeyCode::T) {
            wheely_event_writer.send(WheelyControlEvent::ArmLink2Pos);
        } else if keys.just_pressed(KeyCode::G) {
            wheely_event_writer.send(WheelyControlEvent::ArmLink2Neg);
        } else if keys.just_released(KeyCode::T) || keys.just_released(KeyCode::G){
            wheely_event_writer.send(WheelyControlEvent::ArmLink2Stop);
        } 
    }

    fn handle_control_system(
        mut wheely_event_reader: EventReader<WheelyControlEvent>,
        mut joint_event_writer: EventWriter<JointMotorEvent>,
        query: Query<(&MultibodyRoot, &WheelyController), (With<WheelyController>, With<MultibodyRoot>)>
    ) {
        if let Ok((root, controller)) = query.get_single() {
            for event in wheely_event_reader.iter() {
                match event {
                    WheelyControlEvent::LeftWheelForward => {
                        let entity = root.child_map.get(LEFT_WHEEL).expect("");
                        let action = MotorCommand::VelocityRevolute { velocity: controller.wheel_velocity, damping: None };
                        joint_event_writer.send(JointMotorEvent { entity: *entity, command: action });
                    },
                    WheelyControlEvent::LeftWheelBackward => {
                        let entity = root.child_map.get(LEFT_WHEEL).expect("");
                        let action = MotorCommand::VelocityRevolute { velocity: -controller.wheel_velocity, damping: None };
                        joint_event_writer.send(JointMotorEvent { entity: *entity, command: action });
                    },
                    WheelyControlEvent::LeftWheelStop => {
                        let entity = root.child_map.get(LEFT_WHEEL).expect("");
                        let action = MotorCommand::VelocityRevolute { velocity: 0.0, damping: None };
                        joint_event_writer.send(JointMotorEvent { entity: *entity, command: action });
                    },
                    WheelyControlEvent::RightWheelForward => {
                        let entity = root.child_map.get(RIGHT_WHEEL).expect("");
                        let action = MotorCommand::VelocityRevolute { velocity: controller.wheel_velocity, damping: None };
                        joint_event_writer.send(JointMotorEvent { entity: *entity, command: action });
                    },
                    WheelyControlEvent::RightWheelBackward => {
                        let entity = root.child_map.get(RIGHT_WHEEL).expect("");
                        let action = MotorCommand::VelocityRevolute { velocity: -controller.wheel_velocity, damping: None };
                        joint_event_writer.send(JointMotorEvent { entity: *entity, command: action });
                    },
                    WheelyControlEvent::RightWheelStop => {
                        let entity = root.child_map.get(RIGHT_WHEEL).expect("");
                        let action = MotorCommand::VelocityRevolute { velocity: 0.0, damping: None };
                        joint_event_writer.send(JointMotorEvent { entity: *entity, command: action });
                    },
                    WheelyControlEvent::ArmLink1Pos => {
                        let entity = root.child_map.get(ARM_LINK_1).expect("");
                        let action = MotorCommand::VelocityRevolute { velocity: controller.arm_link1_velocity, damping: None };
                        joint_event_writer.send(JointMotorEvent { entity: *entity, command: action });
                    },
                    WheelyControlEvent::ArmLink1Neg => {
                        let entity = root.child_map.get(ARM_LINK_1).expect("");
                        let action = MotorCommand::VelocityRevolute { velocity: -controller.arm_link1_velocity, damping: None };
                        joint_event_writer.send(JointMotorEvent { entity: *entity, command: action });
                    },
                    WheelyControlEvent::ArmLink1Stop => {
                        let entity = root.child_map.get(ARM_LINK_1).expect("");
                        let action = MotorCommand::HoldPosition { stiffness: Some(1.0) };
                        joint_event_writer.send(JointMotorEvent { entity: *entity, command: action });
                    },
                    WheelyControlEvent::ArmLink2Pos => {
                        let entity = root.child_map.get(ARM_LINK_2).expect("");
                        let action = MotorCommand::VelocityPrismatic { velocity: controller.arm_link2_velocity, damping: None };
                        joint_event_writer.send(JointMotorEvent { entity: *entity, command: action });
                    },
                    WheelyControlEvent::ArmLink2Neg => {
                        let entity = root.child_map.get(ARM_LINK_2).expect("");
                        let action = MotorCommand::VelocityPrismatic { velocity: -controller.arm_link2_velocity, damping: None };
                        joint_event_writer.send(JointMotorEvent { entity: *entity, command: action });
                    },
                    WheelyControlEvent::ArmLink2Stop => {
                        let entity = root.child_map.get(ARM_LINK_2).expect("");
                        let action = MotorCommand::HoldPosition { stiffness: Some(1.0) };
                        joint_event_writer.send(JointMotorEvent { entity: *entity, command: action });
                    },
                }
            }
        }
    }
}
