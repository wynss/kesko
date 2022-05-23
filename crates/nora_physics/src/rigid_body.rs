use bevy::prelude::*;
use rapier3d::prelude as rapier;
use fnv::FnvHashMap;

use crate::conversions::IntoRapier;


pub type EntityBodyHandleMap = FnvHashMap<Entity, rapier::RigidBodyHandle>;

#[derive(Component)]
pub enum RigidBody {
    Fixed,
    Dynamic
}

#[derive(Component)]
pub struct RigidBodyHandle(pub rapier::RigidBodyHandle);


pub(crate) fn add_rigid_bodies_system(
    mut rigid_body_set: ResMut<rapier::RigidBodySet>,
    mut entity_body_map: ResMut<EntityBodyHandleMap>,
    mut commands: Commands,
    query: Query<(Entity, &RigidBody, &Transform), Without<RigidBodyHandle>>
) {

    for (entity, rigid_body_comp, transform) in query.iter() {

        let rigid_body_builder = match rigid_body_comp {
            RigidBody::Fixed => rapier::RigidBodyBuilder::fixed(),
            RigidBody::Dynamic => rapier::RigidBodyBuilder::dynamic()
        };

        let rigid_body = rigid_body_builder
            .position((transform.translation, transform.rotation).into_rapier())
            .build();

        // insert and add to map
        let rigid_body_handle = rigid_body_set.insert(rigid_body);
        entity_body_map.insert(entity, rigid_body_handle);

        // Add the rigid body component to the entity
        commands.entity(entity).insert(RigidBodyHandle(rigid_body_handle));
    }
}

#[cfg(test)]
mod tests {

    use bevy::prelude::*;
    use rapier3d::prelude as rapier;
    use crate::rigid_body::{add_rigid_bodies_system, EntityBodyHandleMap, RigidBody, RigidBodyHandle};
    use crate::conversions::IntoBevy;

    #[test]
    fn add_rigid_body() {

        let mut world = World::default();

        let mut test_stage = SystemStage::parallel();
        test_stage.add_system(add_rigid_bodies_system);

        world.init_resource::<EntityBodyHandleMap>();
        world.init_resource::<rapier3d::prelude::RigidBodySet>();


        let entity = world
            .spawn()
            .insert(RigidBody::Fixed)
            .insert(Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                .with_rotation(Quat::from_xyzw(1.0, 0.0, 0.0, 0.0)))
            .id();

        test_stage.run(&mut world);

        let body_map = world.get_resource::<EntityBodyHandleMap>().unwrap();
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
