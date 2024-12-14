use super::RigidBody;
use bevy::asset::Handle;
use bevy::prelude::*;

use parry3d::shape::*;

#[path = "./utils.rs"]
mod utils;
use utils::*;

#[derive(Component)]
pub struct Collider {
    shape: Box<dyn Shape>,
    chunk: u16
}

