use bevy::prelude::*;
use rapier3d::prelude::{MultibodyJointSet, RigidBodySet};

use crate::{joint::MultibodyJointHandle, rigid_body::RigidBodyHandle};


/// Component for storing the mass of a rigid body
#[derive(Component)]
pub struct Mass {
    pub val: f32
}

/// Compnent that stores the mass of the multibody that a rigid body is part of
#[derive(Component)]
pub struct MultibodyMass {
    pub val: f32
}

/// System for adding multi body mass to rigid bodies that are part of a multibody
pub(crate) fn update_multibody_mass_system(
    mut commands: Commands,
    multibody_set: Res<MultibodyJointSet>,
    bodies: Res<RigidBodySet>,
    query: Query<(Entity, &RigidBodyHandle), (With<MultibodyJointHandle>, Without<MultibodyMass>)>
) {

    // TODO: This could be improved since we will calculate the multibody mass for all the bodies in the multibody,
    // but it is not crucial since it is a one timer. Could be when scaling up the amount of bodies
    for (entity, handle) in query.iter() {

        let mut multibody_mass = 0.0;
        for attached_handle in multibody_set.attached_bodies(handle.0) { 
            if let Some(body) = bodies.get(attached_handle) {
                multibody_mass += body.mass();
            }
        }

        commands.entity(entity).insert(MultibodyMass { val: multibody_mass });
    }
}