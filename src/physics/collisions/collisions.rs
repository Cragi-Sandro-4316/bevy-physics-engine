use super::{RigidBody, Velocity};

use bevy::prelude::*;

use nalgebra::{Isometry3, Point3};
use parry3d::bounding_volume::Aabb;
use parry3d::query::Contact;
use parry3d::query;

#[path = "./utils/utils.rs"]
mod utils;
use utils::*;

#[path = "./classes/collider.rs"]
pub mod collider;
use collider::*;


#[path = "./classes/octree.rs"]
mod octree;
use octree::*;

const MAX_ENTITIES: usize = 16; 
const MAX_DEPTH: usize = 4;

pub const TOLERANCE: f32 = 0.0;


/// Handles the broad phase collision detection.
/// spawns an entity containing the Broad Collison Groups
pub fn broad_phase(
    entity_query: Query<(Entity, &Collider, &Transform)>,
    mut commands: Commands
) {
    // initialize the octree
    let world_bounds = Aabb::new(
        Point3::new(-50.0, -50.0, -50.0), 
        Point3::new(50.0, 50.0, 50.0)
    );

    let mut octree = OctreeNode::new(world_bounds);

    for (entity, collider, transform) in entity_query.iter() {
        let physics_entity = PhysicsEntity {
            entity: entity,
            collider: collider.shape.clone_dyn(),
            transform: *transform,
        };

        octree.insert(physics_entity, MAX_ENTITIES, MAX_DEPTH, &mut commands);
        
    }

}

/// handles the narrow phase collision detection
pub fn narrow_phase(
    mut query: Query<(&mut Transform, &Chunk, &Collider, &RigidBody, &mut Velocity)>
) {

    let mut combinations = query.iter_combinations_mut();
    while let Some([
        (mut transform_1, chunk_1, collider_1, rigid_body_1, mut velocity_1),
        (mut transform_2, chunk_2, collider_2, rigid_body_2, mut velocity_2)
    ]) = combinations.fetch_next() {

        if chunk_1.0 == chunk_2.0 {
            let isometry_1 = transform_to_isometry(*transform_1);
            let isometry_2 = transform_to_isometry(*transform_2);

            collision_check(
                isometry_1, 
                collider_1, 
                rigid_body_1, 
                &mut transform_1, 
                &mut velocity_1,

                isometry_2, 
                collider_2, 
                rigid_body_2, 
                &mut transform_2,
                &mut velocity_2
            );
            
        }
    }
}


fn collision_check(
    isometry_1: Isometry3<f32>,
    collider_1: &Collider,
    rigid_body_1: &RigidBody,
    transform_1: &mut Transform,
    velocity_1: &mut Velocity,

    isometry_2: Isometry3<f32>,
    collider_2: &Collider,
    rigid_body_2: &RigidBody,
    transform_2: &mut Transform,
    velocity_2: &mut Velocity,
) {
    if let Some(contact) = query::contact(
        &isometry_1, 
        &*collider_1.shape, 
        &isometry_2, 
        &*collider_2.shape, 
        0.
    ).ok() {
        match contact {
            Some(contact) => {

                println!("contact");
                contact_handling(
                    contact, 
                    &rigid_body_1, 
                    transform_1, 
                    velocity_1,
                    &rigid_body_2, 
                    transform_2,
                    velocity_2
                );

            },
            None => {
                println!("no");
            }
        }
    }
}


/// Separates the entity pair.
/// Returns the separated transforms of the entities
fn contact_handling(
    contact: Contact,
    
    rigid_body_1: &RigidBody,
    transform_1: &mut Transform,
    velocity_1: &mut Velocity,

    rigid_body_2: &RigidBody,
    transform_2: &mut Transform,
    velocity_2: &mut Velocity,
)  {

    match (rigid_body_1, rigid_body_2) {
        (RigidBody::Static, RigidBody::Static) => {},
        (RigidBody::Static, RigidBody::Dynamic) => {
            separate_objects(transform_2, velocity_2, contact, 2);
        },
        (RigidBody::Dynamic, RigidBody::Static) => {
            separate_objects(transform_1, velocity_1, contact, 1);
        },
        (RigidBody::Dynamic, RigidBody::Dynamic) => {
            todo!()
        }
    }

}

/// separates the objects 
fn separate_objects(
    transform: &mut Transform,
    velocity: &mut Velocity,
    contact: Contact,
    entity: i8
) {
    if contact.dist > TOLERANCE {return}

    println!("contact dist {}", contact.dist);

    let normal = {
        if entity == 1 {
            contact.normal2.xyz()
        }
        else {
            contact.normal1.xyz()
        }
    };

    let normal_vec = Vec3::new(
        normal.x, 
        normal.y, 
        normal.z
    );
        
    let separation_vector = normal_vec * -contact.dist;
    
    // separate the objects
    transform.translation += separation_vector;

    
    // adjust the velocity
    let normal_velocity = velocity.0.dot(normal_vec);

    if normal_velocity < 0.0 {
        let impulse = (1.0 + 0.8) * normal_velocity;

        velocity.0 -= impulse * normal_vec;
    }
}