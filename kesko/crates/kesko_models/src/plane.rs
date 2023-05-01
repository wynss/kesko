use bevy::prelude::*;

use kesko_core::{bundle::MeshPhysicBodyBundle, interaction::groups::GroupStatic, shape::Shape};
use kesko_physics::{mass::Mass, rigid_body::RigidBody};
use kesko_raycast::RayVisible;

const NAME: &str = "plane";

pub fn spawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    // Spawn ground
    commands.spawn((
        MeshPhysicBodyBundle::from(
            RigidBody::Fixed,
            Shape::Box {
                x_length: 2000.0,
                y_length: 2.0,
                z_length: 2000.0,
            },
            material,
            Transform::from_xyz(0.0, -1.0, 0.0),
            meshes,
        ),
        RayVisible::<GroupStatic>::default(),
        Mass { val: 1000.0 },
        Name::new(NAME),
    ));
}
