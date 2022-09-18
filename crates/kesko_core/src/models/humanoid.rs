use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use kesko_physics::{rigid_body::{
    RigidBody,
    RigidBodyName
}, joint::{Joint, revolute::RevoluteJoint, fixed::FixedJoint}};
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

const STIFFNESS: f32 = 0.5;


pub struct Humanoid;
impl Humanoid {

    pub fn spawn(
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        origin: Transform,
        meshes: &mut Assets<Mesh>
    ) {

        let should_dist = 2.6 * SHOULDER_RADIUS;

        let head = commands.spawn_bundle(MeshPhysicBodyBundle::from(RigidBody::Dynamic,
            Shape::Sphere { radius: HEAD_RADIUS, subdivisions: 7 }, 
            material.clone(), 
            origin, 
            meshes
        ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName(NAME.to_owned()))
        .id();

        let shoulder = Self::build_neck(head, commands, material.clone(), origin, meshes);

        

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

        // // neck y
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
        
        // // neck z
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
        .id();

        shoulder

    }

    fn build_upper_body(
        shoulder: Entity,
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        origin: Transform,
        meshes: &mut Assets<Mesh>
    ) -> (Entity, Entity) {

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
        .id();

        (shoulder, hip)

    }

    fn build_left_arm() {

    }

    fn build_right_arm() {

    }

    fn build_left_leg() {

    }

    fn build_right_leg() {

    }
}