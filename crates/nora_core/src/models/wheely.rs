use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use nora_physics::{rigid_body::{RigidBody, RigidBodyName}, joint::{Joint, revolute::RevoluteJoint, prismatic::PrismaticJoint, fixed::FixedJoint}};
use nora_object_interaction::InteractiveBundle;
use crate::{
    shape::Shape,
    bundle::{MeshPhysicBodyBundle, PhysicBodyBundle},
    interaction::groups::GroupDynamic,
    transform::get_world_transform,
};

const BODY_RADIUS: f32 = 0.3;
const BODY_HEIGHT: f32 = 0.3;
const WHEEL_RADIUS: f32 = 0.20;
const WHEEL_WIDTH: f32 = 0.08;
const BACK_WHEEL_RADIUS: f32 = 0.07;
const BACK_WHEEL_WIDTH: f32 = 0.04;


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
        let wheel_offset = WHEEL_RADIUS / 3.0;

        let body = commands.spawn_bundle(MeshPhysicBodyBundle::from(RigidBody::Dynamic,
            Shape::Cylinder { radius: BODY_RADIUS, length: BODY_HEIGHT, resolution: 21 } , material.clone(), transform, meshes
        ))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("wheely".to_owned()))
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
        .insert(RigidBodyName("left_wheel".to_owned()));

        let parent_anchor = Transform::from_translation(Vec3::new(-rh, -hh + wheel_offset, 0.15))
            .with_rotation(Quat::from_rotation_z(FRAC_PI_2));
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
        .insert(RigidBodyName("right_wheel".to_owned()));
        
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
            limits: Some(Vec2::new(-FRAC_PI_2, FRAC_PI_2)),
            ..default()
        }))
        .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
        .insert(RigidBodyName("back_wheel_turn".to_owned()))
        .id();
        
        let parent_anchor = Transform::from_translation(Vec3::new(0.0, -BACK_WHEEL_RADIUS, -BACK_WHEEL_RADIUS))
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
        .insert(RigidBodyName("arm_link_1".to_owned()))
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
        .insert(RigidBodyName("arm_link_2".to_owned()));

    }
}
