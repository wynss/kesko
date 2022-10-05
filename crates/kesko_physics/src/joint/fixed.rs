use bevy::prelude::*;
use rapier3d::prelude::{
    GenericJoint, 
    FixedJointBuilder
};

use crate::conversions::{IntoRapier, IntoBevy};
use super::{JointTrait, Axis};


pub struct FixedJoint {
    pub parent_anchor: Transform,
    pub child_anchor: Transform,
}

impl JointTrait for FixedJoint {
    fn parent_anchor(&self) -> Transform {
        self.parent_anchor
    }

    fn child_anchor(&self) -> Transform {
        self.child_anchor
    }

    fn get_axis(&self) -> Option<Axis> {
        None
    }
}

impl From<FixedJoint> for GenericJoint {
    fn from(joint: FixedJoint) -> Self {
        FixedJointBuilder::new()
            .local_frame1(joint.parent_anchor.into_rapier())
            .local_frame2(joint.child_anchor.into_rapier())
            .into()
    }
}

impl From<GenericJoint> for FixedJoint {
    fn from(joint: GenericJoint) -> Self {
        let (parent_translation, parent_rot) = joint.local_frame1.into_bevy();
        let parent_anchor = Transform::from_translation(parent_translation).with_rotation(parent_rot);
        let (child_translation, child_rot) = joint.local_frame2.into_bevy();
        let child_anchor = Transform::from_translation(child_translation).with_rotation(child_rot);
        FixedJoint { parent_anchor, child_anchor }
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