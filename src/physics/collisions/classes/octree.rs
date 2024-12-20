
use bevy::prelude::*;
use parry3d::{bounding_volume::{Aabb, BoundingVolume}, shape::Shape};

use super::{subdivide_aabb, transform_to_isometry};

pub struct OctreeNode {
    children: Option<[Box<OctreeNode>; 8]>, // None if this is a leaf node
    bounding_box: Aabb,
    pub objects: Vec<PhysicsEntity>,
    chunk_num: i32
}

pub struct PhysicsEntity {
    pub entity: Entity,             // the bevy entity
    pub collider: Box<dyn Shape>,   // the collider shape
    pub transform: Transform,       // the bevy transform
}

#[derive(Component)]
pub struct Chunk(pub i32);

static mut CHUNK_NUMBER: i32 = 0;

impl OctreeNode {
    pub fn insert(
        &mut self, 
        physics_entity: PhysicsEntity,   // the PhysicsEntity to insert
        max_objects: usize,             // the max number of objects in a node
        max_depth: usize,                // the max depth of a node
        commands: &mut Commands
    ) {
        let entity_isometry = transform_to_isometry(physics_entity.transform);
        
        // if the node has children try inserting into one of them
        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                if child.bounding_box.contains(
                    &physics_entity.collider.compute_aabb(&entity_isometry)
                ) {
                    child.insert(physics_entity, max_objects, max_depth - 1, commands);
                    return;
                }
            }
        }

        // otherwise, store the object in this node
        // self.objects.push(physics_entity);

        commands.entity(physics_entity.entity).remove::<Chunk>();
        commands.entity(physics_entity.entity).insert(Chunk(self.chunk_num));

        // check if the node should be split
        if self.objects.len() > max_objects && max_depth > 0 {
            self.split(max_objects, max_depth, commands);
        }

    }



    fn split(
        &mut self, 
        max_objects: usize, 
        max_depth: usize,
        commands: &mut Commands
    ) {
        // Subdivide the current bounding box into eight smaller regions
        let sub_boxes = subdivide_aabb(self.bounding_box);

        unsafe { CHUNK_NUMBER = CHUNK_NUMBER + 1 }

        // Create eight child nodes, each with one of the new bounding boxes
        self.children = Some(sub_boxes.map(|bbox| Box::new(OctreeNode {
            bounding_box: bbox,
            objects: Vec::new(),
            children: None,
            chunk_num: unsafe { CHUNK_NUMBER }
        })));

        // Temporarily take the objects stored in this node
        let entities = std::mem::take(&mut self.objects);

        // Redistribute the objects into the appropriate child nodes
        for physics_entity in entities {
            // converts the bevy Transform to a parry3d Isometry for this entity
            let entity_isometry = transform_to_isometry(physics_entity.transform);

            for child in self.children.as_mut().unwrap().iter_mut() {
                if child.bounding_box.contains(
                    &physics_entity.collider.compute_aabb(&entity_isometry)
                ) {
                    child.insert(physics_entity, max_objects, max_depth - 1, commands);
                    break; // an object should only go in one child
                }
            }

        }

    }

    pub fn new(
        bounding_box: Aabb,
    ) -> Self {
        OctreeNode {
            bounding_box: bounding_box,
            children: None,
            objects: Vec::new(),
            chunk_num: 0
        }
    }

}
