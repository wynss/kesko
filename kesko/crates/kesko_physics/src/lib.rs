pub mod collider;
mod conversions;
pub mod event;
pub mod force;
pub mod gravity;
pub mod impulse;
pub mod joint;
pub mod mass;
pub mod multibody;
pub mod rapier_extern;
pub mod rigid_body;

use bevy::math::Vec3;
use bevy::prelude::*;

use kesko_types::resource::KeskoRes;

use self::rapier_extern::rapier::prelude as rapier;
use conversions::{IntoBevy, IntoRapier};
use gravity::Gravity;

/// State to control the physics system
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PhysicState {
    #[default]
    Running,
    Stopped,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemSet)]
#[system_set(base)]
pub enum PhysicStage {
    AddRigidBodies,
    AddJoints,
    AddColliders,
    AddMultibodies,

    PipelineStep,

    PostPipeline,
}

pub struct PhysicsPlugin {
    pub gravity: Vec3,
    pub initial_state: PhysicState,
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
            initial_state: PhysicState::Running,
        }
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeskoRes<rigid_body::Entity2Body>>()
            .init_resource::<KeskoRes<rigid_body::Body2Entity>>()
            .init_resource::<KeskoRes<joint::Entity2JointHandle>>()
            .init_resource::<KeskoRes<collider::Entity2Collider>>()
            .init_resource::<KeskoRes<rapier::PhysicsPipeline>>() // Runs the complete simulation
            .init_resource::<KeskoRes<rapier::RigidBodySet>>() // Holds all the rigid bodies
            .init_resource::<KeskoRes<rapier::ColliderSet>>() // Holds all the colliders
            .insert_resource(KeskoRes(rapier::IntegrationParameters {
                // sets the parameters that controls the simulation
                // setting this above 0.8 can cause instabilities,
                // if needed f64 feature should be used
                erp: 0.75,
                ..default()
            }))
            .init_resource::<KeskoRes<rapier::IslandManager>>() // Keeps track of which dynamic rigid bodies that are moving and which are not
            .init_resource::<KeskoRes<rapier::BroadPhase>>() // Detects pairs of colliders that are potentially in contact
            .init_resource::<KeskoRes<rapier::NarrowPhase>>() // Calculates contact points of colliders and generate collision events
            .init_resource::<KeskoRes<rapier::ImpulseJointSet>>()
            .init_resource::<KeskoRes<rapier::MultibodyJointSet>>()
            .init_resource::<KeskoRes<rapier::CCDSolver>>()
            // collision event related
            .insert_resource(event::collision::CollisionEventHandler::new())
            .add_event::<event::collision::CollisionEvent>()
            // gravity
            .insert_resource(Gravity::new(self.gravity))
            // state for controlling the physics
            .add_state::<PhysicState>()
            // Physics events
            .add_event::<event::PhysicRequestEvent>()
            .add_event::<event::PhysicResponseEvent>()
            .add_system(event::handle_events)
            .add_event::<joint::JointMotorEvent>()
            // configure how the physics sets are run
            .configure_sets(
                (
                    CoreSet::UpdateFlush,
                    PhysicStage::AddRigidBodies,
                    PhysicStage::AddColliders,
                    PhysicStage::AddJoints,
                    PhysicStage::AddMultibodies,
                    PhysicStage::PipelineStep,
                    PhysicStage::PostPipeline,
                    CoreSet::PostUpdate,
                )
                    .chain(),
            )
            .add_system(rigid_body::add_rigid_bodies.in_base_set(PhysicStage::AddRigidBodies))
            .add_system(apply_system_buffers.in_base_set(PhysicStage::AddRigidBodies))
            .add_system(collider::add_colliders.in_base_set(PhysicStage::AddColliders))
            .add_system(apply_system_buffers.in_base_set(PhysicStage::AddColliders))
            .add_system(joint::add_multibody_joints.in_base_set(PhysicStage::AddJoints))
            .add_system(apply_system_buffers.in_base_set(PhysicStage::AddJoints))
            .add_system(multibody::add_multibodies.in_base_set(PhysicStage::AddMultibodies))
            .add_system(apply_system_buffers.in_base_set(PhysicStage::AddMultibodies))
            .add_system(
                physics_pipeline_step
                    .in_base_set(PhysicStage::PipelineStep)
                    .run_if(in_state(PhysicState::Running)),
            )
            .add_system(apply_system_buffers.in_base_set(PhysicStage::PipelineStep))
            .add_systems(
                (
                    update_bevy_world,
                    multibody::update_multibody_vel_angvel,
                    impulse::update_impulse,
                    force::update_force_system,
                    gravity::update_gravity_scale_system,
                    mass::update_multibody_mass_system,
                    joint::update_joint_motors_system,
                    joint::update_joint_pos_system,
                    event::collision::send_collision_events_system,
                    event::spawn::send_spawned_events,
                )
                    .in_base_set(PhysicStage::PostPipeline),
            )
            .add_system(apply_system_buffers.in_base_set(PhysicStage::PostPipeline));
    }
}

#[allow(clippy::too_many_arguments)]
fn physics_pipeline_step(
    mut pipeline: ResMut<KeskoRes<rapier::PhysicsPipeline>>,
    gravity: Res<Gravity>,
    integration_parameters: Res<KeskoRes<rapier::IntegrationParameters>>,
    mut island_manager: ResMut<KeskoRes<rapier::IslandManager>>,
    mut broad_phase: ResMut<KeskoRes<rapier::BroadPhase>>,
    mut narrow_phase: ResMut<KeskoRes<rapier::NarrowPhase>>,
    mut rigid_bodies: ResMut<KeskoRes<rapier::RigidBodySet>>,
    mut colliders: ResMut<KeskoRes<rapier::ColliderSet>>,
    mut impulse_joints: ResMut<KeskoRes<rapier::ImpulseJointSet>>,
    mut multibody_joints: ResMut<KeskoRes<rapier::MultibodyJointSet>>,
    mut ccd_solver: ResMut<KeskoRes<rapier::CCDSolver>>,
    collision_event_handler: Res<event::collision::CollisionEventHandler>,
) {
    let gravity = gravity.get().into_rapier();

    pipeline.0.step(
        &gravity,
        &integration_parameters,
        &mut island_manager.0,
        &mut broad_phase,
        &mut narrow_phase,
        &mut rigid_bodies,
        &mut colliders,
        &mut impulse_joints,
        &mut multibody_joints,
        &mut ccd_solver,
        &(),
        &*collision_event_handler,
    );
}

fn update_bevy_world(
    rigid_bodies: Res<KeskoRes<rapier::RigidBodySet>>,
    mut query: Query<(&rigid_body::RigidBodyHandle, &mut Transform)>,
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
