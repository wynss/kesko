use std::f32::consts::{FRAC_PI_2, FRAC_PI_6, FRAC_PI_4, PI};

use bevy::prelude::*;

use kesko_physics::{
    rigid_body::{
        RigidBody,
        RigidBodyName
    }, 
    joint::{
        Axis,
        Joint, 
        revolute::RevoluteJoint, 
        fixed::FixedJoint
    }, 
    mass::Mass
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
const MASS: f32 = 0.1;

const STIFFNESS: f32 = 1.0;

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


pub struct Humanoid;
impl Humanoid {

    pub fn spawn(
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        origin: Transform,
        meshes: &mut Assets<Mesh>
    ) {

        let head = commands.spawn_bundle(MeshPhysicBodyBundle::from(RigidBody::Dynamic,
            Shape::Sphere { radius: HEAD_RADIUS, subdivisions: 7 }, 
            material.clone(), 
            origin, 
            meshes
        ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName(Model::Humanoid.name().to_owned()))
        .insert(Mass {val: MASS})
        .id();

        let shoulder = Self::build_neck(head, commands, material.clone(), origin, meshes);
        let hip = Self::build_upper_body(shoulder, commands, material.clone(), origin, meshes);

        Self::build_arms(shoulder, commands, material.clone(), origin, meshes);
        Self::build_legs(hip, commands, material.clone(), origin, meshes);
    }

    fn build_neck(
        head: Entity,
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        origin: Transform,
        meshes: &mut Assets<Mesh>
    ) -> Entity {
        
        // neck x
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -(HEAD_RADIUS + NECK_LENGTH / 4.0), 0.0));
        let child_anchor = Transform::default();
        let neck_x = commands.spawn_bundle(PhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: 0.01, subdivisions: 5 },
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
        ))
        .insert(Joint::new(head, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Axis::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_4, FRAC_PI_4)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("neck_x".to_owned()))
        .id();

        // neck y
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -0.02, 0.0));
        let child_anchor = Transform::default();
        let neck_y = commands.spawn_bundle(PhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: 0.01, subdivisions: 5 },
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
        ))
        .insert(Joint::new(neck_x, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Axis::Y,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("neck_y".to_owned()))
        .id();
        
        // neck z
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -NECK_LENGTH / 2.0, 0.0)).with_rotation(Quat::from_rotation_z(FRAC_PI_2));
        let child_anchor = Transform::default();
        let shoulder = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: SHOULDER_RADIUS, length: SHOULDER_WIDTH },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(neck_y, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Axis::Z,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_6, FRAC_PI_6)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("neck_z".to_owned()))
        .insert(Mass {val: MASS})
        .id();

        shoulder

    }

    fn build_upper_body(
        shoulder: Entity,
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        origin: Transform,
        meshes: &mut Assets<Mesh>
    ) -> Entity {

        let should_dist = 3.0 * SHOULDER_RADIUS;

        let parent_anchor = Transform::from_translation(Vec3::new(-should_dist, 0.0, 0.0));
        let child_anchor = Transform::default();
        let body_1 = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: SHOULDER_RADIUS, length: 0.8*SHOULDER_WIDTH },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(shoulder, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("body_1".to_owned()))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(-should_dist, 0.0, 0.0));
        let child_anchor = Transform::default();
        let body_2 = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: SHOULDER_RADIUS, length: 0.6*SHOULDER_WIDTH },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(body_1, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("body_2".to_owned()))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(-should_dist, 0.0, 0.0));
        let child_anchor = Transform::default();
        let hip = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: SHOULDER_RADIUS, length: 0.5*SHOULDER_WIDTH },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(body_2, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("body_3".to_owned()))
        .insert(Mass {val: MASS})
        .id();

        hip

    }

    fn build_arms(
        shoulder: Entity,
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        origin: Transform,
        meshes: &mut Assets<Mesh>
    ) {
        
        let should_to_arm = SHOULDER_WIDTH / 2.0 + ARM_RADIUS + SHOULDER_RADIUS;
        let half_upper_arm = ARM_UPPER_LENGTH / 2.0 + ARM_RADIUS;
        let half_lower_arm = ARM_LOWER_LENGTH / 2.0 + ARM_RADIUS;

        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -should_to_arm, 0.0)).with_rotation(Quat::from_rotation_z(-FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let left_upper_arm_z = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: ARM_RADIUS, subdivisions: 7 },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(shoulder, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Axis::Z,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_6, PI)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("left_upper_arm_z".to_owned()))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -2.0*ARM_RADIUS, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, ARM_UPPER_LENGTH / 2.0, 0.0));
        let left_upper_arm_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: ARM_RADIUS, length: ARM_UPPER_LENGTH },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(left_upper_arm_z, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Axis::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("left_upper_arm_x".to_owned()))
        .insert(Mass {val: MASS})
        .id();

        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -half_upper_arm, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, half_lower_arm, 0.0));
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: ARM_RADIUS, length: ARM_LOWER_LENGTH },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(left_upper_arm_x, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Axis::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, 0.0)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("left_lower_arm_x".to_owned()))
        .insert(Mass {val: MASS});
        
        // right arm
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, should_to_arm, 0.0)).with_rotation(Quat::from_rotation_z(FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let right_upper_arm_z = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: ARM_RADIUS, subdivisions: 7 },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(shoulder, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Axis::Z,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-PI, FRAC_PI_6)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("right_upper_arm_z".to_owned()))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, 2.0*ARM_RADIUS, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, -ARM_UPPER_LENGTH / 2.0, 0.0));
        let right_upper_arm_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: ARM_RADIUS, length: ARM_UPPER_LENGTH },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(right_upper_arm_z, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Axis::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("right_upper_arm_x".to_owned()))
        .insert(Mass {val: MASS})
        .id();

        let parent_anchor = Transform::from_translation(Vec3::new(0.0, half_upper_arm, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_lower_arm, 0.0));
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: ARM_RADIUS, length: ARM_LOWER_LENGTH },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(right_upper_arm_x, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Axis::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(0.0, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("right_lower_arm_x".to_owned()))
        .insert(Mass {val: MASS});

    }

    fn build_legs(
        hip: Entity,
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        origin: Transform,
        meshes: &mut Assets<Mesh>
    ) {
        let should_to_arm = SHOULDER_WIDTH / 4.0 + 0.02;
        let half_upper_leg = UPPER_LEG_LENGTH / 2.0 + LEG_RADIUS;
        let half_lower_leg = LOWER_LEG_LENGTH / 2.0 + LEG_RADIUS;

        let parent_anchor = Transform::from_translation(Vec3::new(-2.0*LEG_RADIUS, -should_to_arm, 0.0)).with_rotation(Quat::from_rotation_z(-FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let left_upper_leg_z = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: LEG_RADIUS, subdivisions: 7 },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(hip, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Axis::Z,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("left_upper_leg_z".to_owned()))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -2.0*LEG_RADIUS, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, UPPER_LEG_LENGTH / 2.0, 0.0));
        let left_upper_leg_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: LEG_RADIUS, length: UPPER_LEG_LENGTH },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(left_upper_leg_z, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Axis::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("left_upper_leg_x".to_owned()))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -half_upper_leg, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, half_lower_leg, 0.0));
        let left_lower_leg_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: LEG_RADIUS, length: LOWER_LEG_LENGTH },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(left_upper_leg_x, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Axis::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(0.0, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("left_lower_leg_x".to_owned()))
        .insert(Mass {val: MASS})
        .id();

        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -half_upper_leg, 0.0)).with_rotation(Quat::from_rotation_x(-FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, FOOT_LENGTH / 2.0, 0.0));
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: LEG_RADIUS, length: FOOT_LENGTH },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(left_lower_leg_x, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Axis::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(0.0, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("left_foot_x".to_owned()))
        .insert(Mass {val: MASS});
        
        // right leg
        let parent_anchor = Transform::from_translation(Vec3::new(-2.0*LEG_RADIUS, should_to_arm, 0.0)).with_rotation(Quat::from_rotation_z(FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let right_upper_arm_z = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: LEG_RADIUS, subdivisions: 7 },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(hip, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Axis::Z,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("right_upper_leg_z".to_owned()))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, 2.0*LEG_RADIUS, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, -UPPER_LEG_LENGTH / 2.0, 0.0));
        let right_upper_arm_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: LEG_RADIUS, length: UPPER_LEG_LENGTH },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(right_upper_arm_z, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Axis::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("right_upper_leg_x".to_owned()))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, half_upper_leg, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, -half_lower_leg, 0.0));
        let right_lower_leg_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: LEG_RADIUS, length: LOWER_LEG_LENGTH },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(right_upper_arm_x, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Axis::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, 0.0)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("right_lower_leg_x".to_owned()))
        .insert(Mass {val: MASS})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, half_upper_leg, 0.0)).with_rotation(Quat::from_rotation_x(-FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, FOOT_LENGTH / 2.0, 0.0));
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: LEG_RADIUS, length: FOOT_LENGTH },
            material.clone(),
            world_transform_from_joint_anchors(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(right_lower_leg_x, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Axis::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, 0.0)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("right_foot_x".to_owned()))
        .insert(Mass {val: MASS});

    }
}
