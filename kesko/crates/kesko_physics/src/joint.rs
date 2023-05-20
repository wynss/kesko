pub mod fixed;
pub mod prismatic;
pub mod revolute;
pub mod spherical;

use bevy::prelude::*;
use fnv::FnvHashMap;
use serde::{Deserialize, Serialize};

use kesko_types::resource::KeskoRes;

use crate::conversions::IntoBevy;
use crate::rapier_extern::rapier::prelude as rapier;
use crate::rigid_body::{Entity2Body, RigidBodyHandle};

use self::prismatic::PrismaticJoint;
use self::revolute::RevoluteJoint;

pub type Entity2JointHandle = FnvHashMap<Entity, rapier::MultibodyJointHandle>;

/// trait for converting an axis into a unit vector
pub(crate) trait AxisIntoVec {
    fn into_unitvec(self) -> rapier::UnitVector<rapier::Real>;
}
impl AxisIntoVec for KeskoAxis {
    fn into_unitvec(self) -> rapier::UnitVector<rapier::Real> {
        match self {
            KeskoAxis::X | KeskoAxis::AngX => rapier::Vector::x_axis(),
            KeskoAxis::Y | KeskoAxis::AngY => rapier::Vector::y_axis(),
            KeskoAxis::Z | KeskoAxis::AngZ => rapier::Vector::z_axis(),
            KeskoAxis::NegX => -rapier::Vector::x_axis(),
            KeskoAxis::NegY => -rapier::Vector::y_axis(),
            KeskoAxis::NegZ => -rapier::Vector::z_axis(),
        }
    }
}

impl From<KeskoAxis> for rapier::JointAxis {
    fn from(value: KeskoAxis) -> Self {
        match value {
            KeskoAxis::X | KeskoAxis::NegX => rapier::JointAxis::X,
            KeskoAxis::Y | KeskoAxis::NegY => rapier::JointAxis::Y,
            KeskoAxis::Z | KeskoAxis::NegZ => rapier::JointAxis::Z,
            KeskoAxis::AngX => rapier::JointAxis::AngX,
            KeskoAxis::AngY => rapier::JointAxis::AngY,
            KeskoAxis::AngZ => rapier::JointAxis::AngZ,
        }
    }
}

/// Override Rapiers JointAxis since it did not include negative axis
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum KeskoAxis {
    X,
    Y,
    Z,
    NegX,
    NegY,
    NegZ,
    AngX,
    AngY,
    AngZ,
}

/// used to send joint info outside Kesko
/// TODO: This is a bit redundant when spherical joints is not available
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum JointInfo {
    Revolute {
        name: String,
        axis: KeskoAxis,
        limits: Option<Vec2>,
        damping: rapier::Real,
        stiffness: rapier::Real,
        max_motor_force: rapier::Real,
    },
    Prismatic {
        name: String,
        axis: KeskoAxis,
        limits: Option<Vec2>,
        damping: rapier::Real,
        stiffness: rapier::Real,
        max_motor_force: rapier::Real,
    },
}

/// used to send joint state outside Kesko
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum JointState {
    Revolute {
        axis: KeskoAxis,
        angle: rapier::Real,
        angular_velocity: rapier::Real,
    },
    Prismatic {
        axis: KeskoAxis,
        position: rapier::Real,
        velocity: rapier::Real,
    },
}

/// Enum to indicate joint type
#[derive(Debug)]
pub enum JointType {
    Fixed,
    Revolute,
    Prismatic,
    Spherical,
}

/// Component that holds a handle to an entity's joint.
/// Used to indicate that the joint has been added both in Bevy and Rapier
#[derive(Component, Debug)]
pub struct MultibodyJointHandle(pub(crate) rapier::MultibodyJointHandle);

