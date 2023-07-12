use bevy::prelude::*;
use fnv::FnvHashMap;

use kesko_types::resource::KeskoRes;

use crate::{conversions::IntoRapier, gravity::GravityScale};
use crate::{mass::Mass, rapier_extern::rapier::prelude as rapier};

pub type Entity2Body = FnvHashMap<Entity, rapier::RigidBodyHandle>;
pub type Body2Entity = FnvHashMap<rapier::RigidBodyHandle, Entity>;

#[derive(Component)]
pub enum RigidBody {
    Fixed,
    Dynamic,
}

/// If a rigid body can sleep or not
#[derive(Component)]
pub struct CanSleep(pub bool);

#[derive(Component)]
pub struct RigidBodyHandle(pub rapier::RigidBodyHandle);

#[allow(clippy::type_complexity)]
pub(crate) fn add_rigid_bodies(
    mut rigid_bodies: ResMut<KeskoRes<rapier::RigidBodySet>>,
    mut entity2body: ResMut<KeskoRes<Entity2Body>>,
    mut body2entity: ResMut<KeskoRes<Body2Entity>>,
    mut commands: Commands,
    query: Query<
        (
            Entity,
            &RigidBody,
            &Transform,
            Option<&Mass>,
            Option<&GravityScale>,
            Option<&CanSleep>,
        ),
        Without<RigidBodyHandle>,
    >,
) {
    for (entity, rigid_body_comp, transform, mass, gravity_scale, can_sleep) in query.iter() {
        let mut rigid_body_builder = match rigid_body_comp {
            RigidBody::Fixed => rapier::RigidBodyBuilder::fixed(),
            RigidBody::Dynamic => rapier::RigidBodyBuilder::dynamic(),
        };

        if let Some(gravity_scale) = gravity_scale {
            rigid_body_builder = rigid_body_builder.gravity_scale(gravity_scale.val);
        }

        if let Some(can_sleep) = can_sleep {
            rigid_body_builder = rigid_body_builder.can_sleep(can_sleep.0);
        }
        if let Some(mass) = mass {
            rigid_body_builder = rigid_body_builder.additional_mass(mass.val);
        }

        let rigid_body = rigid_body_builder
            .position((transform.translation, transform.rotation).into_rapier())
            .build();

        // insert and add to map
        let rigid_body_handle = rigid_bodies.insert(rigid_body);
        entity2body.insert(entity, rigid_body_handle);
        body2entity.insert(rigid_body_handle, entity);

        // Add the rigid body component to the entity
        commands
            .entity(entity)
            .insert(RigidBodyHandle(rigid_body_handle));
    }
}

#[cfg(test)]
mod tests {

    use crate::conversions::IntoBevy;
    use crate::rapier_extern::rapier::prelude as rapier;
    use crate::rigid_body::{
        add_rigid_bodies, Body2Entity, Entity2Body, RigidBody, RigidBodyHandle,
    };
    use bevy::prelude::*;
    use kesko_types::resource::KeskoRes;

    #[test]
    fn add_rigid_body() {
        let mut app = App::new();

        app.add_systems(Update, add_rigid_bodies);

        app.init_resource::<KeskoRes<Entity2Body>>();
        app.init_resource::<KeskoRes<Body2Entity>>();
        app.init_resource::<KeskoRes<rapier::RigidBodySet>>();

        let entity = app
            .world
            .spawn((
                RigidBody::Fixed,
                Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                    .with_rotation(Quat::from_xyzw(1.0, 0.0, 0.0, 0.0)),
            ))
            .id();
        app.update();

        let body_map = app.world.get_resource::<KeskoRes<Entity2Body>>().unwrap();
        let body_handle = body_map.get(&entity).unwrap();
        let body_set = app
            .world
            .get_resource::<KeskoRes<rapier::RigidBodySet>>()
            .unwrap();

        // only one rigid body
        assert_eq!(body_map.len(), 1);
        assert_eq!(body_set.len(), 1);

        // Check rapier transform equal to bevy transform
        let rigid_body = body_set.get(*body_handle).unwrap();
        let (translation, rotation) = rigid_body.position().into_bevy();

        let (expected_transform, _) = app
            .world
            .query::<(&Transform, &RigidBodyHandle)>()
            .get(&app.world, entity)
            .unwrap();

        assert_eq!(translation, expected_transform.translation);
        assert_eq!(rotation, expected_transform.rotation);
    }
}
