use bevy::prelude::*;
use collisions::collider::{Collider, MeshCollider};
use nalgebra::Vector3;

use crate::physics::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                spawn_ground,
                spawn_cubes,
                spawn_concave_obj
            ));
    }
}



fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let shape_box = Box::new(
        parry3d::shape::Cuboid::new(Vector3::new(5.0, 0.5, 5.0))
    );

    let collider = Collider {
        shape: shape_box,
    };

    let transform = Transform::from_xyz(0., 0., 0.);
    
    // spawn the test platform
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(10., 1., 10.))),
        MeshMaterial3d(materials.add(Color::linear_rgb(1.65, 1.92, 1.98))),
        transform,
        RigidBody::Static,
        collider
    ));
}


fn spawn_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let shape_box_1 = Box::new(
        parry3d::shape::Cuboid::new(Vector3::new(0.5, 0.5, 0.5))
    );

    let collider_1 = Collider {
        shape: shape_box_1,
    };

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))),
        MeshMaterial3d(materials.add(Color::linear_rgb(1., 1., 0.))),
        Transform::from_xyz(-1.5, 7., 0.),
        RigidBody::Dynamic,
        Velocity(Vec3::ZERO),
        Mass(1.),
        collider_1
    ));

    let shape_box_2 = Box::new(
        parry3d::shape::Cuboid::new(Vector3::new(0.5, 0.5, 0.5))
    );

    let collider_2 = Collider {
        shape: shape_box_2,
    };

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))),
        MeshMaterial3d(materials.add(Color::linear_rgb(1., 1., 0.))),
        Transform::from_xyz(1.5, 7., 0.),
        RigidBody::Dynamic,
        Velocity(Vec3::ZERO),
        Mass(3.),
        collider_2
    ));

}

fn spawn_concave_obj(
    assets: Res<AssetServer>,
    mut commands: Commands,
) {

    let collider_shape = assets.load("./concave.glb#Mesh0/Primitive0"); 

    let mesh_collider = MeshCollider(collider_shape);

    let scene = SceneRoot(assets.load(
        GltfAssetLabel::Scene(0).from_asset("./concave.glb")
    ));

    commands.spawn((
        scene,
        mesh_collider,
        Transform::from_xyz(0., 7., 0.),
        Velocity(Vec3::ZERO),
        Mass(1.),
        RigidBody::Dynamic

    ));
        

    

    
}