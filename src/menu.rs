use bevy::prelude::*;

// States
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}

// Plugin
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::MainMenu), (setup_menu, setup_ui_camera))
            .add_systems(Update, (button_system).run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu);
    }
}

// Components
#[derive(Component)]
struct MainMenu;

#[derive(Component)]
enum MenuButton {
    Play,
    Exit,
}

fn setup_ui_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Systems
fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgb(0.1, 0.1, 0.1).into(),
            ..default()
        }, MainMenu))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "My Maze Game",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 64.0,
                    color: Color::WHITE,
                },
            ));

            // Play button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                    ..default()
                },
                MenuButton::Play,
            )).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Play",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ));
            });

            // Exit button
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                    ..default()
                },
                MenuButton::Exit,
            )).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Exit",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ));
            });
        });
}

fn button_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &MenuButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<bevy::app::AppExit>,
) {
    for (interaction, button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match button {
                    MenuButton::Play => {
                        next_state.set(GameState::InGame);
                    }
                    MenuButton::Exit => {
                        app_exit_events.send(bevy::app::AppExit::Success);
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

fn cleanup_menu(
    mut commands: Commands, 
    menu_query: Query<Entity, With<MainMenu>>,
    camera_query: Query<Entity, With<Camera2d>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for camera in camera_query.iter() {
        commands.entity(camera).despawn_recursive();
    }
}