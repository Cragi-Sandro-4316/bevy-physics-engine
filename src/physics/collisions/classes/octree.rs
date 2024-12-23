use std::rc::Rc;

use bevy::prelude::*;
use parry3d::{bounding_volume::{Aabb, BoundingVolume}, shape::Shape};


use super::{subdivide_aabb, transform_to_isometry};

pub struct OctreeNode {
    children: Option<[Box<OctreeNode>; 8]>, // None if this is a leaf node
    bounding_box: Aabb,
    pub objects: Vec<Rc<PhysicsEntity>>,
    pub chunk_num: i32
}

pub struct PhysicsEntity {
    pub entity: Entity,             // the bevy entity
    pub collider: Box<dyn Shape>,   // the collider shape
    pub transform: Transform,       // the bevy transform
}

#[derive(Component)]
pub struct Chunk(
    pub Vec<i32>
);

pub static mut CHUNK_NUMBER: i32 = 0;

impl OctreeNode {
    pub fn insert(
        &mut self, 
        physics_entity: Rc<PhysicsEntity>,   // the PhysicsEntity to insert
        max_objects: usize,             // the max number of objects in a node
        max_depth: usize,                // the max depth of a node
        chunk_query: &mut Query<&mut Chunk>,
        commands: &mut Commands
    ) {

        let entity_isometry = transform_to_isometry(physics_entity.transform);
        let collider = physics_entity.collider.compute_aabb(&entity_isometry).clone();
        
        // if the node has children try inserting into one of them
        if let Some(children) = &mut self.children {

            for child in children.iter_mut() {

                if child.bounding_box.intersects(
                    &collider
                ) {
                    child.insert(physics_entity.clone(), max_objects, max_depth - 1, chunk_query, commands);
                }
            }

            return;
        }

        // assign to the object the number of this chunk
        if let Ok(mut entity_chunks) = chunk_query.get_mut(physics_entity.entity) {
            
            if !entity_chunks.0.contains(&self.chunk_num)  {
                entity_chunks.0.push(self.chunk_num);

            }
        }
        else {
            let mut entity_chunks = Vec::new();
            entity_chunks.push(self.chunk_num);

            commands.entity(physics_entity.entity).insert(Chunk(entity_chunks));
        }
        
    
        self.objects.push(physics_entity);

        // check if the node should be split
        if self.objects.len() > max_objects && max_depth > 1  {
            
            self.split(max_objects, max_depth - 1, commands, chunk_query);
        }
        

    }



    fn split(
        &mut self, 
        max_objects: usize, 
        max_depth: usize,
        commands: &mut Commands,
        chunk_query: &mut Query<&mut Chunk>
    ) {
        // Subdivide the current bounding box into eight smaller regions
        let sub_boxes = subdivide_aabb(self.bounding_box);

        // Create eight child nodes, each with one of the new bounding boxes
        self.children = Some(sub_boxes.map(|bbox| Box::new(OctreeNode {
            bounding_box: bbox,
            objects: Vec::new(),
            children: None,
            chunk_num: unsafe { CHUNK_NUMBER }
        })));

        // assigns the chunks to the children boxes
        for child in self.children.as_mut().unwrap() {
            unsafe { CHUNK_NUMBER = CHUNK_NUMBER + 1 }

            child.chunk_num = unsafe { CHUNK_NUMBER };
        }        

        // Temporarily take the objects stored in this node
        let entities = std::mem::take(&mut self.objects);

        // Redistribute the objects into the appropriate child nodes
        for physics_entity in entities {
            // converts the bevy Transform to a parry3d Isometry for this entity
            let entity_isometry = transform_to_isometry(physics_entity.transform);

            // removes the current chunk from the entity to be reassigned
            if let Ok(mut chunks) = chunk_query.get_mut(physics_entity.entity) {
                let index = chunks.0.iter().position(|x| *x == self.chunk_num).unwrap();
                chunks.0.remove(index);
            }

            for child in self.children.as_mut().unwrap().iter_mut() {
               
                // checks each entity and inserts it in a chunk
                let collider = physics_entity.collider.compute_aabb(&entity_isometry).clone();
                if child.bounding_box.intersects(
                    &collider
                ) {

                    child.insert(physics_entity.clone(), max_objects, max_depth - 1, chunk_query, commands);
                    
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
