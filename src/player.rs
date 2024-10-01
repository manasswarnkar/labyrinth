use bevy::prelude::*;
use bevy_third_person_camera::*;
use bevy_rapier3d::prelude::*;
use crate::menu::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), spawn_player)
            .add_systems(Update, (player_movement).run_if(in_state(GameState::InGame)));

    }
}

#[derive(Component)]
struct Player;


#[derive(Component)]
struct Speed(f32);

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_q : Query<(&mut Transform, &Speed, &mut KinematicCharacterController), With<Player>>,
    cam_q : Query<&Transform , (With<Camera3d>, Without<Player>)>,
) {
    if let Ok((mut player_transform, player_speed, mut controller)) = player_q.get_single_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            direction += *cam.forward();
        }
        
        if keys.pressed(KeyCode::KeyS) {
            direction += *cam.back();
        }

        if keys.pressed(KeyCode::KeyA) {
            direction += *cam.left();
        }

        if keys.pressed(KeyCode::KeyD) {
            direction += *cam.right();
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        
        controller.translation = Some(movement);


        //rotate player to face direction of movement
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y)
        }
 
    }
}



fn spawn_player(
    mut commands: Commands, 
    assets : Res<AssetServer>,
) {
    let flashlight = (
        SpotLightBundle{
            spot_light : SpotLight {
            color : Color::linear_rgba(1.0, 1.0, 0.47, 1.0),
            range:100.0,
            intensity: 1000000.0,
            outer_angle: 0.8,
            inner_angle: 0.4,
            shadows_enabled:true,
            ..Default::default()
            },
            transform: Transform::from_xyz(0.0,1.0, -0.5),
            ..Default::default()
        }, 
        Name::new("Flashlight"));

    let player = (
    SceneBundle {
        scene : assets.load("Player.gltf#Scene0"),
        transform : Transform::from_xyz(0.5, 0.5, 0.5),
        ..default()
    },
    Speed(5.0),
    Player,
    ThirdPersonCameraTarget,
    Name::new("Player"),
    Collider::cuboid(0.5, 1.0, 0.5),
    RigidBody::KinematicPositionBased
);

    commands.spawn(player)
    .insert(KinematicCharacterController{
        ..KinematicCharacterController::default()
    })
    .with_children(|parent| {
        parent.spawn(flashlight);
    });

}