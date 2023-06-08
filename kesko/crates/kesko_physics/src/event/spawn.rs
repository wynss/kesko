use std::collections::BTreeMap;

use bevy::prelude::*;

use crate::{
    event::PhysicResponseEvent,
    joint::{prismatic::PrismaticJoint, revolute::RevoluteJoint, JointInfo},
    multibody::MultibodyRoot,
    rigid_body::RigidBody,
};

pub(crate) fn send_spawned_events(
    mut event_writer: EventWriter<PhysicResponseEvent>,
    bodies: Query<(Entity, Option<&Name>, Option<&MultibodyRoot>), Added<RigidBody>>,
    revolute_joints: Query<&RevoluteJoint>,
    prismatic_joints: Query<&PrismaticJoint>,
) {
    for (entity, name, root) in bodies.iter() {
        if let Some(root) = root {
            // create a map with joint info
            let joint_info_map = root
                .child_map
                .iter()
                .map(|(name, e)| {
                    let joint_info = if let Ok(joint) = revolute_joints.get(*e) {
                        Some(JointInfo::Revolute {
                            name: name.clone(),
                            axis: joint.axis,
                            limits: joint.limits,
                            damping: joint.damping,
                            stiffness: joint.stiffness,
                            max_motor_force: joint.max_motor_force,
                        })
                    } else if let Ok(joint) = prismatic_joints.get(*e) {
                        Some(JointInfo::Prismatic {
                            name: name.clone(),
                            axis: joint.axis,
                            limits: joint.limits,
                            damping: joint.damping,
                            stiffness: joint.stiffness,
                            max_motor_force: joint.max_motor_force,
                        })
                    } else {
                        None
                    };

                    (e.to_bits(), joint_info)
                })
                .filter_map(|(e, joint_info)| match joint_info {
                    Some(joint_info) => Some((e, joint_info)),
                    None => None,
                })
                .collect::<BTreeMap<u64, JointInfo>>();

            info!("Multibody spawned");
            event_writer.send(PhysicResponseEvent::MultibodySpawned {
                id: entity.to_bits(),
                entity,
                name: root.name.clone(),
                joints: joint_info_map,
            });
        } else {
            info!("Rigid body spawned");
            let name = match name {
                Some(name) => name.to_string(),
                None => entity.index().to_string(),
            };
            event_writer.send(PhysicResponseEvent::RigidBodySpawned {
                id: entity.to_bits(),
                name,
            })
        }
    }
}
