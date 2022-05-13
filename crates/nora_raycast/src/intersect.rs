use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};

use crate::{Ray, RayCastable, RayCastSource};
use crate::triangle::{triangle_normal, Triangle, inside_triangle};
use crate::convert::IntoUsize;


const EPSILON: f32 = 1e-8;

#[derive(Clone)]
pub(crate) struct RayHit {
    pub(crate) intersection: RayIntersection,
}

#[derive(Clone)]
pub(crate) struct RayIntersection {
    pub(crate) point: Vec3,
    pub(crate) normal: Vec3,
    pub(crate) distance: f32
}

impl RayIntersection {
    fn transform(&self, transform: &Mat4, ray_origin: &Vec3) -> Self {
        let point_transformed = transform.transform_point3(self.point);
        Self {
            point: point_transformed,
            normal: transform.transform_vector3(self.normal),
            distance: ray_origin.distance(point_transformed)
        }
    }
}

/// Checks if a ray intersects a Mesh, if so the closest point of intersection is returned
pub(crate) fn mesh_intersection(mesh: &Mesh, ray: &Ray, mesh_to_world: &Mat4) -> Option<RayIntersection> {

    if mesh.primitive_topology() != PrimitiveTopology::TriangleList {
        error!("Only work with triangle list topology for now");
        return None;
    }

    let vertex_positions = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        Some(vertex_values) => match vertex_values {
            VertexAttributeValues::Float32x3(positions) => positions,
            _ => panic!("Unsupported vertex attribute values type {:?}", vertex_values)
        },
        None => panic!("Mesh does not have any vertex positions")
    };

    let vertex_normals = match mesh.attribute(Mesh::ATTRIBUTE_NORMAL) {
        Some(vertex_values) => match vertex_values {
            VertexAttributeValues::Float32x3(normals) => normals,
            _ => panic!("Unsupported vertex attribute values type {:?}", vertex_values)
        },
        None => panic!("Could not get vertex normal for mesh")

    };

    if let Some(indices) = mesh.indices() {
        return match indices {
            Indices::U16(indices) => {
                mesh_intersection_with_vertices(
                    ray,
                    mesh_to_world,
                    vertex_positions,
                    vertex_normals,
                    indices
                )
            },
            Indices::U32(indices) => {
                mesh_intersection_with_vertices(
                    ray,
                    mesh_to_world,
                    vertex_positions,
                    vertex_normals,
                    indices
                )
            }
        }
    }

    None
}


fn mesh_intersection_with_vertices(
    ray: &Ray,
    mesh_to_world: &Mat4,
    vertex_positions: &[[f32; 3]],
    vertex_normals: &[[f32; 3]],
    vertex_indices: &[impl IntoUsize]
) -> Option<RayIntersection>
{

    let world_to_mesh = mesh_to_world.inverse();
    let ray_mesh = ray.transform(&world_to_mesh);

    // loop over triangles
    let mut min_intersection_distance = f32::MAX;
    let mut closest_intersection: Option<RayIntersection> = None;

    for indices in vertex_indices.chunks(3) {
        let triangle = Triangle::new(
            Vec3::from(vertex_positions[indices[0].into_usize()]),
            Vec3::from(vertex_positions[indices[1].into_usize()]),
            Vec3::from(vertex_positions[indices[2].into_usize()]),
        );

        let normals = [
            Vec3::from(vertex_normals[indices[0].into_usize()]),
            Vec3::from(vertex_normals[indices[1].into_usize()]),
            Vec3::from(vertex_normals[indices[2].into_usize()]),
        ];

        if let Some(intersection) = triangle_intersect_mt(&triangle, &normals, &ray_mesh) {

            let intersection_world = intersection.transform(mesh_to_world, &ray.origin);

            if intersection_world.distance < min_intersection_distance {
                min_intersection_distance = intersection_world.distance;
                closest_intersection = Some(intersection_world);
            }
        }
    }

    closest_intersection
}

/// Checks if a ray intersects a triangle given its vertices using the Möller-Trumbore algorithm.
///
/// Note that the vertices are relative to the mesh so the ray also need to be in the mesh frame of reference.
fn triangle_intersect_mt(triangle: &Triangle, normals: &[Vec3], ray: &Ray) -> Option<RayIntersection> {

    let v0v1 = triangle.v1 - triangle.v0;
    let v0v2 = triangle.v2 - triangle.v0;
    let p = ray.direction.cross(v0v2);
    let det = v0v1.dot(p);

    if det < 0.0 {
        return None;
    }

    if det.abs() < EPSILON {
        return None;
    }

    let det_inv = 1.0 / det;

    let t = ray.origin - triangle.v0;
    let u = t.dot(p) * det_inv;

    if !(0.0..=1.0).contains(&u) {
        return None;
    }

    let q = t.cross(v0v1);
    let v = ray.direction.dot(q) * det_inv;

    if !(0.0..=1.0).contains(&v) {
        return None;
    }

    let t = v0v2.dot(q) * det_inv;
    if t < EPSILON {
        return None;
    }

    let intersection = ray.origin + t * ray.direction;

    let normal = u * normals[0] + v * normals[1] + (1.0 - u - v) * normals[2];

    Some(RayIntersection {
        point: intersection,
        normal,
        distance: ray.origin.distance(intersection)
    })
}
