pub mod spherical;
pub mod fixed;
pub mod revolute;
pub mod prismatic;

use bevy::prelude::*;
use rapier3d::prelude as rapier;
use rapier3d::dynamics::GenericJoint;
pub use rapier3d::prelude::{JointLimits, JointAxis};
use serde::{Serialize, Deserialize};
use fnv::FnvHashMap;

use crate::rigid_body::{Entity2BodyHandle, RigidBodyHandle};
use crate::conversions::{IntoBevy, IntoRapier};


pub type Entity2JointHandle = FnvHashMap<Entity, rapier::MultibodyJointHandle>;

/// trait for converting an axis into a unit vector
pub(crate) trait AxisIntoVec {
    fn into_unitvec(self) -> rapier::UnitVector<rapier::Real>;
}
impl AxisIntoVec for Axis {
    fn into_unitvec(self) -> rapier::UnitVector<rapier::Real> {
        match self {
            Axis::X => rapier::Vector::x_axis(),
            Axis::Y => rapier::Vector::y_axis(),
            Axis::Z => rapier::Vector::z_axis(),
            Axis::NegX => -rapier::Vector::x_axis(),
            Axis::NegY => -rapier::Vector::y_axis(),
            Axis::NegZ => -rapier::Vector::z_axis()
        }
    }
}

/// Override Rapiers JointAxis since it did not inlcude negative axis
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Axis {
    X,
    Y,
    Z,
    NegX,
    NegY,
    NegZ
}

/// used to send joint state outside Kesko
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum JointState {
    Spherical{ 
        angle: Vec3
    },
    Revolute{ 
        axis: Axis,
        angle: f32,
    }
}


/// Enum to indicate joint type
#[derive(Debug)]
pub enum JointType {
    Fixed,
    Revolute,
    Prismatic,
    Spherical
}

pub trait JointTrait {
    fn parent_anchor(&self) -> Transform;
    fn child_anchor(&self) -> Transform;
    fn get_axis(&self) -> Option<Axis>;
}

/// Component for connecting two bodies with a joint. This component should be added to the body that wants to connect
/// to another body
#[derive(Component)]
pub struct Joint {
    /// entity that the entity with the component should attach to
    parent: Entity,
    /// Rapier joint
    joint: GenericJoint,
    /// local joint_orientation
    local_joint_rot: Quat,
    /// parent anchor
    parent_anchor: Transform,
    /// child anchor
    child_anchor: Transform,
    /// axis that the joint is constrained to move around/along
    axis: Option<Axis>
}

impl Joint {
    pub fn new(parent: Entity, joint: impl Into<GenericJoint> + JointTrait) -> Self 
    {

        let parent_anchor = joint.parent_anchor();
        let child_anchor = joint.child_anchor();
        let axis = joint.get_axis();

        Self {
            parent,
            joint: joint.into(),
            local_joint_rot: Quat::default(),
            parent_anchor,
            child_anchor,
            axis
        }
    }

    pub fn get_type(&self) -> JointType {
        let joint_type = if self.joint.as_fixed().is_some() {
            JointType::Fixed
        } else if self.joint.as_spherical().is_some() {
            JointType::Spherical
        } else if self.joint.as_prismatic().is_some() {
            JointType::Prismatic
        } else {
            JointType::Revolute
        };
        joint_type
    }

    pub fn get_limits(&self) -> Option<JointLimits<f32>> {
        if let Some(rev_joint) = self.joint.as_revolute() {
            if let Some(limits) = rev_joint.limits() {
                return Some(*limits);
            }
        } else if let Some(prism_joint) = self.joint.as_prismatic() {
            if let Some(limits) = prism_joint.limits() {
                return Some(*limits);
            }
        }
        None
    }

    pub fn get_rot_x(&self) -> f32 {
        self.local_joint_rot.to_euler(EulerRot::XYZ).0
    }

    pub fn get_rot_y(&self) -> f32 {
        self.local_joint_rot.to_euler(EulerRot::XYZ).1
    }

    pub fn get_rot_z(&self) -> f32 {
        self.local_joint_rot.to_euler(EulerRot::XYZ).2
    }

    pub fn get_state(&self) -> JointState {
        match self.axis {
            Some(axis) => {
                match axis {
                    Axis::X | Axis::NegX => JointState::Revolute{ axis, angle: self.get_rot_x() },
                    Axis::Y | Axis::NegY => JointState::Revolute{ axis, angle: self.get_rot_y() },
                    Axis::Z | Axis::NegZ => JointState::Revolute{ axis, angle: self.get_rot_z() },
                }
            }
            None => JointState::Spherical{ angle: self.local_joint_rot.to_euler(EulerRot::XYZ).into() }
        }
    }
}

