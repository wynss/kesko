use bevy::prelude::*;

use kesko_types::resource::KeskoRes;

use crate::conversions::IntoRapier;
use crate::rapier_extern::rapier::prelude as rapier;
use crate::rigid_body::RigidBodyHandle;

#[derive(Component, Default)]
pub struct Impulse {
    pub vec: Vec3,
}

pub(crate) fn update_impulse(
    mut rigid_bodies: ResMut<KeskoRes<rapier::RigidBodySet>>,
    query: Query<(&RigidBodyHandle, &Impulse), Changed<Impulse>>,
) {
    for (body_handle, impulse) in query.iter() {
        if let Some(body) = rigid_bodies.get_mut(body_handle.0) {
            body.apply_impulse(impulse.vec.into_rapier(), true);
        }
    }
}
