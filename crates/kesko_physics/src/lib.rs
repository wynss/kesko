pub mod rigid_body;
pub mod collider;
pub mod gravity;
pub mod force;
pub mod impulse;
pub mod mass;
pub mod joint;
pub mod event;
pub mod multibody;
mod conversions;
pub mod rapier_extern;

use bevy::prelude::*;
use bevy::math::Vec3;
use iyes_loopless::prelude::*;
use self::rapier_extern::rapier::prelude as rapier;

use conversions::{IntoRapier, IntoBevy};
use gravity::Gravity;


/// State to control the physics system
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PhysicState {
    Running,
    Stopped
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, StageLabel)]
struct PhysicsStage;

#[derive(Debug, PartialEq, Eq, Clone, Hash, StageLabel)]
pub enum PhysicStage {

    AddRigidBodies,
    AddJoints,
    AddColliders,
    AddMultibodies,

    PipelineStep,

    PostPipeline

}

pub struct PhysicsPlugin {
    pub gravity: Vec3,
    pub initial_state: PhysicState
}

impl PhysicsPlugin {
    pub fn gravity() -> Self {
        Self {
            gravity: Vec3::new(0.0, -9.81, 0.0),
            initial_state: PhysicState::Running,
        }
    }
}

impl Default for PhysicsPlugin {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0.0, -9.81, 0.0),
            initial_state: PhysicState::Running
        }
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<rigid_body::Entity2BodyHandle>()
            .init_resource::<rigid_body::BodyHandle2Entity>()
            .init_resource::<joint::Entity2JointHandle>()
            .init_resource::<collider::Entity2ColliderHandle>()
            .init_resource::<rapier::PhysicsPipeline>()         // Runs the complete simulation
            .init_resource::<rapier::RigidBodySet>()            // Holds all the rigid bodies
            .init_resource::<rapier::ColliderSet>()             // Holds all the colliders
            .insert_resource(rapier::IntegrationParameters {            // sets the parameters that controls the simulation
                // setting this above 0.8 can cause instabilities, 
                // if needed f64 feature should be used
                erp: 0.8,
                ..default()
            })   
            .init_resource::<rapier::IslandManager>()           // Keeps track of which dynamic rigid bodies that are moving and which are not
            .init_resource::<rapier::BroadPhase>()              // Detects pairs of colliders that are potentially in contact
            .init_resource::<rapier::NarrowPhase>()             // Calculates contact points of colliders and generate collision events
            .init_resource::<rapier::ImpulseJointSet>()
            .init_resource::<rapier::MultibodyJointSet>()
            .init_resource::<rapier::CCDSolver>()

            .insert_resource(event::collision::CollisionEventHandler::new())
            .add_event::<event::collision::CollisionEvent>()

            .insert_resource(Gravity::new(self.gravity))

            // state for controlling the physics
            .add_loopless_state(self.initial_state.clone())

            // Physics events
            .add_event::<event::PhysicRequestEvent>()
            .add_event::<event::PhysicResponseEvent>()
            .add_system(event::handle_events)

            // Multibody name registry, making sure all multibodies have unique names
            .insert_resource(multibody::MultibodyNameRegistry::new())

            .add_event::<joint::JointMotorEvent>()

            .add_stage_after(CoreStage::First, PhysicsStage, Schedule::default())
            .stage(PhysicsStage, |stage: &mut Schedule| {
                stage
                .add_stage(
                    PhysicStage::AddRigidBodies, 
                    SystemStage::single_threaded()
                        .with_system(rigid_body::add_rigid_bodies)
                )
                .add_stage(
                    PhysicStage::AddColliders, 
                    SystemStage::single_threaded()
                        .with_system(collider::add_colliders)
                )
                .add_stage(
                    PhysicStage::AddJoints, 
                    SystemStage::single_threaded()
                        .with_system(joint::add_multibody_joints)
                )
                .add_stage(
                    PhysicStage::AddMultibodies, 
                    SystemStage::single_threaded()
                    .with_system(multibody::add_multibodies)
                )
                .add_stage(
                    PhysicStage::PipelineStep, 
                    SystemStage::single_threaded().with_system(physics_pipeline_step.run_in_state(PhysicState::Running))
                )
                .add_stage(
                    "post-pipeline", 
                    SystemStage::parallel()
                    .with_system_set(
                        ConditionSet::new()
                            // .run_in_state(PhysicState::Running)
                            .with_system(update_bevy_world)
                            .with_system(multibody::update_multibody_vel_angvel)
                            .with_system(impulse::update_impulse)
                            .with_system(force::update_force_system)
                            .with_system(gravity::update_gravity_scale_system)
                            .with_system(mass::update_multibody_mass_system)
                            .with_system(joint::update_joint_motors_system)
                            .with_system(joint::update_joint_pos_system)
                            .with_system(event::collision::send_collision_events_system)
                            .with_system(event::spawn::send_spawned_events)
                            .into()
                    )
                )
            });
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
    collision_event_handler: Res<event::collision::CollisionEventHandler>
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
        &*collision_event_handler
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
