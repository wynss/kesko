use bevy::prelude::*;

use kesko_physics::{rigid_body::{RigidBody, RigidBodyName}, mass::Mass};
use kesko_core::{
    bundle::MeshPhysicBodyBundle,
    shape::Shape,
    interaction::groups::GroupStatic
};
use kesko_raycast::RayVisible;


const NAME: &str = "plane";

pub fn spawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    // Spawn ground
    commands.spawn_bundle(MeshPhysicBodyBundle::from(
        RigidBody::Fixed,
        Shape::Box {x_length: 2000.0, y_length: 2.0, z_length: 2000.0},
        material.clone(),
        Transform::from_xyz(0.0, -1.0, 0.0),
        meshes
    ))
    .insert(RayVisible::<GroupStatic>::default())
    .insert(Mass {val: 1000.0})
    .insert(RigidBodyName(NAME.to_owned()));
}
