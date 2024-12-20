use bevy::prelude::*;
use collisions::collider::Collider;
use nalgebra::Vector3;

use crate::physics::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                spawn_ground,
                spawn_cubes
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

    // spawn the test platform
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(10., 1., 10.))),
        MeshMaterial3d(materials.add(Color::linear_rgb(1.65, 1.92, 1.98))),
        Transform::from_xyz(0., 0., 0.),
        RigidBody::Static,
        Velocity(Vec3::ZERO),
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
        Mass(3.),
        collider_1
    ));

}