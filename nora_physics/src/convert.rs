use bevy::math::{Vec3, Quat};
use rapier3d::math::{Translation, Vector, Isometry};
use nalgebra::Vector3;


pub trait IntoRapier<T> {
    #[must_use]
    fn into_rapier(self) -> T;
}

pub trait IntoBevy<T> {
    #[must_use]
    fn into_bevy(self) -> T;
}


impl IntoBevy<Vec3> for Vector3<f32> {
    fn into_bevy(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl IntoRapier<Vector3<f32>> for Vec3 {
    fn into_rapier(self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl IntoBevy<Vec3> for Translation<f32> {
    fn into_bevy(self) -> Vec3 {
       self.vector.into_bevy()
    }
}