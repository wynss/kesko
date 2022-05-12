use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};

use crate::{Ray, RayCastable, RayCastSource};
use crate::triangle::{triangle_normal, Triangle, inside_triangle};
use crate::convert::IntoUsize;


const EPSILON: f32 = 1e-8;

#[derive(Clone)]
pub(crate) struct Intersection {
    pub(crate) intersection: Vec3,
    pub(crate) distance: f32,
    // pub(crate) normal: Vec3,
    // mesh_id: Handle<Mesh>
}

pub(crate) fn calc_intersections_system(
    meshes: Res<Assets<Mesh>>,
    mut source_query: Query<&mut RayCastSource>,
    castable_query: Query<(&Handle<Mesh>, &GlobalTransform), With<RayCastable>>
) {
    for mut source in source_query.iter_mut() {

        if let Some(ray) = &source.ray {

            let mut intersections: Vec<Intersection> = Vec::new();
            for (mesh_handle, transform) in castable_query.iter() {

                if let Some(mesh) = meshes.get(mesh_handle) {

                    let mesh_to_world = transform.compute_matrix();

                    if let Some(intersection) = mesh_intersection(mesh, ray, &mesh_to_world) {
                        intersections.push(intersection);
                    }

                } else {
                    error!("No mesh for mesh handle");
                    continue;
                }
            }

            // todo check for closest intersection here and assign it to the source
            let min_distance = f32::MAX;
            let mut closest_intersection: Option<Intersection> = None;
            for intersection in intersections.iter() {
                if intersection.distance < min_distance {
                    closest_intersection = Some(intersection.clone());
                }
            }

            if let Some(intersection) = closest_intersection {
                source.intersection = Some(intersection);
            }

        };
    }
}

/// Checks if a ray intersects a Mesh, if so the closest point of intersection is returned
fn mesh_intersection(mesh: &Mesh, ray: &Ray, mesh_to_world: &Mat4) -> Option<Intersection> {

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

    if let Some(indices) = mesh.indices() {
        return match indices {
            Indices::U16(indices) => {
                mesh_intersection_with_vertices(
                    ray,
                    mesh_to_world,
                    vertex_positions,
                    indices
                )
            },
            Indices::U32(indices) => {
                mesh_intersection_with_vertices(
                    ray,
                    mesh_to_world,
                    vertex_positions,
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
    vertex_indices: &[impl IntoUsize]
) -> Option<Intersection>
{

    let world_to_mesh = mesh_to_world.inverse();
    let ray_mesh = ray.transform(&world_to_mesh);

    // loop over triangles
    let mut min_intersection_distance = f32::MAX;
    let mut closest_intersection: Option<Intersection> = None;

    for indices in vertex_indices.chunks(3) {
        let triangle = Triangle::new(
            Vec3::from(vertex_positions[indices[0].into_usize()]),
            Vec3::from(vertex_positions[indices[1].into_usize()]),
            Vec3::from(vertex_positions[indices[2].into_usize()]),
        );

        if let Some(intersection) = triangle_intersect_mt(&triangle, &ray_mesh) {

            let intersection_world = mesh_to_world.transform_point3(intersection);
            let distance = ray.origin.distance(intersection_world);

            if distance < min_intersection_distance {
                min_intersection_distance = distance;
                closest_intersection = Some(Intersection {
                    intersection: intersection_world,
                    distance
                });
            }
        }
    }

    closest_intersection
}

/// Checks if a ray intersects a triangle given its vertices using the Möller-Trumbore algorithm.
///
/// Note that the vertices are in relative to the mesh so the ray also need to be in the mesh frame
fn triangle_intersect_mt(triangle: &Triangle, ray: &Ray) -> Option<Vec3> {

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

    let intersection = ray.origin + t * ray.direction;
    Some(intersection)
}
