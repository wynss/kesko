use bevy::prelude::*;
use crate::rapier_extern::rapier::prelude as rapier;

use crate::rigid_body::RigidBodyHandle;


/// Component for storing the mass of a rigid body
#[derive(Component)]
pub struct Mass {
    pub val: rapier::Real
}

/// Component that stores the mass of the multibody that a rigid body is part of
/// Will be same as Mass of the body is not part of a multibody
#[derive(Component)]
pub struct MultibodyMass {
    pub val: rapier::Real
}

/// System for adding multi body mass to rigid bodies. If the body is not part of a multibody the mass will be equal
/// to the body mass
pub(crate) fn update_multibody_mass_system(
    mut commands: Commands,
    multibody_set: Res<rapier::MultibodyJointSet>,
    bodies: Res<rapier::RigidBodySet>,
    query: Query<(Entity, &RigidBodyHandle), Without<MultibodyMass>>
) {
    for (entity, handle) in query.iter() {
        let mass = mass_of_attached(&handle.0, &multibody_set, &bodies, None);
        commands.entity(entity).insert(MultibodyMass { val: mass });
    }
}

/// function that calculates the total mass of a multibody recursively.
/// 
/// TODO: This could be improved since we will calculate the multibody mass for all the bodies in the multibody,
/// but it is not crucial since it is a one timer. Could be when scaling up the amount of bodies
fn mass_of_attached(handle: &rapier::RigidBodyHandle, multibody_set: &rapier::MultibodyJointSet, bodies: &rapier::RigidBodySet, parent_handle: Option<&rapier::RigidBodyHandle>) -> f64 {

    let body = bodies.get(*handle).unwrap();
    let mut mass = body.mass();

    for attached_handle in multibody_set.attached_bodies(*handle) {

        if let Some(handle) = parent_handle {
            if *handle == attached_handle {
                continue;
            }
        }
        mass += mass_of_attached(&attached_handle, multibody_set, bodies, Some(handle));
    }

    mass
}