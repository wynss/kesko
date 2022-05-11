use bevy::prelude::*;
use bevy::render::mesh::{
    PrimitiveTopology, VertexAttributeValues
};

use crate::{Ray, RayCastable, RayCastSource};
use crate::triangle::{triangle_normal, Triangle, inside_triangle};

#[derive(Clone)]
pub(crate) struct Intersection {
    pub(crate) intersection: Vec3,
    pub(crate) distance: f32
    // normal: Vec3,
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

                    if let Some(intersection) = ray_mesh_intersect(mesh, ray, &mesh_to_world) {
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
                println!("Hit!!!!");
                source.intersection = Some(intersection);
            }

        };
    }
}

/// Checks if a ray intersects a Mesh, if so the closest point of intersection is returned
fn ray_mesh_intersect(mesh: &Mesh, ray: &Ray, mesh_to_world: &Mat4) -> Option<Intersection> {

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


    let world_to_mesh = mesh_to_world.inverse();
    let ray_mesh = ray.transform(&world_to_mesh);

    // loop over triangles
    let mut min_intersection_distance = f32::MAX;
    let mut closest_intersection: Option<Intersection> = None;
    for triangle_vertices in vertex_positions.chunks(3) {

        let triangle: Triangle = triangle_vertices.into();
        if let Some(intersection) = triangle_intersect(&triangle, &ray_mesh) {

            let intersection_world = mesh_to_world.transform_point3(intersection);
            let distance = ray.origin.distance(intersection_world);

            if distance < min_intersection_distance {
                min_intersection_distance = distance;
                closest_intersection = Some(Intersection {
                    intersection,
                    distance
                });
            }
        }
    }

    closest_intersection
}

/// Checks if a ray intersects a triangle given its vertices. Note that the vertices are in relative to the mesh so
/// the ray also need to be in the mesh frame
fn triangle_intersect(triangle: &Triangle, ray: &Ray) -> Option<Vec3> {

    let normal = triangle_normal(triangle);
    let distance = normal.dot(triangle.v0);

    let normal_dot_ray_direction = normal.dot(ray.direction);
    if normal_dot_ray_direction.abs() < 1e-5 {
        // the ray is parallel with the plane of the triangle
        return None;
    }

    // distance from ray origin to plane intersection
    let t = - ( normal.dot(ray.origin) + distance ) / normal_dot_ray_direction;

    if t < 0.0 {
        // the triangle is behind the rays origin, ignore
        return None;
    }

    let plane_intersection = ray.origin + t * ray.direction;

    if inside_triangle(triangle, &normal, &plane_intersection) {
        return Some(plane_intersection);
    }

    None
}