/// System to add multibody joints between bodies
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub(crate) fn add_multibody_joints(
    mut commands: Commands,
    mut entity_2_joint_handle: ResMut<KeskoRes<Entity2JointHandle>>,
    mut multibody_joint_set: ResMut<KeskoRes<rapier::MultibodyJointSet>>,
    entity_body_map: Res<KeskoRes<Entity2Body>>,
    revolute_joints: Query<
        (Entity, &revolute::RevoluteJoint),
        (With<RigidBodyHandle>, Without<MultibodyJointHandle>),
    >,
    prismatic_joints: Query<
        (Entity, &prismatic::PrismaticJoint),
        (With<RigidBodyHandle>, Without<MultibodyJointHandle>),
    >,
    spherical_joints: Query<
        (Entity, &spherical::SphericalJoint),
        (With<RigidBodyHandle>, Without<MultibodyJointHandle>),
    >,
    fixed_joints: Query<
        (Entity, &fixed::FixedJoint),
        (With<RigidBodyHandle>, Without<MultibodyJointHandle>),
    >,
) {
    for (entity, joint) in revolute_joints.iter() {
        let joint_handle = multibody_joint_set
            .insert(
                *entity_body_map.get(&joint.parent).unwrap(),
                *entity_body_map.get(&entity).unwrap(),
                *joint,
                true,
            )
            .unwrap();

        entity_2_joint_handle.insert(entity, joint_handle);
        commands
            .entity(entity)
            .insert(MultibodyJointHandle(joint_handle));
    }
    for (entity, joint) in prismatic_joints.iter() {
        let joint_handle = multibody_joint_set
            .insert(
                *entity_body_map.get(&joint.parent).unwrap(),
                *entity_body_map.get(&entity).unwrap(),
                *joint,
                true,
            )
            .unwrap();

        entity_2_joint_handle.insert(entity, joint_handle);
        commands
            .entity(entity)
            .insert(MultibodyJointHandle(joint_handle));
    }
    for (entity, joint) in spherical_joints.iter() {
        let joint_handle = multibody_joint_set
            .insert(
                *entity_body_map.get(&joint.parent).unwrap(),
                *entity_body_map.get(&entity).unwrap(),
                *joint,
                true,
            )
            .unwrap();

        entity_2_joint_handle.insert(entity, joint_handle);
        commands
            .entity(entity)
            .insert(MultibodyJointHandle(joint_handle));
    }
    for (entity, joint) in fixed_joints.iter() {
        let joint_handle = multibody_joint_set
            .insert(
                *entity_body_map.get(&joint.parent).unwrap(),
                *entity_body_map.get(&entity).unwrap(),
                *joint,
                true,
            )
            .unwrap();

        entity_2_joint_handle.insert(entity, joint_handle);
        commands
            .entity(entity)
            .insert(MultibodyJointHandle(joint_handle));
    }
}

/// System that updates the joints positions and velocities
#[allow(clippy::type_complexity)]
pub(crate) fn update_joint_pos_system(
    physics_params: Res<KeskoRes<rapier::IntegrationParameters>>,
    multibody_joint_set: Res<KeskoRes<rapier::MultibodyJointSet>>,
    mut revolute_joints: Query<
        (&mut RevoluteJoint, &MultibodyJointHandle),
        (With<MultibodyJointHandle>, With<RevoluteJoint>),
    >,
    mut prismatic_joints: Query<
        (&mut PrismaticJoint, &MultibodyJointHandle),
        (With<MultibodyJointHandle>, With<PrismaticJoint>),
    >,
) {
    for (mut joint, handle) in revolute_joints.iter_mut() {
        if let Some((mb, link_index)) = multibody_joint_set.get(handle.0) {
            if let Some(link) = mb.link(link_index) {
                // we have the joint transformation in the parent frame
                let (_, rot) = link.joint().body_to_parent().into_bevy();
                joint.update_rotation_angvel(rot, physics_params.dt);
            }
        }
    }
    for (mut joint, handle) in prismatic_joints.iter_mut() {
        if let Some((mb, link_index)) = multibody_joint_set.get(handle.0) {
            if let Some(link) = mb.link(link_index) {
                // we have the joint transformation in the parent frame
                let (translation, _) = link.joint().body_to_parent().into_bevy();
                joint.update_position_vel(translation, physics_params.dt);
            }
        }
    }
}

