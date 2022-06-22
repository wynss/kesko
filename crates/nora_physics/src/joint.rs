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
pub(crate) struct MultibodyJointHandle(pub(crate) rapier::MultibodyJointHandle);


/// System to add multibody joints between bodies
#[allow(clippy::type_complexity)]
pub(crate) fn add_multibody_joints_system(
    mut commands: Commands,
    mut joint_set: ResMut<rapier::MultibodyJointSet>,
    entity_body_map: Res<EntityBodyHandleMap>,
    query: Query<(Entity, &Joint), (With<RigidBodyHandle>, Without<MultibodyJointHandle>)>
) {

    for (entity, joint_comp) in query.iter() {
        let joint_handle = joint_set.insert(
            *entity_body_map.get(&joint_comp.parent).unwrap(),
            *entity_body_map.get(&entity).unwrap(),
            joint_comp.joint
        ).unwrap();
        commands.entity(entity).insert(MultibodyJointHandle(joint_handle));
    }
}

/// Event for communicate joint motor positions and velocities
pub enum JointEvent {
    SetPositionRevolute {
        entity: Entity,
        position: f32,
        dampning: f32,
        stiffness: f32,
    },
    SetVelocityRevolute {
        entity: Entity,
        velocity: f32,
        factor: f32
    },
    SetPositionSpherical {
        entity: Entity,
        position: f32,
        axis: rapier::JointAxis,
        dampning: f32,
        stiffness: f32,
    },
    SetVelocitySpherical {
        entity: Entity,
        velocity: f32,
        axis: rapier::JointAxis,
        factor: f32
    },
}

// pub(crate) fn update_joint_motors_system(
//     mut joint_event: EventReader<JointEvent>,
//     mut joint_set: ResMut<rapier::ImpulseJointSet>,
//     mut query: Query<&JointHandle, With<JointHandle>>
// ) {

//     for event in joint_event.iter() {

//         match event {
//             JointEvent::SetPositionRevolute { entity, position , dampning, stiffness} => {
//                 if let Ok(handle) = query.get_mut(*entity) {
//                     if let Some(joint) = joint_set.get_mut(handle.0) {
//                         if let Some(rev_joint) = joint.data.as_revolute_mut() {
//                             rev_joint.set_motor_position(*position, *stiffness, *dampning);
//                         }
//                     }
//                 }
//             },
//             JointEvent::SetVelocityRevolute{entity, velocity, factor} => {
//                 if let Ok(handle) = query.get_mut(*entity) {
//                     if let Some(joint) = joint_set.get_mut(handle.0) {
//                         if let Some(revolute_joint) = joint.data.as_revolute_mut() {
//                             revolute_joint.set_motor_velocity(*velocity, *factor);
//                         }
//                     }
//                 }
//             },
//             JointEvent::SetPositionSpherical { entity , axis, position, dampning, stiffness} => {
//                 if let Ok(handle) = query.get_mut(*entity) {
//                     if let Some(joint) = joint_set.get_mut(handle.0) {
//                         if let Some(spherical_joint) = joint.data.as_spherical_mut() {
//                             spherical_joint.set_motor_position(*axis, *position, *stiffness, *dampning);
//                         }
//                     }
//                 }
//             },
//             JointEvent::SetVelocitySpherical { entity , axis, velocity, factor } => {
//                 if let Ok(handle) = query.get_mut(*entity) {
//                     if let Some(joint) = joint_set.get_mut(handle.0) {
//                         if let Some(spherical_joint) = joint.data.as_spherical_mut() {
//                             spherical_joint.set_motor_velocity(*axis, *velocity, *factor);
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }


// #[cfg(test)]
// mod tests {
//     use bevy::ecs::event::Events;
//     use rapier3d::prelude::ImpulseJointSet;

//     use crate::joint::fixed::FixedJoint;
//     use crate::conversions::IntoRapier;
//     use super::*;

//     #[test]
//     fn test_add_one_joint() {

//         let mut world = World::default();
//         let mut test_stage = SystemStage::parallel();
//         test_stage.add_system(add_joints_system);

//         world.init_resource::<rapier3d::prelude::ImpulseJointSet>();

//         let parent_body_handle = rapier::RigidBodyHandle::from_raw_parts(1, 0);
//         let parent_entity = world
//             .spawn()
//             .insert(RigidBodyHandle(parent_body_handle))
//             .id();

