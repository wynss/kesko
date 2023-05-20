use bevy::prelude::*;

use kesko_types::resource::KeskoRes;

use crate::rapier_extern::rapier::prelude as rapier;
use crate::{conversions::IntoRapier, rigid_body::RigidBodyHandle};

/// Component to apply force to a rigid body
#[derive(Component)]
pub struct Force {
    /// Force vector
    pub vec: Vec3,
    /// If the forces should be reset before applying force
    pub reset_forces: bool,
}

impl Force {
    pub fn reset(&mut self) {
        self.vec = Vec3::ZERO;
        self.reset_forces = true;
    }
}

impl Default for Force {
    fn default() -> Self {
        Self {
            vec: Vec3::ZERO,
            reset_forces: true,
        }
    }
}

/// System to apply a force to a rigid body
#[allow(clippy::type_complexity)]
pub(crate) fn update_force_system(
    mut rigid_bodies: ResMut<KeskoRes<rapier::RigidBodySet>>,
    changed_force: Query<(&RigidBodyHandle, &Force), (Changed<Force>, With<RigidBodyHandle>)>,
) {
    for (body_handle, force) in changed_force.iter() {
        if let Some(body) = rigid_bodies.get_mut(body_handle.0) {
            if force.reset_forces {
                body.reset_forces(true);
            }
            body.add_force(force.vec.into_rapier(), true);
        }
    }
}
