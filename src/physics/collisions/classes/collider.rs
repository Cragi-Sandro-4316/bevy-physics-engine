use bevy::prelude::*;
use parry3d::shape::*;

use super::extract_mesh_vertices_indices;

#[derive(Component)]
pub struct Collider {
    pub shape: Box<dyn Shape>,
}


impl Collider {
    pub fn _from_mesh(
        &mut self,
        mesh_handle: Handle<Mesh>,
        mesh_server: Res<Assets<Mesh>>
    ) {
        
        if let Some(mesh) = mesh_server.get(&mesh_handle) {
            let verts_idxs = extract_mesh_vertices_indices(mesh).unwrap();
        
            let shared_shape = SharedShape::trimesh(
                verts_idxs.verts, 
                verts_idxs.indices, 
            );

            self.shape = Box::new(shared_shape.as_trimesh().unwrap().clone());
        }
    }


}