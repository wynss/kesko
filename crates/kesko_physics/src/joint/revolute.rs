use bevy::prelude::*;

use super::{AxisIntoVec, JointState, KeskoAxis};
use crate::conversions::IntoRapier;
use crate::rapier_extern::rapier::prelude as rapier;

#[derive(Component, Clone, Copy)]
pub struct RevoluteJoint {
    pub parent: Entity,
    pub parent_anchor: Transform,
    pub child_anchor: Transform,
    pub axis: KeskoAxis,
    pub limits: Option<Vec2>,
    pub damping: rapier::Real,
    pub stiffness: rapier::Real,
    pub max_motor_force: rapier::Real,

    rotation: rapier::Real,
    angvel: rapier::Real,
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
            max_motor_force: rapier::Real::MAX,
            rotation: 0.0,
            angvel: 0.0,
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

    pub fn update_rotation_angvel(&mut self, rot: Quat, dt: rapier::Real) {
        let prev_rot = self.rotation;
        // convert to local orientation by multiplying by the inverse of anchor's rotation
        let rotation =
            (self.parent_anchor.rotation.inverse() * self.child_anchor.rotation.inverse() * rot)
                .to_euler(EulerRot::XYZ);
        match self.axis {
            KeskoAxis::X => self.rotation = rotation.0 as rapier::Real,
            KeskoAxis::NegX => self.rotation = -rotation.0 as rapier::Real,
            KeskoAxis::Y => self.rotation = rotation.1 as rapier::Real,
            KeskoAxis::NegY => self.rotation = -rotation.1 as rapier::Real,
            KeskoAxis::Z => self.rotation = rotation.2 as rapier::Real,
            KeskoAxis::NegZ => self.rotation = -rotation.2 as rapier::Real,
            _ => error!("Revolute joint does not have a valid axis"),
        }

        self.angvel = (self.rotation - prev_rot) / dt;
    }

    pub fn rotation(&self) -> rapier::Real {
        self.rotation
    }

    pub fn state(&self) -> JointState {
        JointState::Revolute {
            axis: self.axis,
            angle: self.rotation,
            angular_velocity: self.angvel,
        }
    }
}

impl From<RevoluteJoint> for rapier::GenericJoint {
    fn from(joint: RevoluteJoint) -> rapier::GenericJoint {
        let mut builder = rapier::RevoluteJointBuilder::new(joint.axis.into_unitvec())
            .local_anchor1(joint.parent_anchor.translation.into_rapier())
            .local_anchor2(joint.child_anchor.translation.into_rapier());

        if joint.stiffness > 0.0 || joint.damping > 0.0 {
            builder = builder.motor(0.0, 0.0, joint.stiffness, joint.damping);
        }

        builder = builder.motor_max_force(joint.max_motor_force);

        if let Some(limits) = joint.limits {
            builder = builder.limits([limits.x as rapier::Real, limits.y as rapier::Real]);
        }

        let mut generic: rapier::GenericJoint = builder.into();
        generic.local_frame1.rotation =
            joint.parent_anchor.rotation.into_rapier() * generic.local_frame1.rotation;
        generic.local_frame1.rotation =
            joint.child_anchor.rotation.into_rapier() * generic.local_frame1.rotation;
        generic
    }
}

#[cfg(test)]
mod tests {
    use super::RevoluteJoint;
    use crate::rapier_extern::rapier::dynamics::JointAxis;
    use crate::rapier_extern::rapier::prelude::GenericJoint;
    use crate::{joint::KeskoAxis, IntoRapier};
    use bevy::prelude::{Entity, Transform, Vec2, Vec3};

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
        assert_eq!(
            generic.local_anchor1(),
            expected_parent_transform.translation.into_rapier()
        );
        assert_eq!(
            generic.local_anchor2(),
            expected_child_transform.translation.into_rapier()
        );
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
