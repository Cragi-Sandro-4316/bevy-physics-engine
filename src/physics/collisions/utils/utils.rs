use nalgebra::{Isometry, Isometry3, Point3, Quaternion, Unit};
use parry3d::bounding_volume::Aabb;
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


pub fn subdivide_aabb(
    bounding_box: Aabb
) -> [Aabb; 8] {
    
    let min = bounding_box.mins;
    let max = bounding_box.maxs;
    let center = bounding_box.center();

    [
        // Bottom-left-front
        Aabb::new(min, center),
        // Bottom-right-front
        Aabb::new(Point3::new(center.x, min.y, min.z), Point3::new(max.x, center.y, center.z)),
        // Bottom-left-back
        Aabb::new(Point3::new(min.x, min.y, center.z), Point3::new(center.x, center.y, max.z)),
        // Bottom-right-back
        Aabb::new(Point3::new(center.x, min.y, center.z), Point3::new(max.x, center.y, max.z)),
        // Top-left-front
        Aabb::new(Point3::new(min.x, center.y, min.z), Point3::new(center.x, max.y, center.z)),
        // Top-right-front
        Aabb::new(Point3::new(center.x, center.y, min.z), Point3::new(max.x, max.y, center.z)),
        // Top-left-back
        Aabb::new(Point3::new(min.x, center.y, center.z), Point3::new(center.x, max.y, max.z)),
        // Top-right-back
        Aabb::new(center, max),
    ]
}


// converts a bevy transform into a parry Isometry for collision detection
pub fn transform_to_isometry(transform: Transform) -> Isometry<f32, Unit<Quaternion<f32>>, 3> {
    let mut isometry3d = Isometry3::translation(
        transform.translation.x, 
        transform.translation.y, 
        transform.translation.z
    );

    let collider_transform = transform.clone();


    let coll_rotation = collider_transform.rotation;

    isometry3d.rotation = Unit::new_normalize(Quaternion::new(
        coll_rotation.w,
        coll_rotation.x, 

        coll_rotation.y, 
        coll_rotation.z,     
    ));


    isometry3d
}