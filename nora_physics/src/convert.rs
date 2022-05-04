use bevy::math::{
    DVec3

};
use nalgebra::Vector3;


pub trait IntoNalgebra<T> {
    fn into_nalgebra(self) -> T;
}

pub trait IntoGlam<T> {
    fn into_glam(self) -> T;
}


impl IntoGlam<DVec3> for Vector3<f64> {
    fn into_glam(self) -> DVec3 {
        DVec3::new(self.x, self.y, self.z)
    }
}

impl IntoNalgebra<Vector3<f64>> for DVec3 {
    fn into_nalgebra(self) -> Vector3<f64> {
        Vector3::new(self.x, self.y, self.z)
    }
}