//         let expected_transform = Transform::from_translation(Vec3::X);
//         let joint = Joint::new(parent_entity, FixedJoint {
//             parent_anchor: expected_transform,
//             child_anchor: expected_transform
//         });

//         let child_body_handle = rapier::RigidBodyHandle::from_raw_parts(2, 0);
//         let child_entity = world
//             .spawn()
//             .insert(joint)
//             .insert(RigidBodyHandle(child_body_handle))
//             .id();

//         let mut entity_body_map = EntityBodyHandleMap::default();
//         entity_body_map.insert(parent_entity, parent_body_handle);
//         entity_body_map.insert(child_entity, child_body_handle);
//         world.insert_resource(entity_body_map);

//         test_stage.run(&mut world);

//         let joint_set = world
//             .get_resource::<rapier3d::prelude::ImpulseJointSet>()
//             .expect("Could not get ImpulseJointSet").clone();

//         // only on element in set
//         assert_eq!(joint_set.len(), 1);

//         // only one entity with joint handle, should only be the child
//         let mut query = world.query::<&MultibodyJointHandle>();
//         assert_eq!(query.iter(&world).len(), 1);

//         let joint_handle = query.get(&world, child_entity)
//             .expect("Failed to get JointHandle from query");

//         let joint = joint_set.get(joint_handle.0).expect("Could not get joint from joint set");

//         assert_eq!(joint.body1, parent_body_handle);
//         assert_eq!(joint.body2, child_body_handle);

//         assert!(joint.data.as_fixed().is_some());

//         assert_eq!(joint.data.local_frame1, expected_transform.into_rapier());
//         assert_eq!(joint.data.local_frame2, expected_transform.into_rapier());
//     }

//     fn setup_joint_motor() -> (World, rapier::RigidBodyHandle, rapier::RigidBodyHandle, ImpulseJointSet) {

//         let world = World::new();

//         let joint_set = rapier::ImpulseJointSet::default();
//         let mut body_set = rapier::RigidBodySet::default();

//         let body1 = rapier::RigidBodyBuilder::new(rapier::RigidBodyType::Dynamic).build();
//         let body2 = rapier::RigidBodyBuilder::new(rapier::RigidBodyType::Dynamic).build();

//         let body_handle1 = body_set.insert(body1);
//         let body_handle2 = body_set.insert(body2);

//         return (world, body_handle1, body_handle2, joint_set);
//     }

//     #[test]
//     fn test_set_joint_velocity_revolute() {

//         let (mut world, body_handle1, body_handle2, mut joint_set) = setup_joint_motor();

//         // create and insert joint
//         let joint = rapier::RevoluteJointBuilder::new(rapier::Vector::x_axis()).build();
//         let joint_handle = joint_set.insert(body_handle1, body_handle2, joint);

//         world.insert_resource(joint_set);
//         let entity = world.spawn().insert(MultibodyJointHandle(joint_handle)).id();

//         // Setup and send test event for setting the velocity
//         let expected_vel = 2.3;
//         let expected_factor = 3.4;
//         let mut events = Events::<JointEvent>::default();
//         events.send(JointEvent::SetVelocityRevolute { entity: entity, velocity: expected_vel, factor: expected_factor });
//         world.insert_resource(events);

//         // Run stage
//         let test_stage = SystemStage::parallel();
//         test_stage.with_system(update_joint_motors_system).run(&mut world);
        
//         // get result
//         let res_set = world.get_resource::<rapier::ImpulseJointSet>().expect("Could not get impule joint set");
//         let res_joint = res_set.get(joint_handle).expect("Could not get joint from joint set");

//         // test correct values
//         assert_eq!(res_joint.data.as_revolute().unwrap().motor().unwrap().target_vel, expected_vel);
//         assert_eq!(res_joint.data.as_revolute().unwrap().motor().unwrap().damping, expected_factor);

//     }

//     #[test]
//     fn test_set_joint_position_revolute() {

//         let (mut world, body_handle1, body_handle2, mut joint_set) = setup_joint_motor();

//         // create and insert joint
//         let joint = rapier::RevoluteJointBuilder::new(rapier::Vector::x_axis()).build();
//         let joint_handle = joint_set.insert(body_handle1, body_handle2, joint);

//         world.insert_resource(joint_set);
//         let entity = world.spawn().insert(MultibodyJointHandle(joint_handle)).id();

