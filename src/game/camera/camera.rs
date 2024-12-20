use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera);
    }
}


fn spawn_camera(mut commands: Commands) {

    commands.spawn((
        PointLight {
            color: Color::WHITE,
            intensity: 1000000.,
            range: 1000.,
            radius: 10000.,
            shadows_enabled: true,
            shadow_depth_bias: 100.,
            ..default()
        },
        Transform::from_xyz(0., 3., -1.)

    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 3., -30.).looking_at(Vec3::splat(0.), Vec3::Y)
    ));

}
