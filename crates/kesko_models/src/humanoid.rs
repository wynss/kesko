use std::f32::consts::{FRAC_PI_2, FRAC_PI_6, FRAC_PI_4, PI};

use bevy::prelude::*;

use kesko_physics::{
    rigid_body::RigidBody,
    joint::{
        KeskoAxis,
        revolute::RevoluteJoint, 
        fixed::FixedJoint
    }, 
    mass::Mass,
    rapier_extern::rapier::prelude as rapier
};
use kesko_object_interaction::InteractiveBundle;
use kesko_core::{
    shape::Shape,
    bundle::{
        MeshPhysicBodyBundle,
        PhysicBodyBundle
    },
    interaction::groups::GroupDynamic,
    transform::world_transform_from_joint_anchors
};

use super::Model;

// For some reason we need to set the mass of the bodies, otherwise it behaves like there is almost
// no gravity, seems like a bug in Rapier. For now use the same mass for all bodies
const MASS: rapier::Real = 0.1;

const STIFFNESS: rapier::Real = 2.0;
const DAMPING: rapier::Real = 0.3;

const HEAD_RADIUS: f32 = 0.13;
const NECK_LENGTH: f32 = 0.13;

const SHOULDER_WIDTH: f32 = 0.28;
const SHOULDER_RADIUS: f32 = 0.04;

const ARM_RADIUS: f32 = 0.035;
const ARM_UPPER_LENGTH: f32 = 0.24;
const ARM_LOWER_LENGTH: f32 = 0.18;

const LEG_RADIUS: f32 = 0.04;
const UPPER_LEG_LENGTH: f32 = 0.3;
const LOWER_LEG_LENGTH: f32 = 0.25;
const FOOT_LENGTH: f32 = 0.15;

// Entity names
const NECK_X: &str = "neck_x";
const NECK_Y: &str = "neck_y";
const NECK_Z: &str = "neck_z";

const TORSO_1: &str = "torso_1";
const TORSO_2: &str = "torso_2";
const TORSO_3: &str = "torso_3";

const LEFT_SHOULDER_Z: &str = "left_shoulder_z";
const LEFT_SHOULDER_X: &str = "left_shoulder_x";
const LEFT_ELBOW_X: &str = "left_elbow_x";
const RIGHT_SHOULDER_Z: &str = "right_shoulder_z";
const RIGHT_SHOULDER_X: &str = "right_shoulder_x";
const RIGHT_ELBOW_X: &str = "right_elbow_x";

const LEFT_HIP_Z: &str = "left_hip_z";
const LEFT_HIP_X: &str = "left_hip_x";
const LEFT_KNEE_X: &str = "left_knee_x";
const LEFT_FOOT_X: &str = "left_foot_x";

const RIGHT_HIP_Z: &str = "right_hip_z";
const RIGHT_HIP_X: &str = "right_hip_x";
const RIGHT_KNEE_X: &str = "right_knee_x";
const RIGHT_FOOT_X: &str = "right_foot_x";


pub struct Humanoid;
impl Humanoid {

