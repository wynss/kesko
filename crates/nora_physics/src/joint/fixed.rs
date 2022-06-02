use bevy::prelude::*;
use rapier3d::prelude::{GenericJoint, FixedJointBuilder};
use crate::conversions::IntoRapier;


pub struct FixedJoint {
    pub parent_anchor: Transform,
    pub child_anchor: Transform,
}

impl From<FixedJoint> for GenericJoint {
    fn from(joint: FixedJoint) -> Self {
        FixedJointBuilder::new()
            .local_frame1(joint.parent_anchor.into_rapier())
            .local_frame2(joint.child_anchor.into_rapier())
            .into()
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{Transform, Vec3};
    use rapier3d::prelude::GenericJoint;
    use rapier3d::dynamics::JointAxis;
    use crate::IntoRapier;
    use super::FixedJoint;

    #[test]
    fn convert() {

        let expected_parent_transform = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let expected_child_transform = Transform::from_translation(Vec3::new(4.0, 5.0, 6.0));
        let fixed_joint = FixedJoint {
            parent_anchor: expected_parent_transform,
            child_anchor: expected_child_transform
        };

        let generic: GenericJoint = fixed_joint.into();

        assert_eq!(generic.local_frame1, expected_parent_transform.into_rapier());
        assert_eq!(generic.local_frame2, expected_child_transform.into_rapier());

        assert!(generic.limits(JointAxis::AngX).is_none());
        assert!(generic.limits(JointAxis::AngY).is_none());
        assert!(generic.limits(JointAxis::AngZ).is_none());
        assert!(generic.limits(JointAxis::X).is_none());
        assert!(generic.limits(JointAxis::Y).is_none());
        assert!(generic.limits(JointAxis::Z).is_none());
    }
}