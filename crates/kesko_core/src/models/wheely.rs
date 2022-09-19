use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use kesko_physics::{
    rigid_body::{
        RigidBody, RigidBodyName
    }, 
    joint::{
        JointMotorEvent,
        MotorAction,
        Joint, 
        revolute::RevoluteJoint, 
        prismatic::PrismaticJoint, 
        fixed::FixedJoint
    },
    multibody::MultibodyRoot
};
use kesko_object_interaction::InteractiveBundle;
use crate::{
    shape::Shape,
    bundle::{
        MeshPhysicBodyBundle, PhysicBodyBundle
    },
    interaction::{
        groups::GroupDynamic, 
        multibody_selection::MultibodySelectionEvent
    },
    transform::get_world_transform,
    models::ControlDescription
};


const NAME: &str = "wheely";
const LEFT_WHEEL: &str = "left_wheel";
const RIGHT_WHEEL: &str = "right_wheel";
const ARM_LINK_1: &str = "arm_link_1";
const ARM_LINK_2: &str = "arm_link_2";

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
    wheel_velocity: f32,
    arm_velocity: f32,
    damping: f32,
}

impl Default for WheelyController {
    fn default() -> Self {
        Self {
            wheel_velocity: 10.0,
            arm_velocity: 1.0,
            damping: 100.0,
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

    pub fn spawn_wheely(
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        wheel_material: Handle<StandardMaterial>,
        transform: Transform,
        meshes: &mut Assets<Mesh>
    ) {

        let hh = BODY_HEIGHT / 2.0;
        let rh = BODY_RADIUS / 2.0 + 0.2;
        let wheel_offset = WHEEL_RADIUS / 4.0;

        let body = commands.spawn_bundle(MeshPhysicBodyBundle::from(RigidBody::Dynamic,
            Shape::Cylinder { radius: BODY_RADIUS, length: BODY_HEIGHT, resolution: 21 } , material.clone(), transform, meshes
        ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName(NAME.to_owned()))
        .insert(ControlDescription("Use following keys\nRight wheel: E-D\tLeft wheel: Q-A\tArm joint 1: R-F\tArm joint 2: T-G".to_owned()))
        .id();

        // left wheel
        let parent_anchor = Transform::from_translation(Vec3::new(rh, -hh + wheel_offset, 0.15))
            .with_rotation(Quat::from_rotation_z(-FRAC_PI_2));
        let child_anchor = Transform::default();
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Cylinder { radius: WHEEL_RADIUS, length: WHEEL_WIDTH, resolution: 21},
            wheel_material.clone(),
            get_world_transform(&transform, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(body, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName(LEFT_WHEEL.to_owned()));

        // right wheel
        let parent_anchor = Transform::from_translation(Vec3::new(-rh, -hh + wheel_offset, 0.15))
            .with_rotation(Quat::from_rotation_z(-FRAC_PI_2));
        let child_anchor = Transform::default();
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Cylinder { radius: WHEEL_RADIUS, length: WHEEL_WIDTH, resolution: 21},
            wheel_material.clone(),
            get_world_transform(&transform, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(body, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName(RIGHT_WHEEL.to_owned()));
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -hh, -rh));
        let child_anchor = Transform::default();
        let back_wheel_turn = commands.spawn_bundle(PhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: 0.01, subdivisions: 5 },
            get_world_transform(&transform, &parent_anchor, &child_anchor),
        ))
        .insert(Joint::new(body, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            stiffness: 1.0,
            damping: 0.1,
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("back_wheel_turn".to_owned()))
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -BACK_WHEEL_RADIUS - 0.01, -BACK_WHEEL_RADIUS))
            .with_rotation(Quat::from_rotation_z(FRAC_PI_2));
        let child_anchor = Transform::default();
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Cylinder { radius: BACK_WHEEL_RADIUS, length: BACK_WHEEL_WIDTH, resolution: 21},
            wheel_material,
            get_world_transform(&transform, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(back_wheel_turn, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("back_wheel".to_owned()));

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
        let arm_base = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box { x_length: 0.03, y_length: 0.5, z_length: 0.03},
            material.clone(),
            get_world_transform(&transform, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(body, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("arm_base".to_owned())).id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.25, -0.015));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, 0.25, 0.015));
        let arm_link_1 = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box { x_length: 0.03, y_length: 0.5, z_length: 0.03},
            material.clone(),
            get_world_transform(&transform, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(arm_base, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::X,
            stiffness: 0.1,
            limits: Some(Vec2::new(0.0, PI)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName(ARM_LINK_1.to_owned()))
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, 0.25, -0.015));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, 0.25, 0.015));
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Box { x_length: 0.03, y_length: 0.5, z_length: 0.03},
            material,
            get_world_transform(&transform, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(arm_link_1, PrismaticJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::NEG_Y,
            limits: Some(Vec2::new(0.0, 0.45)),
            stiffness: 0.1,
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName(ARM_LINK_2.to_owned()));

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
                            commands.entity(*entity).insert(WheelyController::default());
                        }
                    }
                },
                MultibodySelectionEvent::Deselected(entity) => {
                    if let Ok(name) = names.get(*entity) {
                        if name.0 == NAME {
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
                let (entity, action) = match event {
                    WheelyControlEvent::LeftWheelForward => {
                        let entity = root.joint_name_2_entity.get(LEFT_WHEEL).expect("");
                        let action = MotorAction::VelocityRevolute { 
                            velocity: controller.wheel_velocity, factor: controller.damping 
                        };
                        (entity, action)
                    },
                    WheelyControlEvent::LeftWheelBackward => {
                        let entity = root.joint_name_2_entity.get(LEFT_WHEEL).expect("");
                        let action = MotorAction::VelocityRevolute { 
                                    velocity: -controller.wheel_velocity, factor: controller.damping 
                        };
                        (entity, action)
                    },
                    WheelyControlEvent::LeftWheelStop => {
                        let entity = root.joint_name_2_entity.get(LEFT_WHEEL).expect("");
                        let action = MotorAction::VelocityRevolute { 
                            velocity: 0.0, factor: controller.damping 
                        };
                        (entity, action)
                    },
                    WheelyControlEvent::RightWheelForward => {
                        let entity = root.joint_name_2_entity.get(RIGHT_WHEEL).expect("");
                        let action = MotorAction::VelocityRevolute { 
                            velocity: controller.wheel_velocity, factor: controller.damping 
                        };
                        (entity, action)
                    },
                    WheelyControlEvent::RightWheelBackward => {
                        let entity = root.joint_name_2_entity.get(RIGHT_WHEEL).expect("");
                        let action = MotorAction::VelocityRevolute { 
                                    velocity: -controller.wheel_velocity, factor: controller.damping 
                        };
                        (entity, action)
                    },
                    WheelyControlEvent::RightWheelStop => {
                        let entity = root.joint_name_2_entity.get(RIGHT_WHEEL).expect("");
                        let action = MotorAction::VelocityRevolute { 
                            velocity: 0.0, factor: controller.damping 
                        };
                        (entity, action)
                    },
                    WheelyControlEvent::ArmLink1Pos => {
                        let entity = root.joint_name_2_entity.get(ARM_LINK_1).expect("");
                        let action = MotorAction::VelocityRevolute { 
                            velocity: controller.arm_velocity, factor: controller.damping 
                        };
                        (entity, action)
                    },
                    WheelyControlEvent::ArmLink1Neg => {
                        let entity = root.joint_name_2_entity.get(ARM_LINK_1).expect("");
                        let action = MotorAction::VelocityRevolute { 
                            velocity: -controller.arm_velocity, factor: controller.damping 
                        };
                        (entity, action)
                    },
                    WheelyControlEvent::ArmLink1Stop => {
                        let entity = root.joint_name_2_entity.get(ARM_LINK_1).expect("");
                        let action = MotorAction::VelocityRevolute { 
                            velocity: 0.0, factor: controller.damping 
                        };
                        (entity, action)
                    },
                    WheelyControlEvent::ArmLink2Pos => {
                        let entity = root.joint_name_2_entity.get(ARM_LINK_2).expect("");
                        let action = MotorAction::VelocityPrismatic { 
                            velocity: controller.arm_velocity, factor: controller.damping 
                        };
                        (entity, action)
                    },
                    WheelyControlEvent::ArmLink2Neg => {
                        let entity = root.joint_name_2_entity.get(ARM_LINK_2).expect("");
                        let action = MotorAction::VelocityPrismatic { 
                            velocity: -controller.arm_velocity, factor: controller.damping 
                        };
                        (entity, action)
                    },
                    WheelyControlEvent::ArmLink2Stop => {
                        let entity = root.joint_name_2_entity.get(ARM_LINK_2).expect("");
                        let action = MotorAction::VelocityPrismatic { 
                            velocity: 0.0, factor: controller.damping 
                        };
                        (entity, action)
                    },
                };
                joint_event_writer.send(JointMotorEvent { entity: *entity, action });
            }
        }
    }
}
