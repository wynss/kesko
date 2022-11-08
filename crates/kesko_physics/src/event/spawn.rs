use std::collections::BTreeMap;

use bevy::prelude::*;

use crate::{
    event::PhysicResponseEvent,
    multibody::MultibodyRoot, 
    rigid_body::RigidBody,
    joint::{
        revolute::RevoluteJoint,
        prismatic::PrismaticJoint, JointInfo
    }
};


pub(crate) fn send_spawned_events(
    mut event_writer: EventWriter<PhysicResponseEvent>,
    bodies: Query<(Entity, &Name, Option<&MultibodyRoot>), Added<RigidBody>>,
    revolute_joints: Query<&RevoluteJoint>,
    prismatic_joints: Query<&PrismaticJoint>,
) {

    for (entity, name, root) in bodies.iter() {
        if let Some(root) = root {

            // create a map with joint info
            let joint_info_map = root.child_map.iter().map(|(name, e)| {

                let joint_info = if let Ok(joint) = revolute_joints.get(*e) {
                    Some(JointInfo::Revolute { 
                        name: name.clone(), 
                        axis: joint.axis, 
                        limits: joint.limits, 
                        damping: joint.damping, 
                        stiffness: joint.stiffness, 
                        max_motor_force: joint.max_motor_force
                    })
                } else if let Ok(joint) = prismatic_joints.get(*e) {
                    Some(JointInfo::Prismatic { 
                        name: name.clone(), 
                        axis: joint.axis, 
                        limits: joint.limits, 
                        damping: joint.damping, 
                        stiffness: joint.stiffness, 
                        max_motor_force: joint.max_motor_force
                    })
                } else {
                    None
                };

                (e.to_bits(), joint_info)
            })
            .filter(|(_, ji)| {ji.is_some()})
            .map(|(id, ji)| {(id, ji.unwrap())})
            .collect::<BTreeMap<u64, JointInfo>>();

            event_writer.send(PhysicResponseEvent::MultibodySpawned{ 
                id: entity.to_bits(),
                entity,
                name: root.name.clone(),
                joints: joint_info_map
            });
        } else {
            event_writer.send(PhysicResponseEvent::RigidBodySpawned{ 
                id: entity.to_bits(),
                name: name.to_string()
            })
        }
    }
}
