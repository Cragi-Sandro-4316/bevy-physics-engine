use nalgebra::{Isometry, Isometry3, Quaternion, Unit};
use parry3d::shape::SharedShape;
use bevy::{prelude::*, render::mesh::{Indices, VertexAttributeValues}};


// A struct containing a mesh's vertices and indices
pub struct VerticesIndices {
    pub verts: Vec<nalgebra::Point3<f32>>, 
    pub indices: Vec<[u32; 3]>
}

// Extracts the vertices and indices of a mesh and returns them as a VerticesIndices instance
pub fn extract_mesh_vertices_indices(mesh: &Mesh) -> Option<VerticesIndices> {
    let vertices = mesh.attribute(Mesh::ATTRIBUTE_POSITION)?;
    let indices = mesh.indices()?;

    let vtx: Vec<_> = match vertices {
        VertexAttributeValues::Float32(vtx) => Some(
            vtx.chunks(3)
                .map(|v| [v[0], v[1], v[2]].into())
                .collect(),
        ),
        VertexAttributeValues::Float32x3(vtx) => Some(
            vtx.iter()
                .map(|v| [v[0], v[1], v[2]].into())
                .collect(),
        ),
        _ => None,
    }?;

    let idx = match indices {
        Indices::U16(idx) => idx
            .chunks_exact(3)
            .map(|i| [i[0] as u32, i[1] as u32, i[2] as u32])
            .collect(),
        Indices::U32(idx) => idx.chunks_exact(3).map(|i| [i[0], i[1], i[2]]).collect(),
    };

    Some(VerticesIndices {
        verts: vtx, 
        indices: idx
    })
}


