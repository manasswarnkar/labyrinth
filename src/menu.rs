use bevy::prelude::*;

// States
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    GenerateMenu,
    InGame,
}

// Plugin
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(UiCameraExists(false))
            .add_systems(Startup, setup_ui_camera)
            .add_systems(OnEnter(GameState::MainMenu), setup_menu)
            .add_systems(OnEnter(GameState::GenerateMenu), (setup_gen_menu, setup_ui_camera))
            .add_systems(
                Update,
                (
                    button_system.run_if(in_state(GameState::MainMenu)),
                    generate_button_system.run_if(in_state(GameState::GenerateMenu)),
                ),
            )
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu)
            .add_systems(OnExit(GameState::GenerateMenu), cleanup_menu)
            .add_systems(OnExit(GameState::InGame), setup_ui_camera)
            .add_systems(OnEnter(GameState::InGame), remove_ui_camera);
    }
}

// Components
#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct GenerateMenu;

#[derive(Component)]
struct UiCamera;

#[derive(Resource)]
pub struct UiCameraExists(pub bool);

#[derive(Component, Debug)]
enum MenuButton {
    Play,
    Generate,
    Exit,
    Option1,
    Option2,
    Option3,
}
fn setup_ui_camera(
    mut commands: Commands,
    mut ui_camera_exists: ResMut<UiCameraExists>
) {
    if !ui_camera_exists.0 {
        commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    order: 1, // Set a specific order for the UI camera
                    ..default()
                },
                ..default()
            },
            UiCamera
        ));
        ui_camera_exists.0 = true;
    }
}

fn remove_ui_camera(
    mut commands: Commands,
    mut ui_camera_exists: ResMut<UiCameraExists>,
    query: Query<Entity, With<UiCamera>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    ui_camera_exists.0 = false;
}

// Systems
fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
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
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "Labyrinth",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 64.0,
                    color: Color::WHITE,
                },
            ));
            

            //Play button
            spawn_button(parent, "Play", MenuButton::Play, &asset_server);

            // Generate button
            spawn_button(parent, "Generate", MenuButton::Generate, &asset_server);

            // Exit button
            spawn_button(parent, "Exit", MenuButton::Exit, &asset_server);
        });
}

fn setup_gen_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
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
            },
            GenerateMenu,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "Generate Options",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 64.0,
                    color: Color::WHITE,
                },
            ));

            // Generate Option 1
            spawn_button(parent, "Option 1", MenuButton::Option1, &asset_server);

            // Generate Option 2
            spawn_button(parent, "Option 2", MenuButton::Option2, &asset_server);

            // Generate Option 3
            spawn_button(parent, "Option 3", MenuButton::Option3, &asset_server);
        });
}

fn spawn_button(
    parent: &mut ChildBuilder,
    text: &str,
    button_type: MenuButton,
    asset_server: &Res<AssetServer>,
) {
    parent
        .spawn((
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
            button_type,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));
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
            Interaction::Pressed => match button {
                MenuButton::Play => {
                    next_state.set(GameState::InGame);
                }
                MenuButton::Exit => {
                    app_exit_events.send(bevy::app::AppExit::Success);
                }
                MenuButton::Generate => {
                    next_state.set(GameState::GenerateMenu);
                }
                _ => {}
            },
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

fn generate_button_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &MenuButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match button {
                    MenuButton::Option1 | MenuButton::Option2 | MenuButton::Option3 => {
                        println!("Generate option selected: {:?}", button);
                        // Here you can add logic to handle the selected generation option
                        next_state.set(GameState::MainMenu);
                    }
                    _ => {}
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
    menu_query: Query<Entity, Or<(With<MainMenu>, With<GenerateMenu>)>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