    pub fn spawn(
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        transform: Transform,
        meshes: &mut Assets<Mesh>
    ) {

        let head = commands.spawn_bundle(MeshPhysicBodyBundle::from(RigidBody::Dynamic,
            Shape::Sphere { radius: HEAD_RADIUS, subdivisions: 7 }, 
            material.clone(), 
            transform, 
            meshes
        ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(Model::Humanoid.name().to_owned()))
        .insert(Mass {val: MASS})
        .id();

        let (shoulder, shoulder_transform) = Self::build_neck(head, commands, material.clone(), transform, meshes);
        let (hip, hip_transform) = Self::build_upper_body(shoulder, commands, material.clone(), shoulder_transform, meshes);

        Self::build_arms(shoulder, commands, material.clone(), shoulder_transform, meshes);
        Self::build_legs(hip, commands, material.clone(), hip_transform, meshes);
    }

    fn build_neck(
        head: Entity,
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        transform: Transform,
        meshes: &mut Assets<Mesh>
    ) -> (Entity, Transform) {
        
        // neck x
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -(HEAD_RADIUS + NECK_LENGTH / 4.0), 0.0));
        let child_anchor = Transform::default();
        let world_transform = world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
        let neck_x = commands.spawn_bundle(PhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: 0.01, subdivisions: 5 },
            world_transform,
        ))
        .insert(
            RevoluteJoint::attach_to(head)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::X)
                .with_motor_params(STIFFNESS, DAMPING)
                .with_limits(Vec2::new(-FRAC_PI_4, FRAC_PI_4))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Mass {val: MASS})
        .insert(Name::new(NECK_X))
        .id();

        // neck y
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -0.02, 0.0));
        let child_anchor = Transform::default();
        let world_transform = world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        let neck_y = commands.spawn_bundle(PhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: 0.01, subdivisions: 5 },
            world_transform,
        ))
        .insert(
            RevoluteJoint::attach_to(neck_x)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::Y)
                .with_motor_params(STIFFNESS, DAMPING)
                .with_limits(Vec2::new(-FRAC_PI_2, FRAC_PI_2))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Mass {val: MASS})
        .insert(Name::new(NECK_Y))
        .id();
        
        // neck z
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -NECK_LENGTH / 2.0, 0.0)).with_rotation(Quat::from_rotation_z(FRAC_PI_2));
        let child_anchor = Transform::default();
        let world_transform = world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        let shoulder = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: SHOULDER_RADIUS, length: SHOULDER_WIDTH },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(neck_y)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::Z)
                .with_motor_params(STIFFNESS, DAMPING)
                .with_limits(Vec2::new(-FRAC_PI_6, FRAC_PI_6))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(NECK_Z))
        .insert(Mass {val: MASS})
        .id();

        (shoulder, world_transform)

    }

    fn build_upper_body(
        shoulder: Entity,
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        transform: Transform,
        meshes: &mut Assets<Mesh>
    ) -> (Entity, Transform) {

        let should_dist = 3.0 * SHOULDER_RADIUS;

        let parent_anchor = Transform::from_translation(Vec3::new(-should_dist, 0.0, 0.0));
        let child_anchor = Transform::default();
        let world_transform = world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
        let body_1 = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: SHOULDER_RADIUS, length: 0.8*SHOULDER_WIDTH },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            FixedJoint::attach_to(shoulder)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(TORSO_1))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(-should_dist, 0.0, 0.0));
        let child_anchor = Transform::default();
        let world_transform = world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        let body_2 = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: SHOULDER_RADIUS, length: 0.6*SHOULDER_WIDTH },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            FixedJoint::attach_to(body_1)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(TORSO_2))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(-should_dist, 0.0, 0.0));
        let child_anchor = Transform::default();
        let world_transform = world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        let hip = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: SHOULDER_RADIUS, length: 0.5*SHOULDER_WIDTH },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            FixedJoint::attach_to(body_2)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(TORSO_3))
        .insert(Mass {val: MASS})
        .id();

        (hip, world_transform)

    }

    fn build_arms(
        shoulder: Entity,
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        transform: Transform,
        meshes: &mut Assets<Mesh>
    ) {
        
        let should_to_arm = SHOULDER_WIDTH / 2.0 + ARM_RADIUS + SHOULDER_RADIUS;
        let half_upper_arm = ARM_UPPER_LENGTH / 2.0 + ARM_RADIUS;
        let half_lower_arm = ARM_LOWER_LENGTH / 2.0 + ARM_RADIUS;

        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -should_to_arm, 0.0)).with_rotation(Quat::from_rotation_z(-FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let world_transform = world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
        let left_shoulder_z = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: ARM_RADIUS, subdivisions: 7 },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(shoulder)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::Z)
                .with_motor_params(STIFFNESS, DAMPING)
                .with_limits(Vec2::new(-FRAC_PI_6, PI))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(LEFT_SHOULDER_Z))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -2.0*ARM_RADIUS, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, ARM_UPPER_LENGTH / 2.0, 0.0));
        let world_transform = world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        let left_shoulder_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: ARM_RADIUS, length: ARM_UPPER_LENGTH },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(left_shoulder_z)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::X)
                .with_motor_params(STIFFNESS, DAMPING)
                .with_limits(Vec2::new(-FRAC_PI_2, FRAC_PI_2))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(LEFT_SHOULDER_X))
        .insert(Mass {val: MASS})
        .id();

        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -half_upper_arm, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, half_lower_arm, 0.0));
        let world_transform = world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: ARM_RADIUS, length: ARM_LOWER_LENGTH },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(left_shoulder_x)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::X)
                .with_motor_params(STIFFNESS, DAMPING)
                .with_limits(Vec2::new(-FRAC_PI_2, 0.0))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(LEFT_ELBOW_X))
        .insert(Mass {val: MASS});
        
        // right arm
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, should_to_arm, 0.0)).with_rotation(Quat::from_rotation_z(-FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let world_transform = world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
        let right_shoulder_z = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: ARM_RADIUS, subdivisions: 7 },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(shoulder)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::Z)
                .with_motor_params(STIFFNESS, DAMPING)
                .with_limits(Vec2::new(-PI, FRAC_PI_6))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(RIGHT_SHOULDER_Z))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -2.0*ARM_RADIUS, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, ARM_UPPER_LENGTH / 2.0, 0.0));
        let world_transform = world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        let right_shoulder_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: ARM_RADIUS, length: ARM_UPPER_LENGTH },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(right_shoulder_z)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::X)
                .with_motor_params(STIFFNESS, DAMPING)
                .with_limits(Vec2::new(-FRAC_PI_2, FRAC_PI_2))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(RIGHT_SHOULDER_X))
        .insert(Mass {val: MASS})
        .id();

        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -half_upper_arm, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, half_lower_arm, 0.0));
        let world_transform = world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: ARM_RADIUS, length: ARM_LOWER_LENGTH },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(right_shoulder_x)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::X)
                .with_motor_params(STIFFNESS, DAMPING)
                .with_limits(Vec2::new(-FRAC_PI_2, 0.0))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(RIGHT_ELBOW_X))
        .insert(Mass {val: MASS});

    }

    fn build_legs(
        hip: Entity,
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        transform: Transform,
        meshes: &mut Assets<Mesh>
    ) {
        let hip_to_leg = SHOULDER_WIDTH / 4.0 + 0.02;
        let half_upper_leg = UPPER_LEG_LENGTH / 2.0 + LEG_RADIUS;
        let half_lower_leg = LOWER_LEG_LENGTH / 2.0 + LEG_RADIUS;

        // left leg
        let parent_anchor = Transform::from_translation(Vec3::new(-2.0*LEG_RADIUS, -hip_to_leg, 0.0)).with_rotation(Quat::from_rotation_z(-FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let world_transform = world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
        let left_hip_z = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: LEG_RADIUS, subdivisions: 7 },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(hip)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::Z)
                .with_motor_params(STIFFNESS, DAMPING)
                .with_limits(Vec2::new(-FRAC_PI_2, FRAC_PI_2))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(LEFT_HIP_Z))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -2.0*LEG_RADIUS, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, UPPER_LEG_LENGTH / 2.0, 0.0));
        let world_transform = world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        let left_hip_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: LEG_RADIUS, length: UPPER_LEG_LENGTH },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(left_hip_z)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::X)
                .with_motor_params(STIFFNESS, DAMPING)
                .with_limits(Vec2::new(-FRAC_PI_2, FRAC_PI_2))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(LEFT_HIP_X))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -half_upper_leg, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, half_lower_leg, 0.0));
        let world_transform = world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        let left_knee_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: LEG_RADIUS, length: LOWER_LEG_LENGTH },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(left_hip_x)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::X)
                .with_motor_params(STIFFNESS, DAMPING)
                .with_limits(Vec2::new(0.0, FRAC_PI_2))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(LEFT_KNEE_X))
        .insert(Mass {val: MASS})
        .id();

        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -half_upper_leg, 0.0)).with_rotation(Quat::from_rotation_x(-FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, FOOT_LENGTH / 2.0, 0.0));
        let world_transform = world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: LEG_RADIUS, length: FOOT_LENGTH },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(left_knee_x)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::X)
                .with_motor_params(STIFFNESS, DAMPING)
                .with_limits(Vec2::new(0.0, FRAC_PI_2))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(LEFT_FOOT_X))
        .insert(Mass {val: MASS});
        
        // right leg
        let parent_anchor = Transform::from_translation(Vec3::new(-2.0*LEG_RADIUS, hip_to_leg, 0.0)).with_rotation(Quat::from_rotation_z(-FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let world_transform = world_transform_from_joint_anchors(&transform, &parent_anchor, &child_anchor);
        let right_hip_z = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: LEG_RADIUS, subdivisions: 7 },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(hip)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::Z)
                .with_motor_params(STIFFNESS, DAMPING)
                .with_limits(Vec2::new(-FRAC_PI_2, FRAC_PI_2))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(RIGHT_HIP_Z))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -2.0*LEG_RADIUS, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, UPPER_LEG_LENGTH / 2.0, 0.0));
        let world_transform = world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        let right_hip_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: LEG_RADIUS, length: UPPER_LEG_LENGTH },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(right_hip_z)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::X)
                .with_motor_params(STIFFNESS, DAMPING)
                .with_limits(Vec2::new(-FRAC_PI_2, FRAC_PI_2))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(RIGHT_HIP_X))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -half_upper_leg, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, half_lower_leg, 0.0));
        let world_transform = world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        let right_knee_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: LEG_RADIUS, length: LOWER_LEG_LENGTH },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(right_hip_x)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::X)
                .with_motor_params(STIFFNESS, DAMPING)
                .with_limits(Vec2::new(-FRAC_PI_2, 0.0))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(RIGHT_KNEE_X))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -half_upper_leg, 0.0)).with_rotation(Quat::from_rotation_x(-FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, FOOT_LENGTH / 2.0, 0.0));
        let world_transform = world_transform_from_joint_anchors(&world_transform, &parent_anchor, &child_anchor);
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: LEG_RADIUS, length: FOOT_LENGTH },
            material.clone(),
            world_transform,
            meshes
        ))
        .insert(
            RevoluteJoint::attach_to(right_knee_x)
                .with_parent_anchor(parent_anchor)
                .with_child_anchor(child_anchor)
                .with_axis(KeskoAxis::X)
                .with_motor_params(STIFFNESS, DAMPING)
                .with_limits(Vec2::new(0.0, FRAC_PI_2))
        )
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(Name::new(RIGHT_FOOT_X))
        .insert(Mass {val: MASS});

    }
}