/// Component that holds a handle to an entity's joint.
/// Used to indicate that the joint has been added both in Bevy and Rapier
#[derive(Component, Debug)]
pub struct MultibodyJointHandle(pub(crate) rapier::MultibodyJointHandle);


/// System to add multibody joints between bodies
#[allow(clippy::type_complexity)]
pub(crate) fn add_multibody_joints(
    mut commands: Commands,
    mut entity_2_joint_handle: ResMut<Entity2JointHandle>,
    mut multibody_joint_set: ResMut<rapier::MultibodyJointSet>,
    entity_body_map: Res<Entity2BodyHandle>,
    query: Query<(Entity, &Joint), (With<RigidBodyHandle>, Without<MultibodyJointHandle>)>
) {

    for (entity, joint_comp) in query.iter() {
        let joint_handle = multibody_joint_set.insert(
            *entity_body_map.get(&joint_comp.parent).unwrap(),
            *entity_body_map.get(&entity).unwrap(),
            joint_comp.joint,
            true
        ).unwrap();

        entity_2_joint_handle.insert(entity, joint_handle);
        commands.entity(entity).insert(MultibodyJointHandle(joint_handle));
    }
}

/// System that updates the joints positions and velocities
/// 
/// TODO: Add velocity
pub(crate) fn update_joint_pos_system(
    multibody_joint_set: Res<rapier::MultibodyJointSet>,
    mut handle_query: Query<(&mut Joint, &MultibodyJointHandle), (With<MultibodyJointHandle>, With<Joint>)>,
) {
    for (mut joint, handle) in handle_query.iter_mut() {
        if let Some((mb, link_index)) = multibody_joint_set.get(handle.0) {
            if let Some(link) = mb.link(link_index) {

                // we have the joint transformation in the parent frame
                let body_to_parent = link.joint().body_to_parent();

                // convert to local orientation by multiplying by the inverse of anchor's rotation
                joint.local_joint_rot = (
                    joint.parent_anchor.rotation.into_rapier().inverse() * joint.child_anchor.rotation.into_rapier().inverse() * body_to_parent.rotation
                ).into_bevy();
            }
        }
    }
}

/// Event for communicate joint motor positions and velocities
#[derive(Debug)]
pub struct JointMotorEvent {
    pub entity: Entity,
    pub action: MotorAction
}

#[derive(Debug)]
pub enum MotorAction {
    PositionRevolute {
        position: f32
    },
    VelocityRevolute {
        velocity: f32
    },
    PositionSpherical {
        position: f32,
        axis: rapier::JointAxis
    },
    VelocitySpherical {
        velocity: f32,
        axis: rapier::JointAxis,
    },
    PositionPrismatic {
        position: f32,
    },
    VelocityPrismatic {
        velocity: f32,
    }
}

