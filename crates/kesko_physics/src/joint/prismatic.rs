use bevy::prelude::*;
use crate::rapier_extern::rapier::prelude as rapier;
use crate::conversions::IntoRapier;

use super::{AxisIntoVec, KeskoAxis, JointState};


#[derive(Component, Clone, Copy)]
pub struct PrismaticJoint {
    pub parent: Entity,
    pub parent_anchor: Transform,
    pub child_anchor: Transform,
    pub axis: KeskoAxis,
    pub limits: Option<Vec2>,
    pub stiffness: rapier::Real,
    pub damping: rapier::Real,
    pub max_motor_force: rapier::Real,

    position: rapier::Real
}

impl PrismaticJoint {
    pub fn attach_to(parent: Entity) -> Self {
        Self {
            parent,
            parent_anchor: Transform::default(), 
            child_anchor: Transform::default(), 
            axis: KeskoAxis::X, 
            limits: None,
            damping: 0.0,
            stiffness: 0.0,
            max_motor_force: rapier::Real::MAX,

            position: 0.0
        }
    }

    pub fn with_parent_anchor(mut self, parent_anchor: Transform) -> Self {
        self.parent_anchor = parent_anchor;
        self
    }

    pub fn with_child_anchor(mut self, child_anchor: Transform) -> Self {
        self.child_anchor = child_anchor;
        self
    }

    pub fn with_axis(mut self, axis: KeskoAxis) -> Self {
        self.axis = axis;
        self
    }

    pub fn with_limits(mut self, limits: Vec2) -> Self {
        self.limits = Some(limits);
        self
    }

    pub fn with_motor_params(mut self, stiffness: rapier::Real, damping: rapier::Real) -> Self {
        self.stiffness = stiffness;
        self.damping = damping;
        self
    }

    pub fn with_max_motor_force(mut self, max_motor_force: rapier::Real) -> Self {
        self.max_motor_force = max_motor_force;
        self
    }

    pub fn update_position(&mut self, translation: Vec3) {
        match self.axis {
            KeskoAxis::X => self.position = translation.x as rapier::Real,
            KeskoAxis::NegX => self.position = -translation.x as rapier::Real,
            KeskoAxis::Y => self.position = translation.y as rapier::Real,
            KeskoAxis::NegY => self.position = -translation.y as rapier::Real,
            KeskoAxis::Z => self.position = translation.z as rapier::Real,
            KeskoAxis::NegZ => self.position = -translation.z as rapier::Real,
            _ => error!("Prismatic joint does not have a valid axis")
        }
    }

    pub fn position(&self) -> rapier::Real {
        self.position
    }

    pub fn state(&self) -> JointState {
        JointState::Prismatic { axis: self.axis, position: self.position }
    }
}

impl From<PrismaticJoint> for rapier::GenericJoint {
    fn from(joint: PrismaticJoint) -> rapier::GenericJoint {
        let mut builder = rapier::PrismaticJointBuilder::new(joint.axis.into_unitvec())
            .local_anchor1(joint.parent_anchor.translation.into_rapier())
            .local_anchor2(joint.child_anchor.translation.into_rapier());
        
        if joint.stiffness > 0.0 || joint.damping > 0.0 {
            builder = builder.set_motor(0.0, 0.0, joint.stiffness, joint.damping).motor_max_force(joint.max_motor_force);
        }

        // if let Some(limits) = joint.limits {
        //     builder = builder.limits(limits.into());
        // }

        let mut generic: rapier::GenericJoint = builder.into();
        generic.local_frame1.rotation = joint.parent_anchor.rotation.into_rapier() * generic.local_frame1.rotation;
        generic.local_frame1.rotation = joint.child_anchor.rotation.into_rapier() * generic.local_frame1.rotation;
        generic
    }
}


#[cfg(test)]
mod tests {
    use bevy::math::Vec2;
    use bevy::prelude::{Transform, Vec3, Entity};
    use crate::rapier_extern::rapier::dynamics::JointAxis;
    use crate::rapier_extern::rapier::prelude::GenericJoint;
    use crate::{IntoRapier, joint::KeskoAxis};
    use super::PrismaticJoint;

    #[test]
    fn only_translation() {

        let expected_parent_transform = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let expected_child_transform = Transform::from_translation(Vec3::new(4.0, 5.0, 6.0));

        let joint = PrismaticJoint::attach_to(Entity::from_raw(0))
            .with_parent_anchor(expected_parent_transform)
            .with_child_anchor(expected_child_transform);

        let generic: GenericJoint = joint.into();

        assert!(generic.as_prismatic().is_some());
        assert_eq!(generic.local_anchor1(), expected_parent_transform.translation.into_rapier());
        assert_eq!(generic.local_anchor2(), expected_child_transform.translation.into_rapier());
    }

    #[test]
    fn with_limits() {

        let limit_min = -1.0;
        let limit_max = 1.0;

        let joint = PrismaticJoint::attach_to(Entity::from_raw(0))
            .with_axis(KeskoAxis::X)
            .with_limits(Vec2::new(-1.0, 1.0));

        let generic: GenericJoint = joint.into();

        println!("{:?}", generic.limits);

        let limits = generic.limits(JointAxis::X).expect("No limits for X");
        assert_eq!(limits.min, limit_min);
        assert_eq!(limits.max, limit_max);
    }

    #[test]
    fn no_limits() {

        let joint = PrismaticJoint::attach_to(Entity::from_raw(0));
        let generic: GenericJoint = joint.into();

        assert!(generic.limits(JointAxis::AngX).is_none());
        assert!(generic.limits(JointAxis::AngY).is_none());
        assert!(generic.limits(JointAxis::AngZ).is_none());
        assert!(generic.limits(JointAxis::X).is_none());
        assert!(generic.limits(JointAxis::Y).is_none());
        assert!(generic.limits(JointAxis::Z).is_none());
    }
}
