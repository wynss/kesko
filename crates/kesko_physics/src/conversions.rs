use bevy::math::{Vec3, Quat};
use bevy::prelude::Transform;
use nalgebra::{UnitQuaternion, Vector3, Quaternion, Point3, UnitVector3};

use crate::rapier_extern::rapier::{
    math::{
        Translation, Isometry, Vector, Real, UnitVector
    }
};


pub trait IntoBevy<T> {
    #[must_use]
    fn into_bevy(self) -> T;
}

impl IntoBevy<Vec3> for Vector3<Real> {
    fn into_bevy(self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}


impl IntoBevy<Vec3> for Translation<Real> {
    fn into_bevy(self) -> Vec3 {
       self.vector.into_bevy()
    }
}

impl IntoBevy<Quat> for UnitQuaternion<Real> {
    fn into_bevy(self) -> Quat {
        Quat::from_xyzw(self.i as f32, self.j as f32, self.k as f32, self.w as f32)
    }
}

impl IntoBevy<(Vec3, Quat)> for Isometry<Real> {
    fn into_bevy(self) -> (Vec3, Quat) {
        (self.translation.into_bevy(), self.rotation.into_bevy())
    }
}

impl IntoBevy<Vec3> for UnitVector<Real> {
    fn into_bevy(self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.y as f32)
    }
}

pub trait IntoRapier<T> {
    #[must_use]
    fn into_rapier(self) -> T;
}

impl IntoRapier<Vector3<Real>> for Vec3 {
    fn into_rapier(self) -> Vector3<Real> {
        Vector3::new(self.x as Real, self.y as Real, self.z as Real)
    }
}

impl IntoRapier<Point3<Real>> for Vec3 {
    fn into_rapier(self) -> Point3<Real> {
        Point3::from([self.x as Real, self.y as Real, self.z as Real])
    }
}

impl IntoRapier<UnitVector3<Real>> for Vec3 {
    fn into_rapier(self) -> UnitVector3<Real> {
        UnitVector3::new_normalize(self.into_rapier())
    }
}

impl IntoRapier<Translation<Real>> for Vec3 {
    fn into_rapier(self) -> Translation<Real> {
        <Vec3 as IntoRapier<Vector<Real>>>::into_rapier(self).into()
    }
}

impl IntoRapier<UnitQuaternion<Real>> for Quat {
    fn into_rapier(self) -> UnitQuaternion<Real> {
        UnitQuaternion::new_normalize(Quaternion::new(self.w as Real, self.x as Real, self.y as Real, self.z as Real))
    }
}

impl IntoRapier<Isometry<Real>> for (Vec3, Quat) {
    fn into_rapier(self) -> Isometry<Real> {
        Isometry::from_parts(self.0.into_rapier(), self.1.into_rapier())
    }
}

impl IntoRapier<Isometry<Real>> for Transform {
    fn into_rapier(self) -> Isometry<Real> {
        Isometry::from_parts(self.translation.into_rapier(), self.rotation.into_rapier())
    }
}
