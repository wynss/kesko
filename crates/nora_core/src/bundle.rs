use bevy::prelude::*;
use nora_physics::collider::{ColliderPhysicalProperties, ColliderShape};
use nora_physics::gravity::GravityScale;
use nora_physics::impulse::Impulse;
use nora_physics::rigid_body::RigidBody;

#[derive(Debug)]
pub enum Shape {
    Sphere {
        radius: f32,
        subdivisions: usize
    },
    Cube {
        size: f32
    },
    Plane {
        size: f32
    }
}

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
            }
            _ => panic!("Shape {:?} not implemented for BodyBundle", shape)
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