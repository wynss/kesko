use bevy::math::Vec3;

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