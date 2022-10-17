use bevy::prelude::*;
use rapier3d::prelude::*;

use crate::conversions::IntoRapier;

use super::{AxisIntoVec, KeskoAxis, JointState};


#[derive(Component, Clone, Copy)]
pub struct RevoluteJoint {
    pub parent: Entity,
    pub parent_anchor: Transform,
    pub child_anchor: Transform,
    pub axis: KeskoAxis,
    pub limits: Option<Vec2>,
    pub damping: f32,
    pub stiffness: f32,
    pub max_motor_force: Real,

    rotation: f32
}

impl RevoluteJoint {
    pub fn attach_to(parent: Entity) -> Self {
        Self { 
            parent,
            parent_anchor: Transform::default(), 
            child_anchor: Transform::default(), 
            axis: KeskoAxis::X, 
            limits: None,
            damping: 0.0,
            stiffness: 0.0,
            max_motor_force: Real::MAX,
            rotation: 0.0
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

    pub fn with_motor_params(mut self, stiffness: f32, damping: f32) -> Self {
        self.stiffness = stiffness;
        self.damping = damping;
        self
    }

    pub fn with_max_motor_force(mut self, max_motor_force: f32) -> Self {
        self.max_motor_force = max_motor_force;
        self
    }

    pub fn update_rotation(&mut self, rot: Quat) {
        // convert to local orientation by multiplying by the inverse of anchor's rotation
        let rotation = (self.parent_anchor.rotation.inverse() * self.child_anchor.rotation.inverse() * rot).to_euler(EulerRot::XYZ);
        match self.axis {
            KeskoAxis::X | KeskoAxis::NegX => self.rotation = rotation.0,
            KeskoAxis::Y | KeskoAxis::NegY => self.rotation = rotation.1,
            KeskoAxis::Z | KeskoAxis::NegZ => self.rotation = rotation.2,
            _ => {}
        }
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn state(&self) -> JointState {
        JointState::Revolute {
            axis: self.axis,
            angle: self.rotation
        }
    }
}

impl From<RevoluteJoint> for GenericJoint {
    fn from(joint: RevoluteJoint) -> GenericJoint {
        
        let mut builder = RevoluteJointBuilder::new(joint.axis.into_unitvec())
            .local_anchor1(joint.parent_anchor.translation.into_rapier())
            .local_anchor2(joint.child_anchor.translation.into_rapier());

        if joint.stiffness > 0.0 || joint.damping > 0.0 {
            builder = builder.motor(0.0, 0.0, joint.stiffness, joint.damping);
        }

        builder = builder.motor_max_force(joint.max_motor_force);

        if let Some(limits) = joint.limits {
            builder = builder.limits(limits.into());
        }

        let mut generic: GenericJoint = builder.into();
        generic.local_frame1.rotation = joint.parent_anchor.rotation.into_rapier() * generic.local_frame1.rotation;
        generic.local_frame1.rotation = joint.child_anchor.rotation.into_rapier() * generic.local_frame1.rotation;
        generic
    }
}


#[cfg(test)]
mod tests {

    use bevy::math::Vec2;
    use bevy::prelude::{Transform, Vec3, Entity};
    use rapier3d::dynamics::JointAxis;
    use rapier3d::prelude::GenericJoint;
    use crate::{IntoRapier, joint::KeskoAxis};
    use super::RevoluteJoint;

    #[test]
    fn only_translation() {

        let expected_parent_transform = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let expected_child_transform = Transform::from_translation(Vec3::new(4.0, 5.0, 6.0));

        let joint = RevoluteJoint::attach_to(Entity::from_raw(0))
            .with_parent_anchor(expected_parent_transform)
            .with_child_anchor(expected_child_transform)
            .with_axis(KeskoAxis::X);

        let generic: GenericJoint = joint.into();

        assert!(generic.as_revolute().is_some());
        assert_eq!(generic.local_anchor1(), expected_parent_transform.translation.into_rapier());
        assert_eq!(generic.local_anchor2(), expected_child_transform.translation.into_rapier());
    }

    #[test]
    fn with_limits() {

        let limit_min = -1.0;
        let limit_max = 1.0;

        let joint = RevoluteJoint::attach_to(Entity::from_raw(0))
            .with_axis(KeskoAxis::X)
            .with_limits(Vec2::new(-1.0, 1.0));

        let generic: GenericJoint = joint.into();

        println!("{:?}", generic.limits);

        let limits = generic.limits(JointAxis::AngX).expect("No limits for AngX");
        assert_eq!(limits.min, limit_min);
        assert_eq!(limits.max, limit_max);
    }

    #[test]
    fn default_values() {

        let joint = RevoluteJoint::attach_to(Entity::from_raw(0));

        let generic: GenericJoint = joint.into();

        assert!(generic.limits(JointAxis::AngX).is_none());
        assert!(generic.limits(JointAxis::AngY).is_none());
        assert!(generic.limits(JointAxis::AngZ).is_none());
        assert!(generic.limits(JointAxis::X).is_none());
        assert!(generic.limits(JointAxis::Y).is_none());
        assert!(generic.limits(JointAxis::Z).is_none());
    }
}
