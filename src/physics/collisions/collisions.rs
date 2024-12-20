use super::{Mass, RigidBody, Velocity};

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
    mut query: Query<(
        &mut Transform, 
        &Chunk, 
        &Collider, 
        &RigidBody, 
        Option<&mut Velocity>,
        Option<&Mass>
    )>
) {

    let mut combinations = query.iter_combinations_mut();
    while let Some([
        (mut transform_1, chunk_1, collider_1, rigid_body_1, velocity_1, mass_1),
        (mut transform_2, chunk_2, collider_2, rigid_body_2, velocity_2, mass_2)
    ]) = combinations.fetch_next() {

        if chunk_1.0 == chunk_2.0 {
            let isometry_1 = transform_to_isometry(*transform_1);
            let isometry_2 = transform_to_isometry(*transform_2);

            collision_check(
                isometry_1, 
                collider_1, 
                rigid_body_1, 
                &mut transform_1, 
                velocity_1,
                mass_1,

                isometry_2, 
                collider_2, 
                rigid_body_2, 
                &mut transform_2,
                velocity_2,
                mass_2
            );
            
        }
    }
}


fn collision_check(
    isometry_1: Isometry3<f32>,
    collider_1: &Collider,
    rigid_body_1: &RigidBody,
    transform_1: &mut Transform,
    velocity_1: Option<Mut<'_, Velocity>>,
    mass_1: Option<&Mass>,

    isometry_2: Isometry3<f32>,
    collider_2: &Collider,
    rigid_body_2: &RigidBody,
    transform_2: &mut Transform,
    velocity_2: Option<Mut<'_, Velocity>>,
    mass_2: Option<&Mass>,

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

                contact_handling(
                    contact, 
                    &rigid_body_1, 
                    transform_1, 
                    velocity_1,
                    mass_1,

                    &rigid_body_2, 
                    transform_2,
                    velocity_2,
                    mass_2
                );

            },
            None => {}
        }
    }
}


/// Separates the entity pair.
/// Returns the separated transforms of the entities
fn contact_handling(
    contact: Contact,
    
    rigid_body_1: &RigidBody,
    transform_1: &mut Transform,
    velocity_1: Option<Mut<'_, Velocity>>,
    mass_1: Option<&Mass>,


    rigid_body_2: &RigidBody,
    transform_2: &mut Transform,
    velocity_2: Option<Mut<'_, Velocity>>,
    mass_2: Option<&Mass>,
)  {

    match (rigid_body_1, rigid_body_2) {
        (RigidBody::Static, RigidBody::Static) => {},
        (RigidBody::Static, RigidBody::Dynamic) => {
            if let Some(mut velocity) = velocity_2 {
                separate_objects(transform_2, &mut velocity, contact, 2);
            }

        },
        (RigidBody::Dynamic, RigidBody::Static) => {
            if let Some(mut velocity) = velocity_1 {
                separate_objects(transform_1, &mut velocity, contact, 1);
            }
        },
        (RigidBody::Dynamic, RigidBody::Dynamic) => {
            // todo!()

            let mut velocity_1 = velocity_1.unwrap();
            let mut velocity_2 = velocity_2.unwrap();

            let mass_1 = mass_1.unwrap();
            let mass_2 = mass_2.unwrap();
            
            separate_dynamic(
                transform_1, 
                &mut velocity_1, 
                mass_1.0, 
                transform_2, 
                &mut velocity_2, 
                mass_2.0, 
                contact
            );
        }
    }

}


const RESTITUTION: f32 = 0.4;

/// separates the objects 
fn separate_objects(
    transform: &mut Transform,
    velocity: &mut Velocity,
    contact: Contact,
    entity: i8
) {
    if contact.dist > TOLERANCE {return}


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
        let impulse = (1.0 + RESTITUTION) * normal_velocity;

        velocity.0 -= impulse * normal_vec;
    }

    // println!("velocity: {}", velocity.0);
}


/// Handles collisions between two dynamic rigid bodies
fn separate_dynamic(
    transform_1: &mut Transform,
    velocity_1: &mut Velocity,
    mass_1: f32,

    transform_2: &mut Transform,
    velocity_2: &mut Velocity,
    mass_2: f32,

    contact: Contact
) {
    if contact.dist > TOLERANCE { return; }

    // Calculate the collision normal
    let normal = contact.normal1.xyz();
    let normal_vec = Vec3::new(normal.x, normal.y, normal.z);

    // Resolve penetration by moving both objects
    let separation_vector = normal_vec * -contact.dist;
    let total_mass = mass_1 + mass_2;

    transform_1.translation -= separation_vector * (mass_2 / total_mass);
    transform_2.translation += separation_vector * (mass_1 / total_mass);

    // Compute relative velocity along the collision normal
    let relative_velocity = velocity_2.0 - velocity_1.0;
    let normal_velocity = relative_velocity.dot(normal_vec);

    if normal_velocity >= 0.0 {
        // Objects are separating; no impulse needed
        return;
    }

    // Compute impulse scalar
    let impulse = -(1.0 + RESTITUTION) * normal_velocity / (1.0 / mass_1 + 1.0 / mass_2);

    // Apply impulse to both velocities
    velocity_1.0 -= (impulse / mass_1) * normal_vec;
    velocity_2.0 += (impulse / mass_2) * normal_vec;

    // println!("velocity 1: {}", velocity_1.0);
    // println!("velocity 2: {}", velocity_2.0);
}