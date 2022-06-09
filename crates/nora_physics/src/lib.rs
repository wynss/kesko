pub mod rigid_body;
pub mod collider;
pub mod gravity;
pub mod impulse;
pub mod mass;
pub mod joint;
mod conversions;

use bevy::prelude::*;
use bevy::math::Vec3;
use rapier3d::prelude as rapier;

use conversions::{IntoRapier, IntoBevy};
use gravity::Gravity;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub enum PhysicsSystem {
    AddRigidBodies,
    AddJoints,
    AddColliders,
    UpdateImpulse,
    UpdateGravityScale,
    PipelineStep,
    UpdateBevyWorld
}

pub struct PhysicsPlugin {
    pub gravity: Vec3
}

impl PhysicsPlugin {
    pub fn gravity() -> Self {
        Self {
            gravity: Vec3::new(0.0, -9.81, 0.0)
        }
    }
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
            .init_resource::<rigid_body::EntityBodyHandleMap>()
            .init_resource::<collider::EntityColliderMap>()
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
                    .with_system(
                        rigid_body::add_rigid_bodies_system
                        .label(PhysicsSystem::AddRigidBodies)
                    )
                    .with_system(
                        joint::add_joints_system
                            .label(PhysicsSystem::AddJoints)
                            .after(PhysicsSystem::AddRigidBodies)
                    )
                    .with_system(
                        collider::add_collider_to_bodies_system
                            .label(PhysicsSystem::AddColliders)
                            .after(PhysicsSystem::AddRigidBodies))
                    .with_system(
                        impulse::update_impulse
                            .label(PhysicsSystem::UpdateImpulse)
                            .after(PhysicsSystem::AddColliders)
                    )
                    .with_system(gravity::update_gravity_scale_system
                        .label(PhysicsSystem::UpdateGravityScale)
                        .after(PhysicsSystem::AddColliders)
                    )
                    .with_system(
                        physics_pipeline_step
                            .label(PhysicsSystem::PipelineStep)
                            .after(PhysicsSystem::UpdateImpulse)
                    )
                    .with_system(
                        update_bevy_world
                            .label(PhysicsSystem::UpdateBevyWorld)
                            .after(PhysicsSystem::PipelineStep)
                    )
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

fn update_bevy_world(
    rigid_bodies: Res<rapier::RigidBodySet>,
    mut query: Query<(&rigid_body::RigidBodyHandle, &mut Transform)>
) {
    for (rigid_body_handle, mut transform) in query.iter_mut() {

        let handle = rigid_body_handle.0;
        let rigid_body = rigid_bodies.get(handle).unwrap();

        let (translation, rotation) = rigid_body.position().into_bevy();

        transform.translation = translation;
        transform.rotation = rotation;
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
