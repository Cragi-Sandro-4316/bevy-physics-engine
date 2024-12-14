use bevy::prelude::*;

#[path="./physics/physics.rs"]
mod physics;
use physics::PhysicsPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugin
        ))
        .run();
}