use std::collections::HashMap;

use bevy::prelude::*;
use rapier3d::prelude as rapier;

use crate::rigid_body::{RigidBodyHandle, BodyHandle2Entity, RigidBodyName};


/// Component to indicate that the entity is a multibody root entity
#[derive(Component)]
pub struct MultibodyRoot {
    // Name of the multibody
    pub name: String,
    /// map from rigidbody name to entity that has a reference to a joint
    pub joints: HashMap<String, Entity>
}

/// Component to indicate that the entity is a multibody child entity
#[derive(Component)]
pub struct MultiBodyChild {
    pub name: String,
    pub root: Entity,
    pub joints: HashMap<String, Entity>
}

pub struct MultibodyNameRegistry {
    names: HashMap<String, i32>
}

impl MultibodyNameRegistry {
    pub fn new() -> Self {
        Self {
            names: HashMap::<String, i32>::new()
        }
    }

    pub fn create_unique_name(&mut self, name: &String) -> String {

        if self.names.contains_key(name) {
            *self.names.get_mut(name).expect("") += 1;
        } else {
            self.names.insert(name.clone(), 0);
        }
        return format!("{}-{}", name, self.names.get(name).unwrap());
    }
}

/// System that adds components related to multibodies
/// 
/// Todo: Look how to improve this, now it feels a bit complicated and not clear.
#[allow(clippy::type_complexity)]
pub(crate) fn add_multibody_components_system(
    mut commands: Commands,
    mut name_registry: ResMut<MultibodyNameRegistry>,
    multibody_joint_set: Res<rapier::MultibodyJointSet>,
    body_2_entity: Res<BodyHandle2Entity>,
    query: Query<
        (Entity, &RigidBodyHandle, Option<&RigidBodyName>), 
        (With<RigidBodyHandle>, Without<MultiBodyChild>, Without<MultibodyRoot>)
    >
) {

    for (entity, handle, body_name) in query.iter() {

        // get the multibodyjoint link for the rigidbody handle.
        if let Some(body_joint_link) = multibody_joint_set.rigid_body_link(handle.0) {

            // get the multibody of the joint link
            if let Some(multibody) = multibody_joint_set.get_multibody(body_joint_link.multibody) {
                
                // build a hash map with names and entity to store in the component.
                // the component can later be used to easy get hold of different links and joints
                let mut joints = HashMap::<String, Entity>::new();
                for link in multibody.links() {

                    let link_entity = body_2_entity.get(&link.rigid_body_handle()).expect("body should be in body to entity map");

                    match query.get(*link_entity) {
                        Ok((_, _, name)) => {
                            if let Some(name) = name {
                                joints.insert(name.0.clone(), *link_entity);
                            } else {
                                joints.insert(link_entity.id().to_string(), *link_entity);
                            }
                        },
                        Err(e) => error!("{e}")
                    }
                }

                let mut name = if let Some(body_name) = body_name {
                    body_name.0.clone()
                } else {
                    // if we don't have a name use the entity id
                    entity.id().to_string()
                };

                if handle.0 == multibody.root().rigid_body_handle() {
                    // We have a root
                    // make the name unique
                    name = name_registry.create_unique_name(&name);
                    commands.entity(entity).insert(MultibodyRoot { name, joints });
                } else {
                    // not a root
                    let root_rigid_body_handle = multibody.root().rigid_body_handle();
                    let root_entity = body_2_entity.get(&root_rigid_body_handle).expect("Every rigid body should be connected to an entity");
                    commands.entity(entity).insert(MultiBodyChild { 
                        name,
                        root: *root_entity, 
                        joints
                    });
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {

}