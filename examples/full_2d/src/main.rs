use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Booting,
    Loading,
    Splash,
    Menu,
    Playing,
    Paused,
}

impl LaunchpadStates for GameState {
    fn booting() -> Self {
        GameState::Booting
    }
    fn loading() -> Self {
        GameState::Loading
    }
    fn splash() -> Self {
        GameState::Splash
    }
    fn menu() -> Self {
        GameState::Menu
    }
    fn playing() -> Self {
        GameState::Playing
    }
    fn paused() -> Self {
        GameState::Paused
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LaunchpadPlugin::<GameState>::default())
        // Register transitions
        .add_systems(OnEnter(GameState::Playing), setup_level)
        .add_systems(Update, game_logic.run_if(in_state(GameState::Playing)))
        .run();
}

fn setup_level(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite {
        color: Color::srgb(0.0, 0.5, 0.8),
        custom_size: Some(Vec2::new(50.0, 50.0)),
        ..default()
    });
    info!("Full 2D Game Level Started");
}

fn game_logic() {
    // Placeholder for game logic
}