//         // Setup and send test event for setting the velocity
//         let expected_pos = 2.3;
//         let expected_dampning = 3.4;
//         let expected_stiffness = 4.5;
//         let mut events = Events::<JointEvent>::default();
//         events.send(JointEvent::SetPositionRevolute { 
//             entity, 
//             position: expected_pos, 
//             dampning: expected_dampning,
//             stiffness: expected_stiffness
//         });
//         world.insert_resource(events);

//         // Run stage
//         let test_stage = SystemStage::parallel();
//         test_stage.with_system(update_joint_motors_system).run(&mut world);
        
//         // get result
//         let res_set = world.get_resource::<rapier::ImpulseJointSet>().expect("Could not get impule joint set");
//         let res_joint = res_set.get(joint_handle).expect("Could not get joint from joint set");

//         // test correct values
//         assert_eq!(res_joint.data.as_revolute().unwrap().motor().unwrap().target_pos, expected_pos);
//         assert_eq!(res_joint.data.as_revolute().unwrap().motor().unwrap().damping, expected_dampning);
//         assert_eq!(res_joint.data.as_revolute().unwrap().motor().unwrap().stiffness, expected_stiffness);

//     }

//     #[test]
//     fn test_set_joint_velocity_spherical() {

//         let (mut world, body_handle1, body_handle2, mut joint_set) = setup_joint_motor();

//         // create and insert joint
//         let joint = rapier::SphericalJointBuilder::new().build();
//         let joint_handle = joint_set.insert(body_handle1, body_handle2, joint);

//         world.insert_resource(joint_set);
//         let entity = world.spawn().insert(MultibodyJointHandle(joint_handle)).id();

//         // Setup and send test event for setting the velocity
//         let expected_vel = 2.3;
//         let expected_factor = 3.4;
//         let test_axis = rapier::JointAxis::AngX;
//         let mut events = Events::<JointEvent>::default();
//         events.send(JointEvent::SetVelocitySpherical { 
//             entity: entity, 
//             velocity: expected_vel, 
//             axis: test_axis, 
//             factor: expected_factor 
//         });
//         world.insert_resource(events);

//         // Run stage
//         let test_stage = SystemStage::parallel();
//         test_stage.with_system(update_joint_motors_system).run(&mut world);
        
//         // get result
//         let res_set = world.get_resource::<rapier::ImpulseJointSet>().expect("Could not get impule joint set");
//         let res_joint = res_set.get(joint_handle).expect("Could not get joint from joint set");

//         // test correct values
//         assert_eq!(res_joint.data.as_spherical().unwrap().motor(test_axis).unwrap().target_vel, expected_vel);
//         assert_eq!(res_joint.data.as_spherical().unwrap().motor(test_axis).unwrap().damping, expected_factor);

//     }

//     #[test]
//     fn test_set_joint_position_spherical() {

//         let (mut world, body_handle1, body_handle2, mut joint_set) = setup_joint_motor();

//         // create and insert joint
//         let joint = rapier::SphericalJointBuilder::new().build();
//         let joint_handle = joint_set.insert(body_handle1, body_handle2, joint);

//         world.insert_resource(joint_set);
//         let entity = world.spawn().insert(MultibodyJointHandle(joint_handle)).id();

//         // Setup and send test event for setting the velocity
//         let expected_pos = 2.3;
//         let expected_dampning = 3.4;
//         let expected_stiffness = 4.5;
//         let test_axis = rapier::JointAxis::AngY;
//         let mut events = Events::<JointEvent>::default();
//         events.send(JointEvent::SetPositionSpherical { 
//             entity, 
//             position: expected_pos, 
//             axis: test_axis,
//             dampning: expected_dampning,
//             stiffness: expected_stiffness
//         });
//         world.insert_resource(events);

//         // Run stage
//         let test_stage = SystemStage::parallel();
//         test_stage.with_system(update_joint_motors_system).run(&mut world);
        
//         // get result
//         let res_set = world.get_resource::<rapier::ImpulseJointSet>().expect("Could not get impule joint set");
//         let res_joint = res_set.get(joint_handle).expect("Could not get joint from joint set");

//         // test correct values
//         assert_eq!(res_joint.data.as_spherical().unwrap().motor(test_axis).unwrap().target_pos, expected_pos);
//         assert_eq!(res_joint.data.as_spherical().unwrap().motor(test_axis).unwrap().damping, expected_dampning);
//         assert_eq!(res_joint.data.as_spherical().unwrap().motor(test_axis).unwrap().stiffness, expected_stiffness);

//     }

// }