use bevy::prelude::*;
use parry3d::shape::*;

use super::extract_mesh_vertices_indices;


pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, assign_mesh_collider);
    }
}


/// A collider
#[derive(Component)]
pub struct Collider {
    pub shape: Box<dyn Shape>,
}

/// A collider generated from a mesh
#[derive(Component)]
pub struct MeshCollider(pub Handle<Mesh>);

// Generates a trimesh Collider from a mesh
fn assign_mesh_collider(
    collider_q: Query<(Entity, &MeshCollider)>,
    meshes: Res<Assets<Mesh>>,
    mut commands: Commands
) {
    for (ent, mesh_handle) in collider_q.iter() {


        if let Some(mesh) = meshes.get(&mesh_handle.0) {


            let verts_idxs = extract_mesh_vertices_indices(mesh).unwrap();

            let shared_shape = SharedShape::trimesh(
                verts_idxs.verts, 
                verts_idxs.indices, 
            );

            let compound_collider = Box::new(shared_shape.as_trimesh().unwrap().clone());

            
            commands.entity(ent).remove::<MeshCollider>();
            commands.entity(ent).insert(Collider {
                shape: compound_collider
            });

        }

        

    }

}
