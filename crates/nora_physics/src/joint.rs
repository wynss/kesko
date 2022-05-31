use bevy::prelude::*;
use rapier3d::prelude as rapier;
use rapier3d::dynamics::GenericJoint;
use crate::rigid_body::{EntityBodyHandleMap, RigidBodyHandle};
use crate::conversions::IntoRapier;


/// Component that holds a handle to an entity's joint.
/// Used to indicate that the joint has been added both in Bevy and Rapier
#[derive(Component, Debug)]
pub(crate) struct JointHandle(pub(crate) rapier::ImpulseJointHandle);

#[derive(Component)]
pub struct Joint {
    pub joint_type: JointType,
    pub parent: Entity,
    pub parent_anchor: (Vec3, Quat),
    pub child_anchor: (Vec3, Quat)
}

/// Represents a specific joint type and its parameters
#[derive(Debug)]
pub enum JointType {
    /// A fixed joint that prevents two bodies to move relative to each other
    Fixed,
    /// Joint that allows rotations for all axes but prevents any translational motion
    Spherical,
    /// Allows rotation around the specified 'axis' and prevents any translational motion
    Revolute {
        /// Axis of rotation
        axis: Vec3
    },
    /// Allows translational motion along the specified 'axis' and prevents any rotational motion
    Prismatic {
        /// Axis of rotation
        axis: Vec3,
        /// Limits for the motion relative to the joints origin
        limits: Option<Vec2>
    }
}

/// System that adds joint between bodies
#[allow(clippy::type_complexity)]
pub(crate) fn add_joints_system(
    mut commands: Commands,
    mut joint_set: ResMut<rapier::ImpulseJointSet>,
    entity_body_map: Res<EntityBodyHandleMap>,
    query: Query<(Entity, &Joint), (With<RigidBodyHandle>, Without<JointHandle>)>
) {

    for (entity, joint_comp) in query.iter() {
        let mut joint: GenericJoint  = match joint_comp.joint_type {
            JointType::Fixed => {
                rapier::FixedJointBuilder::new().into()
            },
            JointType::Revolute {axis} => {
                rapier::RevoluteJointBuilder::new(axis.into_rapier()).into()
            },
            JointType::Spherical => {
                rapier::SphericalJointBuilder::new().into()
            },
            JointType::Prismatic {axis, limits} => {
                let mut builder = rapier::PrismaticJointBuilder::new(axis.into_rapier());
                if let Some(limits) = limits {
                    builder = builder.limits(limits.into());
                }
                builder.into()
            }
        };

        joint.set_local_frame1(joint_comp.parent_anchor.into_rapier());
        joint.set_local_frame2(joint_comp.child_anchor.into_rapier());

        let joint_handle = joint_set.insert(
            *entity_body_map.get(&joint_comp.parent).unwrap(),
            *entity_body_map.get(&entity).unwrap(),
            joint
        );

        commands.entity(entity).insert(JointHandle(joint_handle));

    }
}

#[cfg(test)]
mod tests {

    use rapier3d::prelude as rapier;
    use bevy::prelude::*;
    use rapier3d::dynamics::JointAxis;
    use crate::IntoRapier;
    use crate::joint::{Joint, JointHandle, JointType};
    use crate::rigid_body::{EntityBodyHandleMap, RigidBodyHandle};
    use super::add_joints_system;

    const ANCHOR_PARENT: (Vec3, Quat) = (Vec3::X, Quat::IDENTITY);
    const ANCHOR_CHILD: (Vec3, Quat) = (Vec3::X, Quat::IDENTITY);

    struct TestCtx {
        child_entity: Entity,
        parent_body_handle: rapier::RigidBodyHandle,
        child_body_handle: rapier::RigidBodyHandle
    }

    fn setup(world: &mut World, joint_type: JointType) -> TestCtx {

        world.init_resource::<rapier3d::prelude::ImpulseJointSet>();

        let parent_body_handle = rapier::RigidBodyHandle::from_raw_parts(1, 0);
        let parent_entity = world
            .spawn()
            .insert(RigidBodyHandle(parent_body_handle))
            .id();

        let joint = Joint {
            joint_type,
            parent: parent_entity,
            parent_anchor: ANCHOR_PARENT,
            child_anchor: ANCHOR_CHILD
        };

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

        TestCtx {
            child_entity,
            parent_body_handle,
            child_body_handle
        }
    }

