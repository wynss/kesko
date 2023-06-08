use bevy::prelude::*;
use fnv::FnvHashMap;

use kesko_types::resource::KeskoRes;

use crate::rapier_extern::rapier::prelude as rapier;
use crate::{event::collision::GenerateCollisionEvents, mass::Mass, rigid_body::RigidBodyHandle};

pub type Entity2Collider = FnvHashMap<Entity, rapier::ColliderHandle>;

#[derive(Component)]
pub enum ColliderShape {
    Cuboid {
        x_half: rapier::Real,
        y_half: rapier::Real,
        z_half: rapier::Real,
    },
    Sphere {
        radius: rapier::Real,
    },
    CapsuleX {
        half_length: rapier::Real,
        radius: rapier::Real,
    },
    CapsuleY {
        half_length: rapier::Real,
        radius: rapier::Real,
    },
    CapsuleZ {
        half_length: rapier::Real,
        radius: rapier::Real,
    },
    Cylinder {
        radius: rapier::Real,
        length: rapier::Real,
    },
}

/// Component for setting the physical material properties for a collider
#[derive(Component, Debug, Clone)]
pub struct ColliderPhysicalProperties {
    /// density of the collider
    pub density: rapier::Real,
    /// friction coefficient of the collider
    pub friction: rapier::Real,
    /// restitution coefficient, controls how elastic or bouncy the collider is
    pub restitution: rapier::Real,
}

impl Default for ColliderPhysicalProperties {
    fn default() -> Self {
        Self {
            density: 1.0,
            friction: 0.9,
            restitution: 0.0,
        }
    }
}

#[derive(Component)]
pub(crate) struct ColliderHandle(rapier::ColliderHandle);

#[allow(clippy::type_complexity)]
pub(crate) fn add_colliders(
    mut commands: Commands,
    mut entity_collider_map: ResMut<KeskoRes<Entity2Collider>>,
    mut colliders: ResMut<KeskoRes<rapier::ColliderSet>>,
    mut rigid_bodies: ResMut<KeskoRes<rapier::RigidBodySet>>,
    query: Query<
        (
            Entity,
            &ColliderShape,
            &RigidBodyHandle,
            Option<&ColliderPhysicalProperties>,
            Option<&GenerateCollisionEvents>,
        ),
        Without<ColliderHandle>,
    >,
) {
    for (entity, collider_shape, rigid_body_handle, physical_props, gen_events) in query.iter() {
        let mut collider_builder = match collider_shape {
            ColliderShape::Cuboid {
                x_half,
                y_half,
                z_half,
            } => rapier::ColliderBuilder::cuboid(*x_half, *y_half, *z_half),
            ColliderShape::Sphere { radius } => rapier::ColliderBuilder::ball(*radius),
            ColliderShape::CapsuleX {
                half_length: half_height,
                radius,
            } => rapier::ColliderBuilder::capsule_x(*half_height, *radius),
            ColliderShape::CapsuleY {
                half_length: half_height,
                radius,
            } => rapier::ColliderBuilder::capsule_y(*half_height, *radius),
            ColliderShape::CapsuleZ {
                half_length: half_height,
                radius,
            } => rapier::ColliderBuilder::capsule_z(*half_height, *radius),
            ColliderShape::Cylinder { radius, length } => {
                rapier::ColliderBuilder::cylinder(length / 2.0, *radius)
            }
        };

        if let Some(physical_props) = physical_props {
            collider_builder = collider_builder
                .density(physical_props.density)
                .friction(physical_props.friction)
                .restitution(physical_props.restitution);
        }

        if gen_events.is_some() {
            collider_builder =
                collider_builder.active_events(rapier::ActiveEvents::COLLISION_EVENTS);
        }

        // store entity as user data
        collider_builder = collider_builder.user_data(entity.to_bits().into());

        let collider_handle = colliders.insert_with_parent(
            collider_builder.build(),
            rigid_body_handle.0,
            &mut rigid_bodies,
        );

        entity_collider_map.insert(entity, collider_handle);

        commands
            .entity(entity)
            .insert(ColliderHandle(collider_handle));

        let body = rigid_bodies
            .get(rigid_body_handle.0)
            .expect("Could not get rigid body");
        commands.entity(entity).insert(Mass { val: body.mass() });
    }
}

