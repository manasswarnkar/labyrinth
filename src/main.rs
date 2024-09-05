use bevy::prelude::*;
use bevy_third_person_camera::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

mod player;
mod camera;
mod world;
mod menu;

use menu::MenuPlugin;
use player::PlayerPlugin;
use camera::CameraPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins, 
            PlayerPlugin, 
            CameraPlugin, 
            WorldPlugin, 
            MenuPlugin, 
            ThirdPersonCameraPlugin, 
            WorldInspectorPlugin::new(), 
            RapierPhysicsPlugin::<NoUserData>::default()
        ))
        .run();
}