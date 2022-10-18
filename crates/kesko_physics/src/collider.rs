use bevy::prelude::*;
use rapier3d::prelude as rapier;
use fnv::FnvHashMap;

use super::rigid_body::RigidBodyHandle;
use crate::{
    mass::Mass, 
    event::collision::GenerateCollisionEvents
};


pub type Entity2ColliderHandle = FnvHashMap<Entity, rapier::ColliderHandle>;

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
        half_length: f32,
        radius: f32
    },
    CapsuleY {
        half_length: f32,
        radius: f32
    },
    CapsuleZ {
        half_length: f32,
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
            friction: 0.9,
            restitution: 0.0
        }
    }
}

#[derive(Component)]
pub(crate) struct ColliderHandle(rapier::ColliderHandle);


#[allow(clippy::type_complexity)]
pub(crate) fn add_colliders(
    mut commands: Commands,
    mut entity_collider_map: ResMut<Entity2ColliderHandle>,
    mut collider_set: ResMut<rapier::ColliderSet>,
    mut rigid_body_set: ResMut<rapier::RigidBodySet>,
    query: Query<(
        Entity, 
        &ColliderShape, 
        &RigidBodyHandle, 
        Option<&Mass>,
        Option<&ColliderPhysicalProperties>, 
        Option<&GenerateCollisionEvents>), 
        Without<ColliderHandle>>
) {
    for (entity, collider_shape, rigid_body_handle, mass, physical_props, gen_events) in query.iter() {

        let mut collider_builder = match collider_shape {
            ColliderShape::Cuboid {x_half, y_half, z_half} => {
                rapier::ColliderBuilder::cuboid(*x_half, *y_half, *z_half)
            },
            ColliderShape::Sphere {radius} => {
                rapier::ColliderBuilder::ball(*radius)
            },
            ColliderShape::CapsuleX { half_length: half_height, radius} => {
                rapier::ColliderBuilder::capsule_x(*half_height, *radius)
            },
            ColliderShape::CapsuleY { half_length: half_height, radius} => {
                rapier::ColliderBuilder::capsule_y(*half_height, *radius)
            },
            ColliderShape::CapsuleZ { half_length: half_height, radius} => {
                rapier::ColliderBuilder::capsule_z(*half_height, *radius)
            },
            ColliderShape::Cylinder {radius, length} => {
                rapier::ColliderBuilder::cylinder(length / 2.0, *radius)
            }
        };

        if let Some(mass) = mass {
            collider_builder = collider_builder.mass(mass.val);
        }
 
        if let Some(physical_props) = physical_props {
            collider_builder = collider_builder
            .density(physical_props.density)
            .friction(physical_props.friction)
            .restitution(physical_props.restitution);
        }

        if gen_events.is_some() {
            collider_builder = collider_builder.active_events(rapier::ActiveEvents::COLLISION_EVENTS);
        }

        // store entity as user data
        collider_builder = collider_builder.user_data(entity.to_bits().into());

        let collider_handle = collider_set.insert_with_parent(
            collider_builder.build(),
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
    use crate::collider::{Entity2ColliderHandle, ColliderShape, ColliderPhysicalProperties, add_colliders, ColliderHandle};
    use crate::rigid_body::{RigidBody, Entity2BodyHandle, add_rigid_bodies, BodyHandle2Entity};

    fn setup_world() -> (World, SystemStage) {

        let mut world = World::default();
        world.init_resource::<Entity2BodyHandle>();
        world.init_resource::<BodyHandle2Entity>();
        world.init_resource::<Entity2ColliderHandle>();

        world.init_resource::<rapier::RigidBodySet>();
        world.init_resource::<rapier::ColliderSet>();

        let test_stage = SystemStage::single_threaded()
            .with_system(add_rigid_bodies)
            .with_system(add_colliders.after(add_rigid_bodies));

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

        let collider_map = world.get_resource::<Entity2ColliderHandle>().unwrap();
        let collider_handle = collider_map.get(&entity).unwrap();
        let collider_set = world.get_resource::<rapier::ColliderSet>().unwrap();

        // only one collider body
        assert_eq!(collider_map.len(), 1);
        assert_eq!(collider_set.len(), 1);

        // check that collider has correct parent
        let body_map = world.get_resource::<Entity2BodyHandle>().unwrap();
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

        assert_eq!(collider.density(), physical_properties.density);
        assert_eq!(collider.friction(), physical_properties.friction);
        assert_eq!(collider.restitution(), physical_properties.restitution);
    }
}
