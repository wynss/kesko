use bevy::prelude::*;
use crate::rapier_extern::rapier::prelude as rapier;
use crate::rigid_body::RigidBodyHandle;


pub struct Gravity(Vec3);

impl Gravity {
    pub fn new(vec: Vec3) -> Self {
        Self(vec)
    }
    pub fn get(&self) -> &Vec3 {
        &self.0
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}

#[derive(Component)]
pub struct GravityScale {
    pub val: rapier::Real,
    pub initial_val: rapier::Real
}

impl Default for GravityScale {
    fn default() -> Self {
        GravityScale { val: 1.0 , initial_val: 1.0}
    }
}

impl GravityScale {
    pub fn new(val: rapier::Real) -> Self {
        Self { val, initial_val: val }
    }

    pub fn reset(&mut self) {
        self.val = self.initial_val;
    }
}

pub(crate) fn update_gravity_scale_system(
    mut rigid_bodies: ResMut<rapier::RigidBodySet>,
    query: Query<(&RigidBodyHandle, &GravityScale), Changed<GravityScale>>
) {
    for (body_handle, gravity_scale) in query.iter() {
        if let Some(body) = rigid_bodies.get_mut(body_handle.0) {
            body.set_gravity_scale(gravity_scale.val, true);
        }
    }
}