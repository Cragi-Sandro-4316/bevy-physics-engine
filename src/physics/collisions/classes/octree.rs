use bevy::prelude::Entity;
use parry3d::bounding_volume::Aabb;

const MAX_ENTITIES: u8 = 16; 


pub struct OctreeNode {
    children: Option<[Box<OctreeNode>; 8]>, // None if this is a leaf node
    bounding_box: Aabb,
    objects: Vec<Entity>
}
