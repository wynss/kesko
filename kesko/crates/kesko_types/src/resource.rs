use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

/// Wrapper in order make any data structure a bevy resource.
#[derive(Resource, Default)]
pub struct KeskoRes<T>(pub T);

impl<T> Deref for KeskoRes<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for KeskoRes<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
