pub mod components;
mod convert;
pub mod gravity;

use bevy::prelude::*;
use bevy::math::Vec3;

use rapier3d::prelude as rapier;
use fnv::FnvHashMap;
use rapier3d::prelude::IslandManager;

use convert::{IntoRapier, IntoBevy};
use components::{RigidBodyComp, RigidBodyHandleComp};
use gravity::Gravity;


type EntityBodyHandleMap = FnvHashMap<Entity, rapier::RigidBodyHandle>;

enum PhysicState {
    Run,
    Pause
}

pub struct PhysicsPlugin {
    pub gravity: Vec3
}

impl Default for PhysicsPlugin {
    fn default() -> Self {
        Self {
            gravity: Vec3::ZERO
        }
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<EntityBodyHandleMap>()
            .init_resource::<rapier::PhysicsPipeline>()         // Runs the complete simulation
            .init_resource::<rapier::RigidBodySet>()            // Holds all the rigid bodies
            .init_resource::<rapier::ColliderSet>()             // Holds all the colliders
            .init_resource::<rapier::IntegrationParameters>()   // sets the parameters that controls the simulation
            .init_resource::<rapier::IslandManager>()           // Keeps track of which dynamic rigid bodies that are moving and which are not
            .init_resource::<rapier::BroadPhase>()              // Detects pairs of colliders that are potentially in contact
            .init_resource::<rapier::NarrowPhase>()             // Calculates contact points of colliders and generate collision events
            .init_resource::<rapier::ImpulseJointSet>()
            .init_resource::<rapier::MultibodyJointSet>()
            .init_resource::<rapier::CCDSolver>()
            .insert_resource(Gravity::new(self.gravity))
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .label("physics-systems")
                    .with_system(add_rigid_bodies.label("add-bodies"))
                    .with_system(physics_pipeline_step.label("step").after("add-bodies"))
                    .with_system(update_bevy_world.after("step"))
            );
    }
}

#[allow(clippy::too_many_arguments)]
fn physics_pipeline_step(
    mut pipeline: ResMut<rapier::PhysicsPipeline>,
    gravity: Res<Gravity>,
    integration_parameters: Res<rapier::IntegrationParameters>,
    mut island_manager: ResMut<rapier::IslandManager>,
    mut broad_phase: ResMut<rapier::BroadPhase>,
    mut narrow_phase: ResMut<rapier::NarrowPhase>,
    mut rigid_bodies: ResMut<rapier::RigidBodySet>,
    mut colliders: ResMut<rapier::ColliderSet>,
    mut impulse_joints: ResMut<rapier::ImpulseJointSet>,
    mut multibody_joints: ResMut<rapier::MultibodyJointSet>,
    mut ccd_solver: ResMut<rapier::CCDSolver>,
) {

    let gravity = gravity.get().into_rapier();

    pipeline.step(
        &gravity,
        &integration_parameters,
        &mut island_manager,
        &mut broad_phase,
        &mut narrow_phase,
        &mut rigid_bodies,
        &mut colliders,
        &mut impulse_joints,
        &mut multibody_joints,
        &mut ccd_solver,
        &(),
        &()
    );
}

fn add_rigid_bodies(
    mut rigid_body_set: ResMut<rapier::RigidBodySet>,
    mut entity_body_map: ResMut<EntityBodyHandleMap>,
    mut commands: Commands,
    query: Query<(Entity, &RigidBodyComp, &Transform), Without<RigidBodyHandleComp>>
) {

    for (entity, rigid_body_comp, transform) in query.iter() {

        let rigid_body = match rigid_body_comp {
            RigidBodyComp::Fixed => {
                rapier::RigidBodyBuilder::fixed()
                    .translation(transform.translation.into_rapier())
                    .build()

            }
            RigidBodyComp::Dynamic => {
                rapier::RigidBodyBuilder::dynamic()
                    .translation(transform.translation.into_rapier())
                    .build()
            }
        };

        let rigid_body_handle = rigid_body_set.insert(rigid_body);
        entity_body_map.insert(entity, rigid_body_handle);

        // Add the rigid body component to the entity
        commands.entity(entity).insert(RigidBodyHandleComp(rigid_body_handle));
    }
}

fn update_bevy_world(
    rigid_bodies: Res<rapier::RigidBodySet>,
    mut query: Query<(&RigidBodyHandleComp, &mut Transform)>
) {
    for (rigid_body_handle, mut transform) in query.iter_mut() {
        let handle = rigid_body_handle.0;
        let rigid_body = rigid_bodies.get(handle).unwrap();
        transform.translation = rigid_body.position().translation.into_bevy();
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_gravity() {

        let gravity = Gravity::default();
        assert_eq!(*gravity.get(), Vec3::ZERO, "test default value");
        
        let expected = Vec3::new(1.0, 2.0, 3.0);
        let gravity = Gravity::new(expected);
        assert_eq!(*gravity.get(), expected, "test default value");
    }
}
