use bevy::prelude::*;

use nora_physics::collider::{ColliderPhysicalProperties, ColliderShape};
use nora_physics::gravity::GravityScale;
use nora_physics::impulse::Impulse;
use nora_physics::rigid_body::RigidBody;
use crate::shape::{Shape, Cylinder};


/// Component bundle for physic enabled Pbr entities
#[derive(Bundle)]
pub struct PhysicBodyBundle {
    rigid_body: RigidBody,
    collider_shape: ColliderShape,
    collider_physical_properties: ColliderPhysicalProperties,
    impulse: Impulse,
    /// How mush the entity will be affected by gravity
    gravity_scale: GravityScale,

    #[bundle]
    pbr_bundle: PbrBundle,
}

impl PhysicBodyBundle {
    pub fn from(
        body_type: RigidBody,
        shape: Shape,
        material: Handle<StandardMaterial>,
        transform: Transform,
        meshes: &mut Assets<Mesh>,
    ) -> Self {

        let (mesh, collider_shape) = match shape {
            Shape::Sphere{radius, subdivisions} => {
                (Mesh::from(shape::Icosphere {radius, subdivisions}), ColliderShape::Sphere {radius})
            },
            Shape::Plane {size} => {
                (
                    Mesh::from(shape::Plane {size}),
                    ColliderShape::Cuboid { x_half: size / 2.0, y_half: 0.0, z_half: size / 2.0}
                )
            },
            Shape::Cube {size} => {
                (
                    Mesh::from(shape::Cube {size}),
                    ColliderShape::Cuboid { x_half: size / 2.0, y_half: size / 2.0, z_half: size / 2.0}
                )
            },
            Shape::Box { x_length, y_length, z_length } => {
                (
                    Mesh::from(shape::Box::new(x_length, y_length, z_length)),
                    ColliderShape::Cuboid {x_half: x_length / 2.0, y_half: y_length / 2.0, z_half: z_length / 2.0}
                )
            },
            Shape::Cylinder {radius, length} => {
                (
                    Mesh::from(Cylinder {radius, height: length, ..default()}),
                    ColliderShape::Cylinder {radius, length}
                )
            }
        };

        Self {
            rigid_body: body_type,
            collider_shape,
            collider_physical_properties: ColliderPhysicalProperties::default(),
            impulse: Impulse::default(),
            gravity_scale: GravityScale::default(),

            pbr_bundle: PbrBundle {
                mesh: meshes.add(mesh),
                material,
                transform,
                ..Default::default()
            },
        }
    }
}