pub(crate) fn update_joint_motors_system(
    mut joint_event: EventReader<JointMotorEvent>,
    mut joint_set: ResMut<rapier::MultibodyJointSet>,
    mut query: Query<&MultibodyJointHandle, With<MultibodyJointHandle>>
) {

    for event in joint_event.iter() {
        match query.get_mut(event.entity) {
            Err(e) => error!("{:?}", e),
            Ok(joint_handle) => match joint_set.get_mut(joint_handle.0) {

                None => { error!("Could not get joint from joint set")},
                Some((mb, id)) => match mb.link_mut(id) {

                    None => { error!("Could not get multi joint link from multibody"); },
                    Some(joint_link) => {

                        match event.action {
                            MotorAction::PositionRevolute { position } => {
                                match joint_link.joint.data.as_revolute_mut() {
                                    Some(rev_joint) => {
                                        let motor = rev_joint.motor().expect("Joint should have a motor");
                                        rev_joint.set_motor_position(position, motor.stiffness, motor.damping); 
                                    },
                                    None => { info!("Joint was not a revolute joint for revolute joint event"); }
                                }
                            },
                            MotorAction::VelocityRevolute { velocity } => {
                                match joint_link.joint.data.as_revolute_mut() {
                                    Some(rev_joint) => { 
                                        let motor = rev_joint.motor().expect("Joint should have a motor");
                                        rev_joint.set_motor_velocity(velocity, 1.0); 
                                    },
                                    None => { info!("Joint was not a revolute joint for revolute joint event"); }
                                }
                            },
                            MotorAction::PositionSpherical {axis, position } => {
                                match joint_link.joint.data.as_spherical_mut() {
                                    Some(spherical_joint) => {
                                        let motor = spherical_joint.motor(axis).expect("Joint should have a motor");
                                        spherical_joint.set_motor_position(axis, position, motor.stiffness, motor.damping); 
                                    },
                                    None => { info!("Joint was not a spherical joint for spherical joint event"); }
                                }
                            },
                            MotorAction::VelocitySpherical {axis, velocity } => {
                                match joint_link.joint.data.as_spherical_mut() {
                                    Some(spherical_joint) => { 
                                        let motor = spherical_joint.motor(axis).expect("Joint should have a motor");
                                        spherical_joint.set_motor_velocity(axis, velocity, motor.damping); 
                                    },
                                    None => { info!("Joint was not a spherical joint for spherical joint event"); }
                                }
                            },
                            MotorAction::PositionPrismatic { position } => {
                                match joint_link.joint.data.as_prismatic_mut() {
                                    Some(prismatic_joint) => { 
                                        let motor = prismatic_joint.motor().expect("Joint should have a motor");
                                        prismatic_joint.set_motor_position(position, motor.stiffness, motor.damping);
                                    }
                                    None => { info!("Joint was not a prismatic joint for prismatic joint event"); }
                                }
                            },
                            MotorAction::VelocityPrismatic { velocity } => {
                                match joint_link.joint.data.as_prismatic_mut() {
                                    Some(prismatic_joint) => { 
                                        let motor = prismatic_joint.motor().expect("Joint should have a motor");
                                        prismatic_joint.set_motor_velocity(velocity, motor.damping); 
                                    }
                                    None => { info!("Joint was not a prismatic joint for prismatic joint event"); }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use bevy::ecs::event::Events;

    use crate::joint::fixed::FixedJoint;
    use crate::conversions::IntoRapier;
    use super::*;

    #[test]
    fn test_add_one_joint() {

        let mut world = World::default();
        let mut test_stage = SystemStage::parallel();
        test_stage.add_system(add_multibody_joints);

        world.init_resource::<rapier3d::prelude::MultibodyJointSet>();
        world.init_resource::<Entity2JointHandle>();

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

        let mut entity_body_map = Entity2BodyHandle::default();
        entity_body_map.insert(parent_entity, parent_body_handle);
        entity_body_map.insert(child_entity, child_body_handle);
        world.insert_resource(entity_body_map);

        test_stage.run(&mut world);

        let multibody_joint_set = world
            .get_resource::<rapier3d::prelude::MultibodyJointSet>()
            .expect("Could not get ImpulseJointSet").clone();

        // only on element in set
        assert_eq!(multibody_joint_set.attached_joints(parent_body_handle).count(), 1);
        assert_eq!(multibody_joint_set.attached_joints(child_body_handle).count(), 1);

        // only one entity with joint handle, should only be the child
        let mut query = world.query::<&MultibodyJointHandle>();
        assert_eq!(query.iter(&world).len(), 1);

        let joint_handle = query.get(&world, child_entity)
            .expect("Failed to get JointHandle from query");

        let (multibody, link_id) = multibody_joint_set.get(joint_handle.0).expect("Could not get joint from joint set");

        assert_eq!(multibody.root().rigid_body_handle(), parent_body_handle);
        assert_eq!(multibody.link(link_id).unwrap().rigid_body_handle(), child_body_handle);

        let joint = multibody.link(link_id).unwrap().joint;
        assert!(joint.data.as_fixed().is_some());
        assert_eq!(joint.data.local_frame1, expected_transform.into_rapier());
        assert_eq!(joint.data.local_frame2, expected_transform.into_rapier());

    }

    fn setup_joint_motor() -> (World, rapier::RigidBodyHandle, rapier::RigidBodyHandle, rapier::MultibodyJointSet) {

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
        let joint = rapier::RevoluteJointBuilder::new(rapier::Vector::x_axis()).build();
        let joint_handle = joint_set.insert(body_handle1, body_handle2, joint, true).unwrap();

        world.insert_resource(joint_set);
        let entity = world.spawn().insert(MultibodyJointHandle(joint_handle)).id();

        // Setup and send test event for setting the velocity
        let expected_vel = 2.3;
        let expected_factor = 3.4;
        let mut events = Events::<JointMotorEvent>::default();
        events.send(JointMotorEvent { entity: entity, action: MotorAction::VelocityRevolute { velocity: expected_vel }});
        world.insert_resource(events);

        // Run stage
        let test_stage = SystemStage::parallel();
        test_stage.with_system(update_joint_motors_system).run(&mut world);
        
        // get result
        let res_set = world.get_resource::<rapier::MultibodyJointSet>().expect("Could not get impulse joint set");
        let (multibody, link_id) = res_set.get(joint_handle).expect("Could not get joint from joint set");

        // test correct values
        let res_joint = multibody.link(link_id).unwrap().joint;
        assert_eq!(res_joint.data.as_revolute().unwrap().motor().unwrap().target_vel, expected_vel);
        assert_eq!(res_joint.data.as_revolute().unwrap().motor().unwrap().damping, expected_factor);

    }

    #[test]
    fn test_set_joint_position_revolute() {

        let (mut world, body_handle1, body_handle2, mut joint_set) = setup_joint_motor();

        // create and insert joint
        let joint = rapier::RevoluteJointBuilder::new(rapier::Vector::x_axis()).build();
        let joint_handle = joint_set.insert(body_handle1, body_handle2, joint, true).unwrap();

        world.insert_resource(joint_set);
        let entity = world.spawn().insert(MultibodyJointHandle(joint_handle)).id();

        // Setup and send test event for setting the velocity
        let expected_pos = 2.3;
        let expected_damping = 3.4;
        let expected_stiffness = 4.5;
        let mut events = Events::<JointMotorEvent>::default();
        events.send(JointMotorEvent { 
            entity, 
            action: MotorAction::PositionRevolute { 
                position: expected_pos
            }
        });
        world.insert_resource(events);

        // Run stage
        let test_stage = SystemStage::parallel();
        test_stage.with_system(update_joint_motors_system).run(&mut world);
        
        // get result
        let res_set = world.get_resource::<rapier::MultibodyJointSet>().expect("Could not get impulse joint set");
        let (multibody, link_id) = res_set.get(joint_handle).expect("Could not get joint from joint set");

        // test correct values
        let res_joint = multibody.link(link_id).unwrap().joint;
        assert_eq!(res_joint.data.as_revolute().unwrap().motor().unwrap().target_pos, expected_pos);
        assert_eq!(res_joint.data.as_revolute().unwrap().motor().unwrap().damping, expected_damping);
        assert_eq!(res_joint.data.as_revolute().unwrap().motor().unwrap().stiffness, expected_stiffness);

    }

    #[test]
    fn test_set_joint_velocity_spherical() {

        let (mut world, body_handle1, body_handle2, mut joint_set) = setup_joint_motor();

        // create and insert joint
        let joint = rapier::SphericalJointBuilder::new().build();
        let joint_handle = joint_set.insert(body_handle1, body_handle2, joint, true).unwrap();

        world.insert_resource(joint_set);
        let entity = world.spawn().insert(MultibodyJointHandle(joint_handle)).id();

        // Setup and send test event for setting the velocity
        let expected_vel = 2.3;
        let expected_factor = 3.4;
        let test_axis = rapier::JointAxis::AngX;
        let mut events = Events::<JointMotorEvent>::default();
        events.send(JointMotorEvent { 
            entity: entity, 
            action: MotorAction::VelocitySpherical {
                velocity: expected_vel, 
                axis: test_axis
            }
        });
        world.insert_resource(events);

        // Run stage
        let test_stage = SystemStage::parallel();
        test_stage.with_system(update_joint_motors_system).run(&mut world);
        
        // get result
        let res_set = world.get_resource::<rapier::MultibodyJointSet>().expect("Could not get impulse joint set");
        let (multibody, link_id) = res_set.get(joint_handle).expect("Could not get joint from joint set");

        // test correct values
        let res_joint = multibody.link(link_id).unwrap().joint;
        assert_eq!(res_joint.data.as_spherical().unwrap().motor(test_axis).unwrap().target_vel, expected_vel);
        assert_eq!(res_joint.data.as_spherical().unwrap().motor(test_axis).unwrap().damping, expected_factor);

    }

    #[test]
    fn test_set_joint_position_spherical() {

        let (mut world, body_handle1, body_handle2, mut joint_set) = setup_joint_motor();

        // create and insert joint
        let joint = rapier::SphericalJointBuilder::new().build();
        let joint_handle = joint_set.insert(body_handle1, body_handle2, joint, true).unwrap();

        world.insert_resource(joint_set);
        let entity = world.spawn().insert(MultibodyJointHandle(joint_handle)).id();

        // Setup and send test event for setting the velocity
        let expected_pos = 2.3;
        let expected_damping = 3.4;
        let expected_stiffness = 4.5;
        let test_axis = rapier::JointAxis::AngY;
        let mut events = Events::<JointMotorEvent>::default();
        events.send(JointMotorEvent { 
            entity, 
            action: MotorAction::PositionSpherical {
                position: expected_pos, 
                axis: test_axis
            }
        });
        world.insert_resource(events);

        // Run stage
        let test_stage = SystemStage::parallel();
        test_stage.with_system(update_joint_motors_system).run(&mut world);
        
        // get result
        let res_set = world.get_resource::<rapier::MultibodyJointSet>().expect("Could not get impulse joint set");
        let (multibody, link_id) = res_set.get(joint_handle).expect("Could not get joint from joint set");

        // test correct values
        let res_joint = multibody.link(link_id).unwrap().joint;
        assert_eq!(res_joint.data.as_spherical().unwrap().motor(test_axis).unwrap().target_pos, expected_pos);
        assert_eq!(res_joint.data.as_spherical().unwrap().motor(test_axis).unwrap().damping, expected_damping);
        assert_eq!(res_joint.data.as_spherical().unwrap().motor(test_axis).unwrap().stiffness, expected_stiffness);

    }

}