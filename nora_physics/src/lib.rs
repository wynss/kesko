pub mod components;
mod convert;
pub mod gravity;

use bevy::prelude::*;
use bevy::math::DVec3;

use rapier3d_f64::prelude as rapier;
use fnv::FnvHashMap;

use convert::{IntoNalgebra};
use components::{RigidBodyComp, RigidBodyHandleComp};
use gravity::Gravity;


type EntityBodyHandleMap = FnvHashMap<Entity, rapier::RigidBodyHandle>;

enum PhysicState {
    Run,
    Pause
}

pub struct PhysicsPlugin {
    gravity: DVec3
}

impl Default for PhysicsPlugin {
    fn default() -> Self {
        Self {
            gravity: DVec3::ZERO
        }
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<EntityBodyHandleMap>()
        .init_resource::<rapier::RigidBodySet>()            // Holds all the rigid bodies
        .init_resource::<rapier::ColliderSet>()             // Holds all the colliders
        .init_resource::<rapier::PhysicsPipeline>()         // Runs the complete simulation
        .init_resource::<rapier::IntegrationParameters>()   // sets the parameters that controls the simulation
        .init_resource::<rapier::IslandManager>()           // Keeps track of which dynamic rigid bodies that are moving and which are not
        .init_resource::<rapier::BroadPhase>()              // Detects pairs of colliders that are potentially in contact 
        .init_resource::<rapier::NarrowPhase>()             // Calculates contact points of colliders and generate collision events
        .init_resource::<rapier::ImpulseJointSet>()
        .init_resource::<rapier::MultibodyJointSet>()
        .init_resource::<rapier::CCDSolver>()
        .insert_resource(Gravity::new(self.gravity))
        .add_system_to_stage(CoreStage::PostUpdate, physics_pipeline_step)
        .add_system_to_stage(CoreStage::PostUpdate, add_rigid_bodies);
    }
}


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

    let gravity = DVec3::from(*gravity.get()).into_nalgebra();

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
        
        let rigid_body_handle = match rigid_body_comp {
            RigidBodyComp::Fixed => {
                let rigid_body = rapier::RigidBodyBuilder::fixed().build();
                rigid_body_set.insert(rigid_body)

            }
            RigidBodyComp::Dynamic => {
                let rigid_body = rapier::RigidBodyBuilder::dynamic().build();
                rigid_body_set.insert(rigid_body)
            }
        };

        entity_body_map.insert(entity, rigid_body_handle);

        // Add the rigid body component to the entity
        commands.entity(entity).insert(RigidBodyHandleComp(rigid_body_handle));
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_gravity() {

        let gravity = Gravity::default();
        assert_eq!(*gravity.get(), DVec3::ZERO, "test default value");
        
        let expected = DVec3::new(1.0, 2.0, 3.0);
        let gravity = Gravity::new(expected);
        assert_eq!(*gravity.get(), expected, "test default value");
    }
}
