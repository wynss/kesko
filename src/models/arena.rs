use bevy::prelude::*;

use nora_physics::rigid_body::RigidBody;
use nora_core::{
    bundle::MeshPhysicBodyBundle,
    shape::Shape,
    interaction::groups::GroupStatic
};
use nora_raycast::RayVisible;


pub fn spawn_arena(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>,
    width: f32,
    length: f32,
    wall_height: f32
) {

    let wall_width = 0.2;
    let half_wall_width = wall_width / 2.0;

    // Spawn ground
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Fixed,
        Shape::Box {x_length: width, y_length: 0.5, z_length: length},
        material.clone(),
        Transform::from_xyz(0.0, -0.25, 0.0),
        meshes
    )).insert(RayVisible::<GroupStatic>::default());

    // right wall
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Fixed,
        Shape::Box {x_length: wall_width, y_length: wall_height, z_length: length},
        material.clone(),
        Transform::from_xyz(half_wall_width - (width / 2.0), wall_height / 2.0, 0.0),
        meshes
    )).insert(RayVisible::<GroupStatic>::default());

    // left wall
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Fixed,
        Shape::Box {x_length: wall_width, y_length: wall_height, z_length: length},
        material.clone(),
        Transform::from_xyz(width / 2.0 - half_wall_width, wall_height / 2.0, 0.0),
        meshes
    )).insert(RayVisible::<GroupStatic>::default());

    // back wall
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Fixed,
        Shape::Box {x_length: length, y_length: wall_height, z_length: wall_width},
        material.clone(),
        Transform::from_xyz(0.0, wall_height / 2.0, half_wall_width - (width / 2.0)),
        meshes
    )).insert(RayVisible::<GroupStatic>::default());

    // front wall
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Fixed,
        Shape::Box {x_length: length, y_length: wall_height, z_length: wall_width},
        material.clone(),
        Transform::from_xyz(0.0, wall_height / 2.0, width / 2.0 - half_wall_width),
        meshes
    )).insert(RayVisible::<GroupStatic>::default());
}