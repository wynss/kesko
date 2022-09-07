use std::collections::HashMap;

use bevy::prelude::*;
use rapier3d::prelude as rapier;

use crate::rigid_body::{RigidBodyHandle, BodyHandle2Entity, RigidBodyName};


/// Component to indicate that the entity is a multibody root entity
#[derive(Component)]
pub struct MultibodyRoot {
    /// map from rigidbody name to entity
    pub joints: HashMap<String, Entity>
}

/// Component to indicate that the entity is a multibody
#[derive(Component)]
pub struct MultiBodyChild;

/// Component to indicate that the entity is not a multibody
#[derive(Component)]
pub struct NotMultibody;

/// System that adds components related to multibodies
/// 
/// Todo: Look how to improve this, now it feels a bit complicated and not clear.
#[allow(clippy::type_complexity)]
pub(crate) fn add_multibody_components_system(
    mut commands: Commands,
    multibody_joint_set: Res<rapier::MultibodyJointSet>,
    body_2_entity: Res<BodyHandle2Entity>,
    query: Query<
        (Entity, &RigidBodyHandle, Option<&RigidBodyName>), 
        (With<RigidBodyHandle>, Without<NotMultibody>, Without<MultibodyRoot>)
    >
) {

    for (entity, handle, _) in query.iter() {

        if let Some(body) = multibody_joint_set.rigid_body_link(handle.0) {
            if let Some(mb) = multibody_joint_set.get_multibody(body.multibody) {
                if handle.0 == mb.root().rigid_body_handle() {

                    // we have the root of the multibody -> create a map from name of the bodies
                    // in the multi body to the entity id
                    let mut joints = HashMap::<String, Entity>::new();
                    for link in mb.links() {

                        let link_entity = body_2_entity.get(&link.rigid_body_handle()).expect("body should be in body to entity map");
                        match query.get(*link_entity) {
                            Ok((_, _, name)) => {
                                if let Some(name) = name {
                                    joints.insert(name.0.clone(), *link_entity);
                                }
                            },
                            Err(e) => error!("{e}")
                        }
                    }
                    commands.entity(entity).insert(MultibodyRoot { joints });

                } else {
                    // not a root
                    commands.entity(entity).insert(MultiBodyChild);
                }
            }
        } else {
            // not a multibody
            commands.entity(entity).insert(NotMultibody);
        }
    }
}


#[cfg(test)]
mod tests {

}