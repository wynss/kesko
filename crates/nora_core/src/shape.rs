use std::f32::consts::PI;
use bevy::render::mesh::{PrimitiveTopology, Indices, Mesh, shape};
use bevy::math::Vec3;

use nora_physics::collider::ColliderShape;


/// A cylinder which stands on the XZ plane
pub struct Cylinder {
    /// Radius of the cylinder (X&Z axis)
    pub radius: f32,
    /// Height of the cylinder (Y axis)
    pub height: f32,
    /// Number of vertices around each horizontal slice of the cylinder
    pub resolution: u32,
    /// Number of vertical subdivisions
    pub rings: u32,
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            radius: 0.5,
            height: 1.0,
            resolution: 50,
            rings: 7
        }
    }
}

impl From<Cylinder> for Mesh {
    fn from(cylinder: Cylinder) -> Self {

        let Cylinder {
            radius,
            height,
            resolution,
            rings
        } = cylinder;

        // the two comes from the vertices centered at the top and bottom circles respectively
        let num_vertices = (resolution * (rings + 3) + 2) as usize;

        // vertex positions
        let mut positions = Vec::with_capacity(num_vertices);

        let rad_step = 2.0 * PI / resolution as f32;
        let mut add_ring = |height, with_center| {
            if with_center {
                positions.push([0.0, height, 0.0]);
            }
            for j in 0..resolution {
                let theta = rad_step * j as f32;
                positions.push([radius * theta.cos(), height, radius * theta.sin()]);
            }
        };

        // Shaft vertices
        let h_step = height / rings as f32;
        for i in 0..(rings + 1) {
            add_ring(height * 0.5 - h_step * i as f32, false);
        }

        // Top vertices
        let top_offset = resolution * (rings + 1);
        add_ring(height * 0.5, true);

        // Bottom vertices
        let bottom_offset = top_offset + resolution + 1;
        add_ring(-height * 0.5, true);
        assert_eq!(positions.len(), num_vertices);

        // Index buffer
        let index_count = ((6 * rings * resolution) + 6 * resolution) as usize;
        let mut indices = Vec::with_capacity(index_count);

        // Shaft quads
        for i in 0..rings {
            let base1 = resolution * i;
            let base2 = base1 + resolution;
            for j in 0..resolution {
                let j1 = (j + 1) % resolution;
                indices.extend([base2 + j, base1 + j, base1 + j1].iter().copied());
                indices.extend([base2 + j, base1 + j1, base2 + j1].iter().copied());
            }
        }

        // Top and bottom circle triangles
        for j in 0..resolution {
            let j1 = (j + 1) % resolution;
            let base_top = top_offset + 1;
            let base_bottom = bottom_offset + 1;
            indices.extend([top_offset, base_top + j1, base_top + j].iter().copied());
            indices.extend([bottom_offset, base_bottom + j, base_bottom + j1].iter().copied());
        }

        assert_eq!(indices.len(), index_count);

        // Shaft normals are their X and Z coordinates normalized
        let mut normals = positions
            .iter()
            .map(|&p| {
                (Vec3::from(p) * Vec3::new(1.0, 0.0, 1.0))
                    .normalize()
                    .into()
            })
            .collect::<Vec<[f32; 3]>>();

        // Give the top and bottom of the cylinder a clear up/down normal
        for i in top_offset..bottom_offset {
            normals[i as usize] = [0.0, 1.0, 0.0];
        }
        for i in bottom_offset..num_vertices as u32 {
            normals[i as usize] = [0.0, -1.0, 0.0];
        }

        let uvs: Vec<[f32; 2]> = positions
            .iter()
            .map(|&p| [p[0] / radius, (p[1] + height) / (height * 2.0)])
            .collect();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}

#[derive(Debug)]
pub enum Shape {
    Sphere {
        radius: f32,
        subdivisions: usize,
    },
    Cube {
        size: f32,
    },
    Box {
        x_length: f32,
        y_length: f32,
        z_length: f32,
    },
    Cylinder {
        radius: f32,
        length: f32,
        resolution: u32
    },
    Capsule {
        radius: f32,
        length: f32
    }
}

impl Shape {
    pub fn into_mesh(&self) -> Option<Mesh> {
        match self {
            Self::Sphere { radius , subdivisions} => {
                Some(Mesh::from(shape::Icosphere { radius: *radius, subdivisions: *subdivisions}))
            },
            Self::Box { x_length, y_length, z_length } => {
                Some(Mesh::from(shape::Box::new(*x_length, *y_length, *z_length)))
            },
            Self::Cylinder { radius, length, resolution} => {
                Some(Mesh::from(Cylinder { radius: *radius, height: *length, resolution: *resolution, ..Default::default()}))
            },
            Self::Capsule { radius, length } => {
                Some(Mesh::from(shape::Capsule{ radius: *radius, depth: *length, ..Default::default()}))
            },
            Self::Cube {size} => {
                Some(Mesh::from(shape::Box::new(*size, *size, *size)))
            }
        } 
    }

    pub fn into_collider_shape(&self) -> ColliderShape {
        match self {
            Self::Sphere { radius , subdivisions: _} => {
                ColliderShape::Sphere { radius: *radius }
            },
            Self::Box { x_length, y_length, z_length } => {
                ColliderShape::Cuboid { x_half: x_length / 2.0, y_half: y_length / 2.0, z_half: z_length / 2.0 }
            },
            Self::Cylinder { radius, length, resolution: _} => {
                ColliderShape::Cylinder { radius: *radius, length: *length }
            },
            Self::Capsule { radius, length } => {
                ColliderShape::CapsuleY { half_length: length / 2.0, radius: *radius }
            },
            Self::Cube { size } => {
                ColliderShape::Cuboid { x_half: size / 2.0, y_half: size / 2.0, z_half: size / 2.0 }
            }
        } 
    }
}