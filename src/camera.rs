use bevy::prelude::*;
use bevy_third_person_camera::*;
use crate::menu::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), spawn_camera)
            .add_systems(OnExit(GameState::InGame), despawn_camera);
    }
}


#[derive(Component)]
struct GameCamera;

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        ThirdPersonCamera{
            zoom: Zoom::new(1.0, 10.0),
            ..default()
        },
        GameCamera,
    );

    commands.spawn(camera);
}

fn despawn_camera(mut commands: Commands, query: Query<Entity, With<GameCamera>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}