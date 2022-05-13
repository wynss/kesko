use std::convert::From;
use bevy::math::Vec3;


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

pub(crate) fn triangle_normal(triangle: &Triangle) -> Vec3 {
    (triangle.v1 - triangle.v0).cross(triangle.v2 - triangle.v0)
}

pub(crate) fn inside_triangle(triangle: &Triangle, normal: &Vec3, point: &Vec3) -> bool {

    let edge0 = triangle.v1 - triangle.v0;
    let edge1 = triangle.v2 - triangle.v1;
    let edge2 = triangle.v0 - triangle.v2;
    let v0_to_p = *point - triangle.v0;
    let v1_to_p = *point - triangle.v1;
    let v2_to_p = *point - triangle.v2;

    if normal.dot(edge0.cross(v0_to_p)) < 0.0 {
        return false;
    }

    if normal.dot(edge1.cross(v1_to_p)) < 0.0 {
        return false;
    }

    if normal.dot(edge2.cross(v2_to_p)) < 0.0 {
        return false;
    }

    true
}


#[cfg(test)]
mod tests {
    use super::*;

    const V0: [f32; 3] = [1.0, 5.0, -1.0];
    const V1: [f32; 3] = [-1.0, 5.0, -1.0];
    const V2: [f32; 3] = [0.0, 5.0, 2.0];

    #[test]
    fn test_triangle_normal() {

        let triangle: Triangle = [V0, V1, V2].into();
        let normal = triangle_normal(&triangle);
        assert_eq!(normal.normalize(), Vec3::Y);
    }

    #[test]
    fn test_inside_triangle() {

        let triangle: Triangle = [V0, V1, V2].into();
        let normal = Vec3::Y;

        // check inside
        let inside_point = Vec3::new(0.0, 5.0, 0.0);
        let res = inside_triangle(&triangle, &normal, &inside_point);
        assert!(res);

        // check outside
        let outside_point = Vec3::new(5.0, 5.0, -5.0);
        let res = inside_triangle(&triangle, &normal, &outside_point);
        assert!(!res);
    }
}