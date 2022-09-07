use bevy::prelude::Transform;


pub fn get_world_transform(
    origin: &Transform, 
    parent_anchor: &Transform, 
    child_anchor: &Transform
) -> Transform {
    let translation = origin.translation + parent_anchor.translation - child_anchor.rotation.inverse().mul_vec3(child_anchor.translation);
    Transform::from_translation(translation).with_rotation(child_anchor.rotation.inverse())
}