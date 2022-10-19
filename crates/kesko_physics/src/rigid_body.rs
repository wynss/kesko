use bevy::prelude::*;
use crate::{
    rapier_extern::rapier::prelude as rapier, 
    mass::Mass
};
use fnv::FnvHashMap;

use crate::{
    conversions::IntoRapier, 
    gravity::GravityScale
};


pub type Entity2BodyHandle = FnvHashMap<Entity, rapier::RigidBodyHandle>;
pub type BodyHandle2Entity = FnvHashMap<rapier::RigidBodyHandle, Entity>;

#[derive(Component)]
pub enum RigidBody {
    Fixed,
    Dynamic
}

/// If a rigid body can sleep or not
#[derive(Component)]
pub struct CanSleep(pub bool);

// Name of a rigid body
#[derive(Component, Debug)]
pub struct RigidBodyName(pub String);

#[derive(Component)]
pub struct RigidBodyHandle(pub rapier::RigidBodyHandle);


#[allow(clippy::type_complexity)]
pub(crate) fn add_rigid_bodies(
    mut rigid_body_set: ResMut<rapier::RigidBodySet>,
    mut entity_2_body_handle: ResMut<Entity2BodyHandle>,
    mut body_handle_2_entity: ResMut<BodyHandle2Entity>,
    mut commands: Commands,
    query: Query<(
        Entity, 
        &RigidBody, 
        &Transform, 
        Option<&Mass>,
        Option<&GravityScale>, 
        Option<&CanSleep>), 
        Without<RigidBodyHandle>>
) {
    for (entity, rigid_body_comp, transform, mass, gravity_scale, can_sleep) in query.iter() {

        let mut rigid_body_builder = match rigid_body_comp {
            RigidBody::Fixed => rapier::RigidBodyBuilder::fixed(),
            RigidBody::Dynamic => rapier::RigidBodyBuilder::dynamic()
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
        let rigid_body_handle = rigid_body_set.insert(rigid_body);
        entity_2_body_handle.insert(entity, rigid_body_handle);
        body_handle_2_entity.insert(rigid_body_handle, entity);

        // Add the rigid body component to the entity
        commands.entity(entity).insert(RigidBodyHandle(rigid_body_handle));
    }
}


#[cfg(test)]
mod tests {

    use bevy::prelude::*;
    use crate::rapier_extern::rapier::prelude as rapier;
    use crate::rigid_body::{add_rigid_bodies, Entity2BodyHandle, RigidBody, RigidBodyHandle, BodyHandle2Entity};
    use crate::conversions::IntoBevy;

    #[test]
    fn add_rigid_body() {

        let mut world = World::default();

        let mut test_stage = SystemStage::single_threaded();
        test_stage.add_system(add_rigid_bodies);

        world.init_resource::<Entity2BodyHandle>();
        world.init_resource::<BodyHandle2Entity>();
        world.init_resource::<rapier::RigidBodySet>();


        let entity = world
            .spawn()
            .insert(RigidBody::Fixed)
            .insert(Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                .with_rotation(Quat::from_xyzw(1.0, 0.0, 0.0, 0.0)))
            .id();

        test_stage.run(&mut world);

        let body_map = world.get_resource::<Entity2BodyHandle>().unwrap();
        let body_handle = body_map.get(&entity).unwrap();
        let body_set = world.get_resource::<rapier::RigidBodySet>().unwrap();

        // only one rigid body
        assert_eq!(body_map.len(), 1);
        assert_eq!(body_set.len(), 1);

        // Check rapier transform equal to bevy transform
        let rigid_body = body_set.get(*body_handle).unwrap();
        let (translation, rotation) = rigid_body.position().into_bevy();

        let (expected_transform, _) = world.query::<(&Transform, &RigidBodyHandle)>().get(&world, entity).unwrap();

        assert_eq!(translation, expected_transform.translation);
        assert_eq!(rotation, expected_transform.rotation);
    }
}
