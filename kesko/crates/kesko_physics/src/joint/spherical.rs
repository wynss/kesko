use crate::rapier_extern::rapier::prelude as rapier;
use bevy::prelude::*;

use crate::conversions::IntoRapier;

#[derive(Component, Clone, Copy)]
pub struct SphericalJoint {
    pub parent: Entity,
    pub parent_anchor: Transform,
    pub child_anchor: Transform,
    pub x_ang_limit: Option<Vec2>,
    pub y_ang_limit: Option<Vec2>,
    pub z_ang_limit: Option<Vec2>,
    pub x_stiffness: rapier::Real,
    pub x_damping: rapier::Real,
    pub y_stiffness: rapier::Real,
    pub y_damping: rapier::Real,
    pub z_stiffness: rapier::Real,
    pub z_damping: rapier::Real,
}

impl SphericalJoint {
    pub fn attach_to(parent: Entity) -> Self {
        Self {
            parent,
            parent_anchor: Transform::default(),
            child_anchor: Transform::default(),
            x_ang_limit: None,
            y_ang_limit: None,
            z_ang_limit: None,
            x_stiffness: 0.0,
            x_damping: 0.0,
            y_stiffness: 0.0,
            y_damping: 0.0,
            z_stiffness: 0.0,
            z_damping: 0.0,
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

    pub fn with_x_limits(mut self, limits: Vec2) -> Self {
        self.x_ang_limit = Some(limits);
        self
    }

    pub fn with_y_limits(mut self, limits: Vec2) -> Self {
        self.y_ang_limit = Some(limits);
        self
    }

    pub fn with_z_limits(mut self, limits: Vec2) -> Self {
        self.z_ang_limit = Some(limits);
        self
    }
}

impl From<SphericalJoint> for rapier::GenericJoint {
    fn from(joint: SphericalJoint) -> rapier::GenericJoint {
        let mut builder = rapier::SphericalJointBuilder::new();

        // set activate and set motor parameters
        if joint.x_stiffness > 0.0 || joint.x_damping > 0.0 {
            builder = builder.motor(
                rapier::JointAxis::AngX,
                0.0,
                0.0,
                joint.x_stiffness,
                joint.x_damping,
            );
        }
        if joint.y_stiffness > 0.0 || joint.y_damping > 0.0 {
            builder = builder.motor(
                rapier::JointAxis::AngY,
                0.0,
                0.0,
                joint.y_stiffness,
                joint.y_damping,
            );
        }
        if joint.z_stiffness > 0.0 || joint.z_damping > 0.0 {
            builder = builder.motor(
                rapier::JointAxis::AngZ,
                0.0,
                0.0,
                joint.z_stiffness,
                joint.z_damping,
            );
        }

        // set rotational limits if any
        if let Some(x_ang_limit) = joint.x_ang_limit {
            builder = builder.limits(
                rapier::JointAxis::AngX,
                [x_ang_limit.x as rapier::Real, x_ang_limit.y as rapier::Real],
            );
        }

        if let Some(y_ang_limit) = joint.y_ang_limit {
            builder = builder.limits(
                rapier::JointAxis::AngY,
                [y_ang_limit.x as rapier::Real, y_ang_limit.y as rapier::Real],
            );
        }

        if let Some(z_ang_limit) = joint.z_ang_limit {
            builder = builder.limits(
                rapier::JointAxis::AngZ,
                [z_ang_limit.x as rapier::Real, z_ang_limit.y as rapier::Real],
            );
        }

        let mut generic: rapier::GenericJoint = builder.into();
        *generic
            .set_local_frame1(joint.parent_anchor.into_rapier())
            .set_local_frame2(joint.child_anchor.into_rapier())
    }
}

impl From<rapier::GenericJoint> for SphericalJoint {
    fn from(_joint: rapier::GenericJoint) -> Self {
        todo!("Implement this when we need to convert back to the specific joint");
    }
}

#[cfg(test)]
mod tests {
    use super::SphericalJoint;
    use crate::rapier_extern::rapier::dynamics::JointAxis;
    use crate::rapier_extern::rapier::prelude::GenericJoint;
    use crate::IntoRapier;
    use bevy::math::Vec2;
    use bevy::prelude::{Entity, Transform, Vec3};

    #[test]
    fn only_translation() {
        let expected_parent_transform = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let expected_child_transform = Transform::from_translation(Vec3::new(4.0, 5.0, 6.0));

        let joint = SphericalJoint::attach_to(Entity::from_raw(0))
            .with_parent_anchor(expected_parent_transform)
            .with_child_anchor(expected_child_transform);

        let generic: GenericJoint = joint.into();

        assert!(generic.as_spherical().is_some());
        assert_eq!(
            generic.local_frame1,
            expected_parent_transform.into_rapier()
        );
        assert_eq!(generic.local_frame2, expected_child_transform.into_rapier());
    }

    #[test]
    fn with_limits() {
        let x_min = -1.0;
        let x_max = 1.0;

        let y_min = -2.0;
        let y_max = 2.0;

        let z_min = -3.0;
        let z_max = 3.0;

        let joint = SphericalJoint::attach_to(Entity::from_raw(0))
            .with_x_limits(Vec2::new(-1.0, 1.0))
            .with_y_limits(Vec2::new(-2.0, 2.0))
            .with_z_limits(Vec2::new(-3.0, 3.0));

        let generic: GenericJoint = joint.into();

        let x_ang_limits = generic.limits(JointAxis::AngX).expect("No limits for AngX");
        let y_ang_limits = generic.limits(JointAxis::AngY).expect("No limits for AngY");
        let z_ang_limits = generic.limits(JointAxis::AngZ).expect("No limits for AngZ");

        assert_eq!(x_ang_limits.min, x_min);
        assert_eq!(x_ang_limits.max, x_max);

        assert_eq!(y_ang_limits.min, y_min);
        assert_eq!(y_ang_limits.max, y_max);

        assert_eq!(z_ang_limits.min, z_min);
        assert_eq!(z_ang_limits.max, z_max);
    }

    #[test]
    fn default_values() {
        let joint = SphericalJoint::attach_to(Entity::from_raw(0));

        let generic: GenericJoint = joint.into();

        assert_eq!(generic.local_frame1, Transform::default().into_rapier());
        assert_eq!(generic.local_frame2, Transform::default().into_rapier());

        assert!(generic.limits(JointAxis::AngX).is_none());
        assert!(generic.limits(JointAxis::AngY).is_none());
        assert!(generic.limits(JointAxis::AngZ).is_none());
        assert!(generic.limits(JointAxis::X).is_none());
        assert!(generic.limits(JointAxis::Y).is_none());
        assert!(generic.limits(JointAxis::Z).is_none());
    }
}
