use super::RigidBody;
use bevy::asset::Handle;
use bevy::prelude::*;

use parry3d::shape::*;

#[path = "./utils/utils.rs"]
mod utils;
use utils::*;

#[path = "./classes/collider.rs"]
mod collider;
use collider::*;


#[path = "./classes/octree.rs"]
mod octree;
use octree::*;


