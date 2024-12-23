
use bevy::prelude::*;
use collisions::{broad_phase, collider::ColliderPlugin, narrow_phase};

#[path = "./collisions/collisions.rs"]
pub mod collisions;

pub struct PhysicsPlugin;

const UPDATE_FREQUENCY: f32 = 30.;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Time::<Fixed>::from_hz(UPDATE_FREQUENCY.into()))
            .add_plugins(ColliderPlugin)
            .add_systems(FixedUpdate, (
                apply_gravity,
                apply_velocity,
                broad_phase,
                narrow_phase,
            ).chain());
    }
}


#[derive(Component, Clone, Copy)]
pub enum RigidBody {
    Static,
    Dynamic
}

#[derive(Component, Clone, Copy)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Mass(pub f32);

pub const GRAVITY: f32 = 9.8;

pub const DELTA: f32 = 1. / UPDATE_FREQUENCY;

pub const TERMINAL_VELOCITY: f32 = 100.;



fn apply_gravity(
    mut query: Query<(&mut RigidBody, &mut Velocity, &Mass)>
) {
    for (rigid_body, mut velocity, mass) in query.iter_mut() {
        
        match *rigid_body {
            RigidBody::Static => {}
            RigidBody::Dynamic => {
                velocity.0.y -= mass.0 / GRAVITY;

                if velocity.0.y <= -TERMINAL_VELOCITY {
                    velocity.0.y = -TERMINAL_VELOCITY;
                }

            }
        }
    }

}

fn apply_velocity(
    mut query: Query<(&mut RigidBody, &mut Transform, &Velocity)>
) {
    for (rigid_body, mut tranform, velocity) in query.iter_mut() {
        
        match *rigid_body {
            RigidBody::Static => {}
            RigidBody::Dynamic => {
                tranform.translation += velocity.0 * DELTA;
            }
        }
    }
}