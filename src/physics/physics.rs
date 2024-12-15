use bevy::prelude::*;

#[path = "./collisions/collisions.rs"]
mod collisions;
// use collisions::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.));
    }
}


#[derive(Component)]
pub enum RigidBody {
    Static,
    Dynamic {
        velocity: Vec3,
        mass: f32,
    }
}