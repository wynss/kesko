use bevy::prelude::Transform;


/// get the transformation of a multibody joint link in world coordinates
pub fn world_transform_from_joint_anchors(
    origin: &Transform, 
    parent_anchor: &Transform, 
    child_anchor: &Transform
) -> Transform {

    let translation = 
        // normal translation
        origin.translation + 
        // parent translation taking rotation into account in the origins coordinate system
        origin.rotation.mul_vec3(parent_anchor.translation) - 
        // child translation taking rotation into account in the origins coordinate system
        parent_anchor.rotation.mul_vec3(origin.rotation.mul_vec3(child_anchor.rotation.mul_vec3(child_anchor.translation)));

    Transform::from_translation(translation).with_rotation(origin.rotation * parent_anchor.rotation * child_anchor.rotation)
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_PI_2, PI};

    use bevy::prelude::*;
    use super::world_transform_from_joint_anchors;

    #[test]
    fn no_transform() {
        let origin = Transform::default();
        let parent = Transform::default();
        let child = Transform::default();

        let res = world_transform_from_joint_anchors(&origin, &parent, &child);

        assert_eq!(res, Transform::default());
    }

    #[test]
    fn only_translate() {
        let origin = Transform::default().with_translation(Vec3::new(1.0, 0.0, 0.0));
        let parent = Transform::default().with_translation(Vec3::new(0.0, 1.0, 0.0));
        let child = Transform::default().with_translation(Vec3::new(0.0, 1.0, 0.0));

        let expected = Transform::from_translation(Vec3::new(1.0, 0.0, 0.0));
        let result = world_transform_from_joint_anchors(&origin, &parent, &child);

        assert_almost_eq(&result, &expected);

        let origin = Transform::default().with_translation(Vec3::new(1.0, 0.0, 0.0));
        let parent = Transform::default().with_translation(Vec3::new(0.0, 0.0, 0.0));
        let child = Transform::default().with_translation(Vec3::new(0.0, 1.0, 0.0));

        let expected = Transform::from_translation(Vec3::new(1.0, -1.0, 0.0));
        let result = world_transform_from_joint_anchors(&origin, &parent, &child);

        assert_almost_eq(&result, &expected);
    }

    #[test]
    fn translation_and_rotation() {

        let origin = Transform::default().with_rotation(Quat::from_rotation_x(FRAC_PI_2));
        let parent = Transform::default().with_translation(Vec3::new(0.0, 1.0, 0.0));
        let child = Transform::default().with_translation(Vec3::new(0.0, 0.0, 0.0));

        let expected = Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)).with_rotation(Quat::from_rotation_x(FRAC_PI_2));
        let result = world_transform_from_joint_anchors(&origin, &parent, &child);

        assert_almost_eq(&result, &expected);

        let origin = Transform::default();
        let parent = Transform::default().with_translation(Vec3::new(0.0, 1.0, 0.0)).with_rotation(Quat::from_rotation_x(-FRAC_PI_2));
        let child = Transform::default().with_translation(Vec3::new(0.0, 0.0, 0.0));

        let expected = Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)).with_rotation(Quat::from_rotation_x(-FRAC_PI_2));
        let result = world_transform_from_joint_anchors(&origin, &parent, &child);

        assert_almost_eq(&result, &expected);

        let origin = Transform::default().with_rotation(Quat::from_rotation_x(FRAC_PI_2));
        let parent = Transform::default().with_translation(Vec3::new(0.0, 1.0, 0.0)).with_rotation(Quat::from_rotation_x(FRAC_PI_2));
        let child = Transform::default().with_translation(Vec3::new(-1.0, 0.0, 0.0));

        let expected = Transform::from_translation(Vec3::new(1.0, 0.0, 1.0)).with_rotation(Quat::from_rotation_x(PI));
        let result = world_transform_from_joint_anchors(&origin, &parent, &child);

        assert_almost_eq(&result, &expected);

        let origin = Transform::default().with_rotation(Quat::from_rotation_x(FRAC_PI_2));
        let parent = Transform::default().with_translation(Vec3::new(0.0, 1.0, 0.0));
        let child = Transform::default().with_translation(Vec3::new(-1.0, 0.0, 0.0)).with_rotation(Quat::from_rotation_x(FRAC_PI_2));

        let expected = Transform::from_translation(Vec3::new(1.0, 0.0, 1.0)).with_rotation(Quat::from_rotation_x(PI));
        let result = world_transform_from_joint_anchors(&origin, &parent, &child);

        assert_almost_eq(&result, &expected);
    }

    fn assert_almost_eq(result: &Transform, expected: &Transform) {

        // check translation
        assert!(
            (result.translation.x - expected.translation.x).abs() < f32::EPSILON, 
            "X coordinate was wrong, expected {} was {}", expected.translation.x, result.translation.x
        );
        assert!(
            (result.translation.y - expected.translation.y).abs() < f32::EPSILON, 
            "Y coordinate was wrong, expected {} was {}", expected.translation.y, result.translation.y
        );
        assert!(
            (result.translation.z - expected.translation.z).abs() < f32::EPSILON, 
            "Z coordinate was wrong, expected {} was {}", expected.translation.z, result.translation.z
        );

        // check rotation
        assert!(
            (result.rotation.x - expected.rotation.x).abs() < f32::EPSILON, 
            "X coordinate was wrong, expected {} was {}", expected.rotation.x, result.rotation.x
        );
        assert!(
            (result.rotation.y - expected.rotation.y).abs() < f32::EPSILON, 
            "Y rotation coordinate was wrong, expected {} was {}", expected.rotation.y, result.rotation.y
        );
        assert!(
            (result.rotation.z - expected.rotation.z).abs() < f32::EPSILON, 
            "Z rotation coordinate was wrong, expected {} was {}", expected.rotation.z, result.translation.z
        );
        assert!(
            (result.rotation.w - expected.rotation.w).abs() < f32::EPSILON, 
            "W rotation coordinate was wrong, expected {} was {}", expected.rotation.w, result.rotation.w
        );
    }
}