#[cfg(test)]
mod tests {

    use crate::collider::{
        add_colliders, ColliderHandle, ColliderPhysicalProperties, ColliderShape, Entity2Collider,
    };
    use crate::rapier_extern::rapier::prelude as rapier;
    use crate::rigid_body::{add_rigid_bodies, Body2Entity, Entity2Body, RigidBody};
    use bevy::prelude::*;
    use kesko_types::resource::KeskoRes;

    fn setup_app() -> App {
        let mut app = App::new();

        app.init_resource::<KeskoRes<Entity2Body>>();
        app.init_resource::<KeskoRes<Body2Entity>>();
        app.init_resource::<KeskoRes<Entity2Collider>>();

        app.init_resource::<KeskoRes<rapier::RigidBodySet>>();
        app.init_resource::<KeskoRes<rapier::ColliderSet>>();

        app.add_systems((add_rigid_bodies, add_colliders).chain());

        app
    }

    #[test]
    fn add_collider_body() {
        let mut app = setup_app();

        let entity = app
            .world
            .spawn((
                RigidBody::Fixed,
                ColliderShape::Cuboid {
                    x_half: 1.0,
                    y_half: 1.0,
                    z_half: 1.0,
                },
                Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                    .with_rotation(Quat::from_xyzw(1.0, 0.0, 0.0, 0.0)),
            ))
            .id();

        // run two times so the collider is added
        app.update();
        app.update();

        let collider_map = app
            .world
            .get_resource::<KeskoRes<Entity2Collider>>()
            .unwrap();
        let collider_handle = collider_map.get(&entity).unwrap();
        let collider_set = app
            .world
            .get_resource::<KeskoRes<rapier::ColliderSet>>()
            .unwrap();

        // only one collider body
        assert_eq!(collider_map.len(), 1);
        assert_eq!(collider_set.len(), 1);

        // check that collider has correct parent
        let body_map = app.world.get_resource::<KeskoRes<Entity2Body>>().unwrap();
        let body_handle = body_map.get(&entity).unwrap();
        let collider = collider_set.get(*collider_handle).unwrap();
        let parent_body = collider.parent().unwrap();
        assert_eq!(parent_body, *body_handle);

        // assert that we have the correct shape
        assert!(collider.shape().as_cuboid().is_some());
    }

    #[test]
    fn collider_physic_properties() {
        let mut app = setup_app();

        let physical_properties = ColliderPhysicalProperties {
            density: 2.5,
            friction: 0.65,
            restitution: 0.55,
        };

        let entity = app
            .world
            .spawn((
                RigidBody::Fixed,
                ColliderShape::Cuboid {
                    x_half: 1.0,
                    y_half: 1.0,
                    z_half: 1.0,
                },
                physical_properties.clone(),
                Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                    .with_rotation(Quat::from_xyzw(1.0, 0.0, 0.0, 0.0)),
            ))
            .id();

        // run two times so the collider is added
        app.update();
        app.update();

        assert!(app
            .world
            .query::<&ColliderPhysicalProperties>()
            .get(&app.world, entity)
            .is_ok());

        let collider_handle = app
            .world
            .query::<&ColliderHandle>()
            .get(&app.world, entity)
            .unwrap();
        let collider_set = app
            .world
            .get_resource::<KeskoRes<rapier::ColliderSet>>()
            .unwrap();
        let collider = collider_set.get(collider_handle.0).unwrap();

        assert_eq!(collider.density(), physical_properties.density);
        assert_eq!(collider.friction(), physical_properties.friction);
        assert_eq!(collider.restitution(), physical_properties.restitution);
    }
}
