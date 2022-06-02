pub mod spherical;
pub mod fixed;
pub mod revolute;
pub mod prismatic;

use bevy::prelude::*;
use rapier3d::prelude as rapier;
use rapier3d::dynamics::GenericJoint;
use crate::rigid_body::{EntityBodyHandleMap, RigidBodyHandle};


/// Component for connecting two bodies with a joint. This component should be added to the body that wants to connect
/// to another body
#[derive(Component)]
pub struct Joint {
    /// entity that the entity with the component should attach to
    parent: Entity,
    /// Rapier joint
    joint: GenericJoint
}


impl Joint {
    pub fn new(parent: Entity, joint: impl Into<GenericJoint>) -> Self {
        Self {
            parent,
            joint: joint.into()
        }
    }
}

/// Component that holds a handle to an entity's joint.
/// Used to indicate that the joint has been added both in Bevy and Rapier
#[derive(Component, Debug)]
pub(crate) struct JointHandle(pub(crate) rapier::ImpulseJointHandle);


/// System that adds joint between bodies
#[allow(clippy::type_complexity)]
pub(crate) fn add_joints_system(
    mut commands: Commands,
    mut joint_set: ResMut<rapier::ImpulseJointSet>,
    entity_body_map: Res<EntityBodyHandleMap>,
    query: Query<(Entity, &Joint), (With<RigidBodyHandle>, Without<JointHandle>)>
) {

    for (entity, joint_comp) in query.iter() {
        let joint_handle = joint_set.insert(
            *entity_body_map.get(&joint_comp.parent).unwrap(),
            *entity_body_map.get(&entity).unwrap(),
            joint_comp.joint
        );
        commands.entity(entity).insert(JointHandle(joint_handle));
    }
}

#[cfg(test)]
mod tests {
    use crate::joint::fixed::FixedJoint;
    use crate::conversions::IntoRapier;
    use super::*;

    #[test]
    fn test_add_one_joint() {

        let mut world = World::default();
        let mut test_stage = SystemStage::parallel();
        test_stage.add_system(add_joints_system);

        world.init_resource::<rapier3d::prelude::ImpulseJointSet>();

        let parent_body_handle = rapier::RigidBodyHandle::from_raw_parts(1, 0);
        let parent_entity = world
            .spawn()
            .insert(RigidBodyHandle(parent_body_handle))
            .id();

        let expected_transform = Transform::from_translation(Vec3::X);
        let joint = Joint::new(parent_entity, FixedJoint {
            parent_anchor: expected_transform,
            child_anchor: expected_transform
        });

        let child_body_handle = rapier::RigidBodyHandle::from_raw_parts(2, 0);
        let child_entity = world
            .spawn()
            .insert(joint)
            .insert(RigidBodyHandle(child_body_handle))
            .id();

        let mut entity_body_map = EntityBodyHandleMap::default();
        entity_body_map.insert(parent_entity, parent_body_handle);
        entity_body_map.insert(child_entity, child_body_handle);
        world.insert_resource(entity_body_map);

        test_stage.run(&mut world);

        let joint_set = world
            .get_resource::<rapier3d::prelude::ImpulseJointSet>()
            .expect("Could not get ImpulseJointSet").clone();

        // only on element in set
        assert_eq!(joint_set.len(), 1);

        // only one entity with joint handle, should only be the child
        let mut query = world.query::<&JointHandle>();
        assert_eq!(query.iter(&world).len(), 1);

        let joint_handle = query.get(&world, child_entity)
            .expect("Failed to get JointHandle from query");

        let joint = joint_set.get(joint_handle.0).expect("Could not get joint from joint set");

        assert_eq!(joint.body1, parent_body_handle);
        assert_eq!(joint.body2, child_body_handle);

        assert!(joint.data.as_fixed().is_some());

        assert_eq!(joint.data.local_frame1, expected_transform.into_rapier());
        assert_eq!(joint.data.local_frame2, expected_transform.into_rapier());
    }
}