    #[test]
    fn test_add_one_joint() {

        let mut world = World::default();
        let mut test_stage = SystemStage::parallel();
        test_stage.add_system(add_joints_system);

        let ctx = setup(&mut world, JointType::Fixed);

        test_stage.run(&mut world);

        let joint_set = world
            .get_resource::<rapier3d::prelude::ImpulseJointSet>()
            .expect("Could not get ImpulseJointSet").clone();

        // only on element in set
        assert_eq!(joint_set.len(), 1);

        // only one entity with joint handle, should only be the child
        let mut query = world.query::<&JointHandle>();
        assert_eq!(query.iter(&world).len(), 1);

        let joint_handle = query.get(&world, ctx.child_entity)
            .expect("Failed to get JointHandle from query");

        let joint = joint_set.get(joint_handle.0).expect("Could not get joint from joint set");

        assert_eq!(joint.body1, ctx.parent_body_handle);
        assert_eq!(joint.body2, ctx.child_body_handle);

        assert!(joint.data.as_fixed().is_some());
        assert_eq!(joint.data.local_frame1, ANCHOR_PARENT.into_rapier());
        assert_eq!(joint.data.local_frame2, ANCHOR_CHILD.into_rapier());
    }

    #[test]
    fn test_spherical() {
        let mut world = World::default();
        let mut test_stage = SystemStage::parallel();
        test_stage.add_system(add_joints_system);

        let ctx = setup(&mut world, JointType::Spherical);

        test_stage.run(&mut world);

        let joint_set = world
            .get_resource::<rapier3d::prelude::ImpulseJointSet>()
            .expect("Could not get ImpulseJointSet").clone();
        let joint_handle = world.query::<&JointHandle>().get(&world, ctx.child_entity)
            .expect("Failed to get JointHandle from query");

        let joint = joint_set.get(joint_handle.0).expect("Could not get joint from joint set");

        assert_eq!(joint.body1, ctx.parent_body_handle);
        assert_eq!(joint.body2, ctx.child_body_handle);

        assert!(joint.data.as_spherical().is_some());
        assert_eq!(joint.data.local_frame1, ANCHOR_PARENT.into_rapier());
        assert_eq!(joint.data.local_frame2, ANCHOR_CHILD.into_rapier());
    }

    #[test]
    fn test_revolute() {
        let mut world = World::default();
        let mut test_stage = SystemStage::parallel();
        test_stage.add_system(add_joints_system);

        let ctx = setup(&mut world, JointType::Revolute {
            axis: Vec3::Z
        });

        test_stage.run(&mut world);

        let joint_set = world
            .get_resource::<rapier3d::prelude::ImpulseJointSet>()
            .expect("Could not get ImpulseJointSet").clone();
        let joint_handle = world.query::<&JointHandle>().get(&world, ctx.child_entity)
            .expect("Failed to get JointHandle from query");

        let joint = joint_set.get(joint_handle.0).expect("Could not get joint from joint set");

        // correct body mapping
        assert_eq!(joint.body1, ctx.parent_body_handle);
        assert_eq!(joint.body2, ctx.child_body_handle);

        // correct type
        assert!(joint.data.as_revolute().is_some());

        // axis of rotation
        assert_eq!(joint.data.local_axis1(), Vec3::X.into_rapier());
        assert_eq!(joint.data.local_axis2(), Vec3::X.into_rapier());

        // anchor points
        assert_eq!(joint.data.local_frame1, ANCHOR_PARENT.into_rapier());
        assert_eq!(joint.data.local_frame2, ANCHOR_CHILD.into_rapier());
    }

    #[test]
    fn test_prismatic() {
        let mut world = World::default();
        let mut test_stage = SystemStage::parallel();
        test_stage.add_system(add_joints_system);

        let axis = Vec3::X;
        let ctx = setup(&mut world, JointType::Prismatic {
            axis,
            limits: Some(Vec2::new(-2.0, 2.0))
        });

        test_stage.run(&mut world);

        let joint_set = world
            .get_resource::<rapier3d::prelude::ImpulseJointSet>()
            .expect("Could not get ImpulseJointSet").clone();
        let joint_handle = world.query::<&JointHandle>().get(&world, ctx.child_entity)
            .expect("Failed to get JointHandle from query");

        let joint = joint_set.get(joint_handle.0).expect("Could not get joint from joint set");

        // correct body mapping
        assert_eq!(joint.body1, ctx.parent_body_handle);
        assert_eq!(joint.body2, ctx.child_body_handle);

        // correct type
        assert!(joint.data.as_prismatic().is_some());

        // axis of rotation
        assert_eq!(joint.data.local_axis1(), axis.into_rapier());
        assert_eq!(joint.data.local_axis2(), axis.into_rapier());

        // limits
        assert_eq!(joint.data.limits(JointAxis::X).unwrap().max, 2.0);
        assert_eq!(joint.data.limits(JointAxis::X).unwrap().min, -2.0);

        // anchor points
        assert_eq!(joint.data.local_frame1, ANCHOR_PARENT.into_rapier());
        assert_eq!(joint.data.local_frame2, ANCHOR_CHILD.into_rapier());
    }
}