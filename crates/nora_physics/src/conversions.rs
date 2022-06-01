use bevy::math::{Vec3, Quat};
use bevy::prelude::Transform;
use rapier3d::math::{Translation, Isometry, Vector};
use nalgebra::{UnitQuaternion, Vector3, Quaternion, Point3, UnitVector3};


pub trait IntoBevy<T> {
    #[must_use]
    fn into_bevy(self) -> T;
}

impl IntoBevy<Vec3> for Vector3<f32> {
    fn into_bevy(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}


impl IntoBevy<Vec3> for Translation<f32> {
    fn into_bevy(self) -> Vec3 {
       self.vector.into_bevy()
    }
}

impl IntoBevy<Quat> for UnitQuaternion<f32> {
    fn into_bevy(self) -> Quat {
        Quat::from_xyzw(self.i, self.j, self.k, self.w)
    }
}

impl IntoBevy<(Vec3, Quat)> for Isometry<f32> {
    fn into_bevy(self) -> (Vec3, Quat) {
        (self.translation.into_bevy(), self.rotation.into_bevy())
    }
}

pub trait IntoRapier<T> {
    #[must_use]
    fn into_rapier(self) -> T;
}

impl IntoRapier<Vector3<f32>> for Vec3 {
    fn into_rapier(self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl IntoRapier<Point3<f32>> for Vec3 {
    fn into_rapier(self) -> Point3<f32> {
        Point3::from([self.x, self.y, self.z])
    }
}

impl IntoRapier<UnitVector3<f32>> for Vec3 {
    fn into_rapier(self) -> UnitVector3<f32> {
        UnitVector3::new_normalize(self.into_rapier())
    }
}

impl IntoRapier<Translation<f32>> for Vec3 {
    fn into_rapier(self) -> Translation<f32> {
        <Vec3 as IntoRapier<Vector<f32>>>::into_rapier(self).into()
    }
}

impl IntoRapier<UnitQuaternion<f32>> for Quat {
    fn into_rapier(self) -> UnitQuaternion<f32> {
        UnitQuaternion::new_normalize(Quaternion::new(self.w, self.x, self.y, self.z))
    }
}

impl IntoRapier<Isometry<f32>> for (Vec3, Quat) {
    fn into_rapier(self) -> Isometry<f32> {
        Isometry::from_parts(self.0.into_rapier(), self.1.into_rapier())
    }
}

impl IntoRapier<Isometry<f32>> for Transform {
    fn into_rapier(self) -> Isometry<f32> {
        Isometry::from_parts(self.translation.into_rapier(), self.rotation.into_rapier())
    }
}