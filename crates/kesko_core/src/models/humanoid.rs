use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use kesko_physics::{rigid_body::{
    RigidBody,
    RigidBodyName
}, joint::{Joint, revolute::RevoluteJoint, fixed::FixedJoint}, mass::Mass};
use kesko_object_interaction::InteractiveBundle;

use crate::{
    shape::Shape,
    bundle::{
        MeshPhysicBodyBundle,
        PhysicBodyBundle
    },
    interaction::groups::GroupDynamic,
    transform::get_world_transform
};


const NAME: &str = "humanoid";
const HEAD_RADIUS: f32 = 0.1;
const NECK_LENGTH: f32 = 0.1;
const SHOULDER_WIDTH: f32 = 0.3;
const SHOULDER_RADIUS: f32 = 0.03;

const NECK_X: &str = "neck_x";
const NECK_Y: &str = "neck_y";
const NECK_Z: &str = "neck_Z";

const STIFFNESS: f32 = 1.0;


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
        .insert(RigidBodyName(NAME.to_owned()))
        .insert(Mass {val: 1.0})
        .id();

        let shoulder = Self::build_neck(head, commands, material.clone(), origin, meshes);
        Self::build_arms(shoulder, commands, material.clone(), origin, meshes);
        let hip = Self::build_upper_body(shoulder, commands, material.clone(), origin, meshes);
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
        let neck_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: 0.01, subdivisions: 5 },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(head, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::X,
            stiffness: STIFFNESS,
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName(NECK_X.to_owned()))
        .id();

        // neck y
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -0.02, 0.0));
        let child_anchor = Transform::default();
        let neck_y = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: 0.01, subdivisions: 5 },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(neck_x, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Y,
            stiffness: STIFFNESS,
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName(NECK_Y.to_owned()))
        .id();
        
        // neck z
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -NECK_LENGTH / 2.0, 0.0)).with_rotation(Quat::from_rotation_z(FRAC_PI_2));
        let child_anchor = Transform::default();
        let shoulder = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: SHOULDER_RADIUS, length: SHOULDER_WIDTH },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(neck_y, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Z,
            stiffness: STIFFNESS,
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName(NECK_Z.to_owned()))
        .insert(Mass {val: 1.0})
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
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(shoulder, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("body_1".to_owned()))
        .insert(Mass {val: 1.0})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(-should_dist, 0.0, 0.0));
        let child_anchor = Transform::default();
        let body_2 = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: SHOULDER_RADIUS, length: 0.6*SHOULDER_WIDTH },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(body_1, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("body_2".to_owned()))
        .insert(Mass {val: 1.0})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(-should_dist, 0.0, 0.0));
        let child_anchor = Transform::default();
        let hip = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: SHOULDER_RADIUS, length: 0.5*SHOULDER_WIDTH },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(body_2, FixedJoint {
            parent_anchor,
            child_anchor
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("body_3".to_owned()))
        .insert(Mass {val: 1.0})
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
        let arm_radius = 0.025;
        let upper_arm_length = 0.20;
        let lower_arm_length = 0.15;
        let should_to_arm = SHOULDER_WIDTH / 2.0 + arm_radius + 0.02;

        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -should_to_arm, 0.0)).with_rotation(Quat::from_rotation_z(-FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let left_upper_arm_z = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: arm_radius, subdivisions: 7 },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(shoulder, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Z,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("left_upper_arm_z".to_owned()))
        .insert(Mass {val: 1.0})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -2.0*arm_radius, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, upper_arm_length / 2.0, 0.0));
        let left_upper_arm_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: arm_radius, length: upper_arm_length },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(left_upper_arm_z, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("left_upper_arm_x".to_owned()))
        .insert(Mass {val: 1.0})
        .id();
        
        let dist_upper = upper_arm_length / 2.0 + arm_radius;
        let dist_lower = lower_arm_length / 2.0 + arm_radius;

        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -dist_upper, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, dist_lower, 0.0));
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: arm_radius, length: lower_arm_length },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(left_upper_arm_x, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, 0.0)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("left_lower_arm_x".to_owned()))
        .insert(Mass {val: 1.0});
        
        // right arm
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, should_to_arm, 0.0)).with_rotation(Quat::from_rotation_z(FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let right_upper_arm_z = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: arm_radius, subdivisions: 7 },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(shoulder, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Z,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("right_upper_arm_z".to_owned()))
        .insert(Mass {val: 1.0})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, 2.0*arm_radius, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, -upper_arm_length / 2.0, 0.0));
        let right_upper_arm_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: arm_radius, length: upper_arm_length },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(right_upper_arm_z, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("left_upper_arm_x".to_owned()))
        .insert(Mass {val: 1.0})
        .id();
        
        let dist_upper = upper_arm_length / 2.0 + arm_radius;
        let dist_lower = lower_arm_length / 2.0 + arm_radius;

        let parent_anchor = Transform::from_translation(Vec3::new(0.0, dist_upper, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, -dist_lower, 0.0));
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: arm_radius, length: lower_arm_length },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(right_upper_arm_x, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, 0.0)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("right_lower_arm_x".to_owned()))
        .insert(Mass {val: 1.0});

    }

    fn build_legs(
        hip: Entity,
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        origin: Transform,
        meshes: &mut Assets<Mesh>
    ) {
        let leg_radius = 0.03;
        let upper_leg_length = 0.3;
        let lower_leg_length = 0.25;
        let foot_length = 0.1;
        let should_to_arm = SHOULDER_WIDTH / 4.0 + 0.02;
        let half_upper_leg = upper_leg_length / 2.0 + leg_radius;
        let half_lower_leg = lower_leg_length / 2.0 + leg_radius;

        let parent_anchor = Transform::from_translation(Vec3::new(-2.0*leg_radius, -should_to_arm, 0.0)).with_rotation(Quat::from_rotation_z(-FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let left_upper_leg_z = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: leg_radius, subdivisions: 7 },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(hip, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Z,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("left_upper_leg_z".to_owned()))
        .insert(Mass {val: 1.0})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -2.0*leg_radius, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, upper_leg_length / 2.0, 0.0));
        let left_upper_leg_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: leg_radius, length: upper_leg_length },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(left_upper_leg_z, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("left_upper_leg_x".to_owned()))
        .insert(Mass {val: 1.0})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -half_upper_leg, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, half_lower_leg, 0.0));
        let left_lower_leg_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: leg_radius, length: lower_leg_length },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(left_upper_leg_x, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, 0.0)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("left_lower_leg_x".to_owned()))
        .insert(Mass {val: 1.0})
        .id();

        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -half_upper_leg, 0.0)).with_rotation(Quat::from_rotation_x(-FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, foot_length / 2.0, 0.0));
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: leg_radius, length: foot_length },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(left_lower_leg_x, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, 0.0)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("left_foot_x".to_owned()))
        .insert(Mass {val: 1.0});
        
        // right leg
        let parent_anchor = Transform::from_translation(Vec3::new(-2.0*leg_radius, should_to_arm, 0.0)).with_rotation(Quat::from_rotation_z(FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let right_upper_arm_z = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Sphere { radius: leg_radius, subdivisions: 7 },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(hip, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::Z,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("right_upper_arm_z".to_owned()))
        .insert(Mass {val: 1.0})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, 2.0*leg_radius, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, -upper_leg_length / 2.0, 0.0));
        let right_upper_arm_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: leg_radius, length: upper_leg_length },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(right_upper_arm_z, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("left_upper_arm_x".to_owned()))
        .insert(Mass {val: 1.0})
        .id();
        
        let dist_upper = upper_leg_length / 2.0 + leg_radius;
        let dist_lower = lower_leg_length / 2.0 + leg_radius;

        let parent_anchor = Transform::from_translation(Vec3::new(0.0, dist_upper, 0.0));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, -dist_lower, 0.0));
        let right_lower_leg_x = commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: leg_radius, length: lower_leg_length },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(right_upper_arm_x, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, 0.0)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("right_lower_leg_x".to_owned()))
        .insert(Mass {val: 1.0})
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, half_upper_leg, 0.0)).with_rotation(Quat::from_rotation_x(-FRAC_PI_2));
        let child_anchor = Transform::from_translation(Vec3::new(0.0, foot_length / 2.0, 0.0));
        commands.spawn_bundle(MeshPhysicBodyBundle::from(
            RigidBody::Dynamic,
            Shape::Capsule { radius: leg_radius, length: foot_length },
            material.clone(),
            get_world_transform(&origin, &parent_anchor, &child_anchor),
            meshes
        ))
        .insert(Joint::new(right_lower_leg_x, RevoluteJoint {
            parent_anchor,
            child_anchor,
            axis: Vec3::X,
            stiffness: STIFFNESS,
            limits: Some(Vec2::new(-FRAC_PI_2, 0.0)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("right_foot_x".to_owned()))
        .insert(Mass {val: 1.0});

    }
}