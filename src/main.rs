use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};

#[path="./physics/physics.rs"]
mod physics;
use physics::PhysicsPlugin;

#[path="./game/camera/camera.rs"]
mod camera;
use camera::CameraPlugin;

#[path="./game/world/world.rs"]
mod level;
use level::LevelPlugin;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            LevelPlugin,
            PhysicsPlugin,
            
            FrameTimeDiagnosticsPlugin::default()

        ))
        .add_systems(Update, show_fps)
        .run();
}


fn show_fps (
    diagnostics: Res<DiagnosticsStore>,
) {
    if let Some(value) = diagnostics
    .get(&FrameTimeDiagnosticsPlugin::FPS)
    .and_then(|fps| fps.smoothed()){ 
        println!("{}", value);    
    }
    else {
        println!("no");
    }
}