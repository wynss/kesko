use bevy::math::Vec3;
use std::convert::From;

impl From<&[[f32; 3]]> for Triangle {
    fn from(vertices: &[[f32; 3]]) -> Self {
        Self {
            v0: Vec3::from(vertices[0]),
            v1: Vec3::from(vertices[1]),
            v2: Vec3::from(vertices[2]),
        }
    }
}

impl From<[[f32; 3]; 3]> for Triangle {
    fn from(vertices: [[f32; 3]; 3]) -> Self {
        Self {
            v0: Vec3::from(vertices[0]),
            v1: Vec3::from(vertices[1]),
            v2: Vec3::from(vertices[2]),
        }
    }
}

pub(crate) struct Triangle {
    pub(crate) v0: Vec3,
    pub(crate) v1: Vec3,
    pub(crate) v2: Vec3,
}

impl Triangle {
    pub(crate) fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        Self { v0, v1, v2 }
    }
}
