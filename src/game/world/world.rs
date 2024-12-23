use bevy::prelude::*;
use collisions::collider::{Collider, MeshCollider};
use nalgebra::Vector3;
use rand::Rng;

use crate::physics::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                spawn_objects,
                spawn_cubes,
                spawn_concave_obj,
                spawn_test_plat
            ));
    }
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



fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,

) {
    let mut rng = rand::thread_rng();

    let num = 1000;

    // spawn cubes
    for _ in 0..num {
        let shape_box = Box::new(
            parry3d::shape::Cuboid::new(Vector3::new(0.5, 0.5, 0.5))
        );
    
        let collider_1 = Collider {
            shape: shape_box,
        };
    
        let random_color = Color::linear_rgb(
            rng.gen_range(0..100) as f32 / 100., 
            rng.gen_range(0..100) as f32 / 100.,  
            rng.gen_range(0..100) as f32 / 100.
        );

        let random_pos = Vec3::new(
            rng.gen_range(-20..20) as f32, 
            rng.gen_range(5..10) as f32, 
            rng.gen_range(-20..20) as f32, 
        );

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))),
            MeshMaterial3d(materials.add(random_color)),
            Transform::from_translation(random_pos),
            RigidBody::Dynamic,
            Velocity(Vec3::ZERO),
            Mass(1.),
            collider_1
        ));
    

    
    }
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


#[derive(Component)]
pub struct Ground;

fn spawn_test_plat(
    assets: Res<AssetServer>,
    mut commands: Commands,
) {

    let collider_shape = assets.load("./test_plat.glb#Mesh0/Primitive0"); 

    let mesh_collider = MeshCollider(collider_shape);

    let scene = SceneRoot(assets.load(
        GltfAssetLabel::Scene(0).from_asset("./test_plat.glb")
    ));

    commands.spawn((
        scene,
        mesh_collider,
        Transform::from_xyz(0., 10., 0.),
        RigidBody::Static,
        Ground
    ));
}