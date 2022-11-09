use std::collections::BTreeMap;

use crate::rapier_extern::rapier::prelude as rapier;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    conversions::IntoBevy,
    joint::JointState,
    rigid_body::{BodyHandle2Entity, Entity2BodyHandle, RigidBodyHandle},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct MultiBodyStates(pub Vec<MultiBodyState>);

/// Used for sending data outside of Kesko
#[derive(Serialize, Deserialize, Clone)]
pub struct MultiBodyState {
    pub name: String,
    pub id: u64,
    pub position: Vec3,
    pub orientation: Quat,
    pub velocity: Vec3,
    pub angular_velocity: Vec3,
    pub relative_positions: Option<BTreeMap<String, Vec3>>,
    pub joint_states: Option<BTreeMap<String, Option<JointState>>>,
}

/// Component to indicate that the entity is a multibody root entity
#[derive(Component)]
pub struct MultibodyRoot {
    // Name of the multibody
    pub name: String,
    /// Linear velocity of body
    pub linvel: Vec3,
    /// Angular velocity of body
    pub angvel: Vec3,
    /// Map from rigidbody name to entity that has a reference to a joint
    pub child_map: BTreeMap<String, Entity>,
}

/// Component to indicate that the entity is a multibody child entity
#[derive(Component)]
pub struct MultibodyChild {
    /// Name of the child link
    pub name: String,
    /// name of the root link
    pub root: Entity,
    /// Map from rigidbody name to entity that has a reference to a joint
    pub child_map: BTreeMap<String, Entity>,
}

/// System that adds components related to multibodies
///
/// Todo: Look how to improve this, now it feels a bit complicated and not clear.
#[allow(clippy::type_complexity)]
pub(crate) fn add_multibodies(
    mut commands: Commands,
    multibody_joint_set: Res<rapier::MultibodyJointSet>,
    body_2_entity: Res<BodyHandle2Entity>,
    rigid_body_query: Query<
        (Entity, &RigidBodyHandle, Option<&Name>),
        (
            With<RigidBodyHandle>,
            Without<MultibodyChild>,
            Without<MultibodyRoot>,
        ),
    >,
) {
    for (entity, handle, body_name) in rigid_body_query.iter() {
        // get the multibodyjoint link for the rigidbody handle.
        if let Some(body_joint_link) = multibody_joint_set.rigid_body_link(handle.0) {
            // get the multibody of the joint link
            if let Some(multibody) = multibody_joint_set.get_multibody(body_joint_link.multibody) {
                // build a hash map with names and entity to store in the component.
                // the component can later be used to easy get hold of different links and joints
                let mut joints = BTreeMap::<String, Entity>::new();

                for link in multibody.links() {
                    // don't add root link to root
                    if link.is_root()
                        & multibody
                            .link(body_joint_link.id)
                            .expect("link should exists")
                            .is_root()
                    {
                        continue;
                    }

                    let link_entity = body_2_entity
                        .get(&link.rigid_body_handle())
                        .expect("body should be in body to entity map");

                    match rigid_body_query.get(*link_entity) {
                        Ok((_, _, name)) => {
                            if let Some(name) = name {
                                joints.insert(name.to_string(), *link_entity);
                            } else {
                                joints.insert(link_entity.id().to_string(), *link_entity);
                            }
                        }
                        Err(e) => error!("{e}"),
                    }
                }

                let mut name = if let Some(body_name) = body_name {
                    body_name.to_string()
                } else {
                    // if we don't have a name use the entity id
                    entity.id().to_string()
                };

                if handle.0 == multibody.root().rigid_body_handle() {
                    // We have a root
                    // make the name unique by combining the name and entity id
                    name = format!("{}-{}", name, entity.id());
                    commands.entity(entity).insert(MultibodyRoot {
                        name,
                        linvel: Vec3::ZERO,
                        angvel: Vec3::ZERO,
                        child_map: joints,
                    });
                } else {
                    // not a root
                    let root_rigid_body_handle = multibody.root().rigid_body_handle();
                    let root_entity = body_2_entity
                        .get(&root_rigid_body_handle)
                        .expect("Every rigid body should be connected to an entity");
                    commands.entity(entity).insert(MultibodyChild {
                        name,
                        root: *root_entity,
                        child_map: joints,
                    });
                }
            }
        }
    }
}

pub(crate) fn update_multibody_vel_angvel(
    rigid_body_set: ResMut<rapier::RigidBodySet>,
    e2b: Res<Entity2BodyHandle>,
    mut multibodies: Query<(Entity, &mut MultibodyRoot)>,
) {
    for (e, mut multibody) in multibodies.iter_mut() {
        if let Some(handle) = e2b.get(&e) {
            if let Some(rigid_body) = rigid_body_set.get(*handle) {
                multibody.linvel = rigid_body.linvel().into_bevy();
                multibody.angvel = rigid_body.angvel().into_bevy();
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{joint, rigid_body};

    #[test]
    fn test_name() {
        let mut app = App::new();

        // add systems and resources for building a multibody
        app.add_stage("Test", Schedule::default())
            .stage("Test", |stage: &mut Schedule| {
                stage
                    .add_stage(
                        "Add bodies",
                        SystemStage::single_threaded().with_system(rigid_body::add_rigid_bodies),
                    )
                    .add_stage(
                        "Add joints",
                        SystemStage::single_threaded().with_system(joint::add_multibody_joints),
                    )
                    .add_stage(
                        "Add multi",
                        SystemStage::single_threaded().with_system(add_multibodies),
                    )
            });

        app.init_resource::<rapier::RigidBodySet>();
        app.init_resource::<rapier::MultibodyJointSet>();
        app.init_resource::<joint::Entity2JointHandle>();
        app.init_resource::<rigid_body::Entity2BodyHandle>();
        app.init_resource::<rigid_body::BodyHandle2Entity>();

        // add root
        let root_entity = app
            .world
            .spawn()
            .insert_bundle(TransformBundle::default())
            .insert(rigid_body::RigidBody::Dynamic)
            .insert(Name::new("Root".to_owned()))
            .id();

        // attach a child
        app.world
            .spawn()
            .insert_bundle(TransformBundle::default())
            .insert(rigid_body::RigidBody::Dynamic)
            .insert(joint::fixed::FixedJoint::attach_to(root_entity));

        app.update();

        // assert expected name
        let expected_name = format!("Root-{}", root_entity.id());
        let root_comp = app.world.get::<MultibodyRoot>(root_entity).unwrap();
        assert_eq!(*root_comp.name, expected_name)
    }
}
