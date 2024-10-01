use bevy::prelude::*;
use bevy_third_person_camera::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use menu::UiCameraExists;

mod player;
mod camera;
mod world;
mod menu;
mod algo;

use menu::MenuPlugin;
use player::PlayerPlugin;
use camera::CameraPlugin;
use world::WorldPlugin;
use algo::Prim;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                watch_for_changes_override: Some(true),
                ..Default::default()
            }), 
            PlayerPlugin, 
            CameraPlugin, 
            WorldPlugin, 
            MenuPlugin, 
            ThirdPersonCameraPlugin, 
            WorldInspectorPlugin::new(), 
            RapierPhysicsPlugin::<NoUserData>::default()
        ))
        .insert_resource(UiCameraExists(false))
        .run();

    
    // let maz = Prim::new(50, 50);

    // for row in maz {
    //     println!("{}", row.iter().collect::<String>());
    // }
}