/// Event for communicate joint motor positions and velocities
#[derive(Debug)]
pub struct JointMotorEvent {
    pub entity: Entity,
    pub command: MotorCommand,
}

#[derive(Debug)]
pub enum MotorCommand {
    PositionRevolute {
        position: rapier::Real,
        stiffness: Option<rapier::Real>,
        damping: Option<rapier::Real>,
    },
    VelocityRevolute {
        velocity: rapier::Real,
        damping: Option<rapier::Real>,
    },
    PositionSpherical {
        position: rapier::Real,
        axis: KeskoAxis,
    },
    VelocitySpherical {
        velocity: rapier::Real,
        axis: KeskoAxis,
    },
    PositionPrismatic {
        position: rapier::Real,
        stiffness: Option<rapier::Real>,
        damping: Option<rapier::Real>,
    },
    VelocityPrismatic {
        velocity: rapier::Real,
        damping: Option<rapier::Real>,
    },
    SetStiffness {
        val: rapier::Real,
    },
    SetDamping {
        val: rapier::Real,
    },
    HoldPosition {
        stiffness: Option<rapier::Real>,
    },
}

#[allow(clippy::type_complexity)]
pub(crate) fn update_joint_motors_system(
    mut joint_event: EventReader<JointMotorEvent>,
    mut joint_set: ResMut<KeskoRes<rapier::MultibodyJointSet>>,
    mut query: Query<
        (
            Option<&RevoluteJoint>,
            Option<&PrismaticJoint>,
            &MultibodyJointHandle,
        ),
        With<MultibodyJointHandle>,
    >,
) {
    for event in joint_event.iter() {
        match query.get_mut(event.entity) {
            Err(e) => error!("{:?}", e),
            Ok((revolute_joint, prismatic_joint, joint_handle)) => match joint_set
                .get_mut(joint_handle.0)
            {
                None => {
                    error!("Could not get joint from joint set")
                }
                Some((mb, id)) => match mb.link_mut(id) {
                    None => {
                        error!("Could not get multi joint link from multibody");
                    }
                    Some(joint_link) => match event.command {
                        MotorCommand::PositionRevolute {
                            position,
                            stiffness,
                            damping,
                        } => match joint_link.joint.data.as_revolute_mut() {
                            Some(rev_joint) => {
                                let motor = rev_joint.motor().expect("Joint should have a motor");

                                let stiffness = match stiffness {
                                    Some(stiffness) => stiffness,
                                    None => motor.stiffness,
                                };

                                let damping = match damping {
                                    Some(damping) => damping,
                                    None => motor.damping,
                                };

                                rev_joint.set_motor_position(position, stiffness, damping);
                            }
                            None => {
                                info!("Joint was not a revolute joint for revolute joint event");
                            }
                        },
                        MotorCommand::VelocityRevolute { velocity, damping } => {
                            match joint_link.joint.data.as_revolute_mut() {
                                Some(rev_joint) => {
                                    let motor =
                                        rev_joint.motor().expect("Joint should have a motor");

                                    let damping = match damping {
                                        Some(damping) => damping,
                                        None => motor.damping,
                                    };

                                    rev_joint.set_motor_velocity(velocity, damping);
                                }
                                None => {
                                    info!(
                                        "Joint was not a revolute joint for revolute joint event"
                                    );
                                }
                            }
                        }
                        MotorCommand::PositionSpherical { axis, position } => {
                            match joint_link.joint.data.as_spherical_mut() {
                                Some(spherical_joint) => {
                                    let motor = spherical_joint
                                        .motor(axis.into())
                                        .expect("Joint should have a motor");
                                    spherical_joint.set_motor_position(
                                        axis.into(),
                                        position,
                                        motor.stiffness,
                                        motor.damping,
                                    );
                                }
                                None => {
                                    info!(
                                        "Joint was not a spherical joint for spherical joint event"
                                    );
                                }
                            }
                        }
                        MotorCommand::VelocitySpherical { axis, velocity } => {
                            match joint_link.joint.data.as_spherical_mut() {
                                Some(spherical_joint) => {
                                    let motor = spherical_joint
                                        .motor(axis.into())
                                        .expect("Joint should have a motor");
                                    spherical_joint.set_motor_velocity(
                                        axis.into(),
                                        velocity,
                                        motor.damping,
                                    );
                                }
                                None => {
                                    info!(
                                        "Joint was not a spherical joint for spherical joint event"
                                    );
                                }
                            }
                        }
                        MotorCommand::PositionPrismatic {
                            position,
                            stiffness,
                            damping,
                        } => match joint_link.joint.data.as_prismatic_mut() {
                            Some(prismatic_joint) => {
                                let motor =
                                    prismatic_joint.motor().expect("Joint should have a motor");

                                let stiffness = match stiffness {
                                    Some(stiffness) => stiffness,
                                    None => motor.stiffness,
                                };

                                let damping = match damping {
                                    Some(damping) => damping,
                                    None => motor.damping,
                                };

                                prismatic_joint.set_motor_position(position, stiffness, damping);
                            }
                            None => {
                                info!("Joint was not a prismatic joint for prismatic joint event");
                            }
                        },
                        MotorCommand::VelocityPrismatic { velocity, damping } => {
                            match joint_link.joint.data.as_prismatic_mut() {
                                Some(prismatic_joint) => {
                                    let motor =
                                        prismatic_joint.motor().expect("Joint should have a motor");
                                    let damping = match damping {
                                        Some(damping) => damping,
                                        None => motor.damping,
                                    };
                                    prismatic_joint.set_motor_velocity(velocity, damping);
                                }
                                None => {
                                    info!(
                                        "Joint was not a prismatic joint for prismatic joint event"
                                    );
                                }
                            }
                        }
                        MotorCommand::SetStiffness { val } => {
                            if let Some(joint) = joint_link.joint.data.as_revolute_mut() {
                                let motor = joint.motor().expect("Joint should have a motor");
                                joint.set_motor(
                                    motor.target_pos,
                                    motor.target_vel,
                                    val,
                                    motor.damping,
                                );
                            } else if let Some(joint) = joint_link.joint.data.as_prismatic_mut() {
                                let motor = joint.motor().expect("Joint should have a motor");
                                joint.set_motor(
                                    motor.target_pos,
                                    motor.target_vel,
                                    val,
                                    motor.damping,
                                );
                            }
                        }
                        MotorCommand::SetDamping { val } => {
                            if let Some(joint) = joint_link.joint.data.as_revolute_mut() {
                                let motor = joint.motor().expect("Joint should have a motor");
                                joint.set_motor(
                                    motor.target_pos,
                                    motor.target_vel,
                                    motor.stiffness,
                                    val,
                                );
                            } else if let Some(joint) = joint_link.joint.data.as_prismatic_mut() {
                                let motor = joint.motor().expect("Joint should have a motor");
                                joint.set_motor(
                                    motor.target_pos,
                                    motor.target_vel,
                                    motor.stiffness,
                                    val,
                                );
                            }
                        }
                        MotorCommand::HoldPosition { stiffness } => {
                            if let Some(prismatic_joint) = prismatic_joint {
                                let joint =
                                    joint_link.joint.data.as_prismatic_mut().expect(
                                        "Prismatic component should have a prismatic joint",
                                    );
                                let motor = joint.motor().expect("Joint should have a motor");

                                let stiffness = match stiffness {
                                    Some(stiffness) => stiffness,
                                    None => motor.stiffness,
                                };
                                joint.set_motor_position(
                                    prismatic_joint.position(),
                                    stiffness,
                                    0.0,
                                );
                            } else if let Some(revolute_joint) = revolute_joint {
                                let joint =
                                    joint_link.joint.data.as_revolute_mut().expect(
                                        "Prismatic component should have a prismatic joint",
                                    );
                                let motor = joint.motor().expect("Joint should have a motor");

                                let stiffness = match stiffness {
                                    Some(stiffness) => stiffness,
                                    None => motor.stiffness,
                                };
                                joint.set_motor_position(revolute_joint.rotation(), stiffness, 0.0);
                            };
                        }
                    },
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::event::Events;

    use crate::rapier_extern::rapier::prelude as rapier;

    use super::*;
    use crate::conversions::IntoRapier;
    use crate::joint::fixed::FixedJoint;

    #[test]
    fn test_add_one_joint() {
        let mut world = World::default();
        let mut test_stage = SystemStage::parallel();
        test_stage.add_system(add_multibody_joints);

        world.init_resource::<KeskoRes<rapier::MultibodyJointSet>>();
        world.init_resource::<KeskoRes<Entity2JointHandle>>();

        let parent_body_handle = rapier::RigidBodyHandle::from_raw_parts(1, 0);
        let parent_entity = world.spawn(RigidBodyHandle(parent_body_handle)).id();

        let expected_transform = Transform::from_translation(Vec3::X);
        let joint = FixedJoint {
            parent: parent_entity,
            parent_anchor: expected_transform,
            child_anchor: expected_transform,
        };

        let child_body_handle = rapier::RigidBodyHandle::from_raw_parts(2, 0);
        let child_entity = world
            .spawn((joint, RigidBodyHandle(child_body_handle)))
            .id();

        let mut entity_body_map = Entity2Body::default();
        entity_body_map.insert(parent_entity, parent_body_handle);
        entity_body_map.insert(child_entity, child_body_handle);
        world.insert_resource(KeskoRes(entity_body_map));

        test_stage.run(&mut world);

        // only one entity with joint handle, should only be the child
        let mut query = world.query::<&MultibodyJointHandle>();
        assert_eq!(query.iter(&world).len(), 1);

        let joint_handle = query
            .get(&world, child_entity)
            .expect("Failed to get JointHandle from query");

        let multibody_joint_set = world
            .get_resource::<KeskoRes<rapier::MultibodyJointSet>>()
            .expect("Could not get ImpulseJointSet")
            .clone();

        // only on element in set
        assert_eq!(
            multibody_joint_set
                .attached_joints(parent_body_handle)
                .count(),
            1
        );
        assert_eq!(
            multibody_joint_set
                .attached_joints(child_body_handle)
                .count(),
            1
        );

        let (multibody, link_id) = multibody_joint_set
            .get(joint_handle.0)
            .expect("Could not get joint from joint set");

        assert_eq!(multibody.root().rigid_body_handle(), parent_body_handle);
        assert_eq!(
            multibody.link(link_id).unwrap().rigid_body_handle(),
            child_body_handle
        );

        let joint = multibody.link(link_id).unwrap().joint;
        assert!(joint.data.as_fixed().is_some());
        assert_eq!(joint.data.local_frame1, expected_transform.into_rapier());
        assert_eq!(joint.data.local_frame2, expected_transform.into_rapier());
    }

    fn setup_joint_motor() -> (
        World,
        rapier::RigidBodyHandle,
        rapier::RigidBodyHandle,
        rapier::MultibodyJointSet,
    ) {
        let world = World::new();

        let joint_set = rapier::MultibodyJointSet::default();
        let mut body_set = rapier::RigidBodySet::default();

        let body1 = rapier::RigidBodyBuilder::new(rapier::RigidBodyType::Dynamic).build();
        let body2 = rapier::RigidBodyBuilder::new(rapier::RigidBodyType::Dynamic).build();

        let body_handle1 = body_set.insert(body1);
        let body_handle2 = body_set.insert(body2);

        return (world, body_handle1, body_handle2, joint_set);
    }

    #[test]
    fn test_set_joint_velocity_revolute() {
        let (mut world, body_handle1, body_handle2, mut joint_set) = setup_joint_motor();

        // create and insert joint
        let expected_vel = 2.3;
        let expected_factor = 3.4;
        let joint = rapier::RevoluteJointBuilder::new(rapier::Vector::x_axis())
            .motor(0.0, expected_vel, 0.0, expected_factor)
            .build();
        let joint_handle = joint_set
            .insert(body_handle1, body_handle2, joint, true)
            .unwrap();

        world.insert_resource(KeskoRes(joint_set));
        let entity = world.spawn(MultibodyJointHandle(joint_handle)).id();

        // Setup and send test event for setting the velocity
        let mut events = Events::<JointMotorEvent>::default();
        events.send(JointMotorEvent {
            entity: entity,
            command: MotorCommand::VelocityRevolute {
                velocity: expected_vel,
                damping: None,
            },
        });
        world.insert_resource(events);

        // Run stage
        let test_stage = SystemStage::parallel();
        test_stage
            .with_system(update_joint_motors_system)
            .run(&mut world);

        // get result
        let res_set = world
            .get_resource::<KeskoRes<rapier::MultibodyJointSet>>()
            .expect("Could not get impulse joint set");
        let (multibody, link_id) = res_set
            .get(joint_handle)
            .expect("Could not get joint from joint set");

        // test correct values
        let res_joint = multibody.link(link_id).unwrap().joint;
        assert_eq!(
            res_joint
                .data
                .as_revolute()
                .unwrap()
                .motor()
                .unwrap()
                .target_vel,
            expected_vel
        );
        assert_eq!(
            res_joint
                .data
                .as_revolute()
                .unwrap()
                .motor()
                .unwrap()
                .damping,
            expected_factor
        );
    }

    #[test]
    fn test_set_joint_position_revolute() {
        let (mut world, body_handle1, body_handle2, mut joint_set) = setup_joint_motor();

        // Setup and send test event for setting the velocity
        let expected_pos = 2.3;
        let expected_damping = 3.4;
        let expected_stiffness = 4.5;
        let mut events = Events::<JointMotorEvent>::default();

        // create and insert joint
        let joint = rapier::RevoluteJointBuilder::new(rapier::Vector::x_axis())
            .motor(expected_pos, 0.0, expected_stiffness, expected_damping)
            .build();
        let joint_handle = joint_set
            .insert(body_handle1, body_handle2, joint, true)
            .unwrap();

        world.insert_resource(KeskoRes(joint_set));
        let entity = world.spawn(MultibodyJointHandle(joint_handle)).id();

        events.send(JointMotorEvent {
            entity,
            command: MotorCommand::PositionRevolute {
                position: expected_pos,
                stiffness: None,
                damping: None,
            },
        });
        world.insert_resource(events);

        // Run stage
        let test_stage = SystemStage::parallel();
        test_stage
            .with_system(update_joint_motors_system)
            .run(&mut world);

        // get result
        let res_set = world
            .get_resource::<KeskoRes<rapier::MultibodyJointSet>>()
            .expect("Could not get impulse joint set");
        let (multibody, link_id) = res_set
            .get(joint_handle)
            .expect("Could not get joint from joint set");

        // test correct values
        let res_joint = multibody.link(link_id).unwrap().joint;
        assert_eq!(
            res_joint
                .data
                .as_revolute()
                .unwrap()
                .motor()
                .unwrap()
                .target_pos,
            expected_pos
        );
        assert_eq!(
            res_joint
                .data
                .as_revolute()
                .unwrap()
                .motor()
                .unwrap()
                .damping,
            expected_damping
        );
        assert_eq!(
            res_joint
                .data
                .as_revolute()
                .unwrap()
                .motor()
                .unwrap()
                .stiffness,
            expected_stiffness
        );
    }

    #[test]
    fn test_set_joint_velocity_spherical() {
        let (mut world, body_handle1, body_handle2, mut joint_set) = setup_joint_motor();

        // create and insert joint
        let expected_vel = 2.3;
        let expected_factor = 3.4;
        let test_axis = KeskoAxis::AngX;
        let joint = rapier::SphericalJointBuilder::new()
            .motor(test_axis.into(), 0.0, expected_vel, 0.0, expected_factor)
            .build();
        let joint_handle = joint_set
            .insert(body_handle1, body_handle2, joint, true)
            .unwrap();

        world.insert_resource(KeskoRes(joint_set));
        let entity = world.spawn(MultibodyJointHandle(joint_handle)).id();

        // Setup and send test event for setting the velocity
        let mut events = Events::<JointMotorEvent>::default();
        events.send(JointMotorEvent {
            entity: entity,
            command: MotorCommand::VelocitySpherical {
                velocity: expected_vel,
                axis: test_axis,
            },
        });
        world.insert_resource(events);

        // Run stage
        let test_stage = SystemStage::parallel();
        test_stage
            .with_system(update_joint_motors_system)
            .run(&mut world);

        // get result
        let res_set = world
            .get_resource::<KeskoRes<rapier::MultibodyJointSet>>()
            .expect("Could not get impulse joint set");
        let (multibody, link_id) = res_set
            .get(joint_handle)
            .expect("Could not get joint from joint set");

        // test correct values
        let res_joint = multibody.link(link_id).unwrap().joint;
        assert_eq!(
            res_joint
                .data
                .as_spherical()
                .unwrap()
                .motor(test_axis.into())
                .unwrap()
                .target_vel,
            expected_vel
        );
        assert_eq!(
            res_joint
                .data
                .as_spherical()
                .unwrap()
                .motor(test_axis.into())
                .unwrap()
                .damping,
            expected_factor
        );
    }

    #[test]
    fn test_set_joint_position_spherical() {
        let (mut world, body_handle1, body_handle2, mut joint_set) = setup_joint_motor();

        let expected_pos = 2.3;
        let expected_damping = 3.4;
        let expected_stiffness = 4.5;
        let test_axis = KeskoAxis::AngY;

        // create and insert joint
        let joint = rapier::SphericalJointBuilder::new()
            .motor(
                test_axis.into(),
                expected_pos,
                0.0,
                expected_stiffness,
                expected_damping,
            )
            .build();
        let joint_handle = joint_set
            .insert(body_handle1, body_handle2, joint, true)
            .unwrap();

        world.insert_resource(KeskoRes(joint_set));
        let entity = world.spawn(MultibodyJointHandle(joint_handle)).id();

        // Setup and send test event for setting the velocity
        let mut events = Events::<JointMotorEvent>::default();
        events.send(JointMotorEvent {
            entity,
            command: MotorCommand::PositionSpherical {
                position: expected_pos,
                axis: test_axis,
            },
        });
        world.insert_resource(events);

        // Run stage
        let test_stage = SystemStage::parallel();
        test_stage
            .with_system(update_joint_motors_system)
            .run(&mut world);

        // get result
        let res_set = world
            .get_resource::<KeskoRes<rapier::MultibodyJointSet>>()
            .expect("Could not get impulse joint set");
        let (multibody, link_id) = res_set
            .get(joint_handle)
            .expect("Could not get joint from joint set");

        // test correct values
        let res_joint = multibody.link(link_id).unwrap().joint;
        assert_eq!(
            res_joint
                .data
                .as_spherical()
                .unwrap()
                .motor(test_axis.into())
                .unwrap()
                .target_pos,
            expected_pos
        );
        assert_eq!(
            res_joint
                .data
                .as_spherical()
                .unwrap()
                .motor(test_axis.into())
                .unwrap()
                .damping,
            expected_damping
        );
        assert_eq!(
            res_joint
                .data
                .as_spherical()
                .unwrap()
                .motor(test_axis.into())
                .unwrap()
                .stiffness,
            expected_stiffness
        );
    }
}
