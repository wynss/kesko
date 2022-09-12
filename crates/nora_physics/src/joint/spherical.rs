use std::any::Any;

use bevy::prelude::*;
use rapier3d::prelude::{GenericJoint, JointAxis, SphericalJointBuilder};

use crate::conversions::IntoRapier;
use super::AsAnyJoint;


#[derive(Default)]
pub struct SphericalJoint {
    pub parent_anchor: Transform,
    pub child_anchor: Transform,
    pub x_ang_limit: Option<Vec2>,
    pub y_ang_limit: Option<Vec2>,
    pub z_ang_limit: Option<Vec2>,
    pub x_stiffness: f32,
    pub x_damping: f32,
    pub y_stiffness: f32,
    pub y_damping: f32,
    pub z_stiffness: f32,
    pub z_damping: f32,
}

impl AsAnyJoint for SphericalJoint {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<SphericalJoint> for GenericJoint {
    fn from(joint: SphericalJoint) -> GenericJoint {

        let mut builder = SphericalJointBuilder::new();

        // set activate and set motor parameters
        if joint.x_stiffness > 0.0 || joint.x_damping > 0.0 {
            builder = builder.motor(JointAxis::AngX, 0.0, 0.0,  joint.x_stiffness, joint.x_damping);
        }
        if joint.y_stiffness > 0.0 || joint.y_damping > 0.0 {
            builder = builder.motor(JointAxis::AngY, 0.0, 0.0,  joint.y_stiffness, joint.y_damping);
        }
        if joint.z_stiffness > 0.0 || joint.z_damping > 0.0 {
            builder = builder.motor(JointAxis::AngZ, 0.0, 0.0,  joint.z_stiffness, joint.z_damping);
        }

        // set rotational limits if any
        if let Some(x_ang_limit) = joint.x_ang_limit {
            builder = builder.limits(JointAxis::AngX, x_ang_limit.into());
        }

        if let Some(y_ang_limit) = joint.y_ang_limit {
            builder = builder.limits(JointAxis::AngY, y_ang_limit.into());
        }

        if let Some(z_ang_limit) = joint.z_ang_limit {
            builder = builder.limits(JointAxis::AngZ, z_ang_limit.into());
        }

        let mut generic: GenericJoint = builder.into();
        *generic
            .set_local_frame1(joint.parent_anchor.into_rapier())
            .set_local_frame2(joint.child_anchor.into_rapier())
    }
}

impl From<GenericJoint> for SphericalJoint {
    fn from(joint: GenericJoint) -> Self {
        todo!("Implement this when we need to convert back to the specific joint");
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec2;
    use bevy::prelude::{Transform, Vec3};
    use rapier3d::dynamics::JointAxis;
    use rapier3d::prelude::GenericJoint;
    use crate::{default, IntoRapier};
    use super::SphericalJoint;

    #[test]
    fn only_translation() {

        let expected_parent_transform = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let expected_child_transform = Transform::from_translation(Vec3::new(4.0, 5.0, 6.0));

        let fixed_joint = SphericalJoint {
            parent_anchor: expected_parent_transform,
            child_anchor: expected_child_transform,
            ..default()
        };

        let generic: GenericJoint = fixed_joint.into();

        assert!(generic.as_spherical().is_some());
        assert_eq!(generic.local_frame1, expected_parent_transform.into_rapier());
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

        let fixed_joint = SphericalJoint {
            x_ang_limit: Some(Vec2::new(-1.0, 1.0)),
            y_ang_limit: Some(Vec2::new(-2.0, 2.0)),
            z_ang_limit: Some(Vec2::new(-3.0, 3.0)),
            ..default()
        };

        let generic: GenericJoint = fixed_joint.into();

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

        let joint = SphericalJoint::default();

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