use bevy::math::DVec3;

 pub struct Gravity(DVec3);

impl Gravity {
    pub fn new(vec: DVec3) -> Self {
        Self(vec)
    }

    pub fn get(&self) -> &DVec3 {
        &self.0
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Self(DVec3::ZERO)
    }
}