use bevy::prelude::*;
use rapier3d::prelude as rapier;
use fnv::FnvHashMap;
use rapier3d::prelude::ColliderBuilder;

use super::rigid_body::RigidBodyHandle;
use crate::mass::Mass;


pub type EntityColliderMap = FnvHashMap<Entity, rapier::ColliderHandle>;

#[derive(Component)]
pub enum ColliderShape {
    Cuboid {
        x_half: f32,
        y_half: f32,
        z_half: f32
    },
    Sphere {
        radius: f32
    },
    CapsuleX {
        half_height: f32,
        radius: f32
    },
    CapsuleY {
        half_height: f32,
        radius: f32
    },
    CapsuleZ {
        half_height: f32,
        radius: f32
    },
    Cylinder {
        radius: f32,
        length: f32
    }
}

/// Component for setting the physical material properties for a collider
#[derive(Component, Debug, Clone)]
pub struct ColliderPhysicalProperties {
    /// density of the collider
    pub density: f32,
    /// friction coefficient of the collider
    pub friction: f32,
    /// restitution coefficient, controls how elastic or bouncy the collider is
    pub restitution: f32
}

impl Default for ColliderPhysicalProperties {
    fn default() -> Self {
        Self {
            density: 1.0,
            friction: 0.7,
            restitution: 0.3
        }
    }
}

#[derive(Component)]
pub(crate) struct ColliderHandle(rapier::ColliderHandle);


pub(crate) fn add_collider_to_bodies_system(
    mut commands: Commands,
    mut entity_collider_map: ResMut<EntityColliderMap>,
    mut collider_set: ResMut<rapier::ColliderSet>,
    mut rigid_body_set: ResMut<rapier::RigidBodySet>,
    query: Query<(Entity, &ColliderShape, &RigidBodyHandle, Option<&ColliderPhysicalProperties>), Without<ColliderHandle>>
) {
    for (entity, collider_shape, rigid_body_handle, material) in query.iter() {

        let collider_builder = match collider_shape {
            ColliderShape::Cuboid {x_half, y_half, z_half} => {
                rapier::ColliderBuilder::cuboid(*x_half, *y_half, *z_half)
            },
            ColliderShape::Sphere {radius} => {
                rapier::ColliderBuilder::ball(*radius)
            },
            ColliderShape::CapsuleX {half_height, radius} => {
                ColliderBuilder::capsule_x(*half_height, *radius)
            },
            ColliderShape::CapsuleY {half_height, radius} => {
                ColliderBuilder::capsule_y(*half_height, *radius)
            },
            ColliderShape::CapsuleZ {half_height, radius} => {
                ColliderBuilder::capsule_z(*half_height, *radius)
            },
            ColliderShape::Cylinder {radius, length} => {
                ColliderBuilder::cylinder(length / 2.0, *radius)
            }
        };

        let collider = if let Some(material) = material {
            collider_builder
                .density(material.density)
                .friction(material.friction)
                .restitution(material.restitution)
                .build()
        } else {
            collider_builder.build()
        };

        let collider_handle = collider_set.insert_with_parent(
            collider,
            rigid_body_handle.0,
            &mut rigid_body_set
        );

        entity_collider_map.insert(entity, collider_handle);

        commands.entity(entity).insert(ColliderHandle(collider_handle));

        let body = rigid_body_set.get(rigid_body_handle.0).expect("Could not get rigid body");
        commands.entity(entity).insert(Mass {val: body.mass()});

    }
}


#[cfg(test)]
mod tests {

    use bevy::prelude::*;
    use rapier3d::prelude as rapier;
    use crate::collider::{EntityColliderMap, ColliderShape, ColliderPhysicalProperties, add_collider_to_bodies_system, ColliderHandle};
    use crate::rigid_body::{RigidBody, EntityBodyHandleMap, add_rigid_bodies_system};

    fn setup_world() -> (World, SystemStage) {

        let mut world = World::default();
        world.init_resource::<EntityBodyHandleMap>();
        world.init_resource::<EntityColliderMap>();

        world.init_resource::<rapier::RigidBodySet>();
        world.init_resource::<rapier::ColliderSet>();

        let test_stage = SystemStage::parallel()
            .with_system(add_rigid_bodies_system)
            .with_system(add_collider_to_bodies_system.after(add_rigid_bodies_system));

        (world, test_stage)
    }


    #[test]
    fn add_collider_body() {

        let (mut world, mut stage) = setup_world();

        let entity = world
            .spawn()
            .insert(RigidBody::Fixed)
            .insert(ColliderShape::Cuboid {x_half: 1.0, y_half: 1.0, z_half: 1.0})
            .insert(Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                .with_rotation(Quat::from_xyzw(1.0, 0.0, 0.0, 0.0)))
            .id();

        // run two times so the collider is added
        stage.run(&mut world);
        stage.run(&mut world);

        let collider_map = world.get_resource::<EntityColliderMap>().unwrap();
        let collider_handle = collider_map.get(&entity).unwrap();
        let collider_set = world.get_resource::<rapier::ColliderSet>().unwrap();

        // only one collider body
        assert_eq!(collider_map.len(), 1);
        assert_eq!(collider_set.len(), 1);

        // check that collider has correct parent
        let body_map = world.get_resource::<EntityBodyHandleMap>().unwrap();
        let body_handle = body_map.get(&entity).unwrap();
        let collider = collider_set.get(*collider_handle).unwrap();
        let parent_body = collider.parent().unwrap();
        assert_eq!(parent_body, *body_handle);

        // assert that we have the correct shape
        assert!(collider.shape().as_cuboid().is_some());
    }

    #[test]
    fn collider_physic_properties() {

        let (mut world, mut stage) = setup_world();

        let physical_properties = ColliderPhysicalProperties {
            density: 2.5,
            friction: 0.65,
            restitution: 0.55
        };

        let entity = world
            .spawn()
            .insert(RigidBody::Fixed)
            .insert(ColliderShape::Cuboid {x_half: 1.0, y_half: 1.0, z_half: 1.0})
            .insert(physical_properties.clone())
            .insert(Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                .with_rotation(Quat::from_xyzw(1.0, 0.0, 0.0, 0.0)))
            .id();

        // run two times so the collider is added
        stage.run(&mut world);
        stage.run(&mut world);

        assert!(world.query::<&ColliderPhysicalProperties>().get(&world, entity).is_ok());

        let collider_handle = world.query::<&ColliderHandle>().get(&world, entity).unwrap();
        let collider_set = world.get_resource::<rapier::ColliderSet>().unwrap();
        let collider = collider_set.get(collider_handle.0).unwrap();

        assert_eq!(collider.density().unwrap(), physical_properties.density);
        assert_eq!(collider.friction(), physical_properties.friction);
        assert_eq!(collider.restitution(), physical_properties.restitution);
    }
}
