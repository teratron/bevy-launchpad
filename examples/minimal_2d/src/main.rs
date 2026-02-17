use bevy::prelude::*;
use bevy_launchpad::core::boot::metadata::AppMetadata;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Booting,
    Loading, // Added missing required state
    Splash,  // Added missing required state
    Menu,
    Playing,
    Paused, // Added missing required state
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
        // Required for embedded assets (fonts/branding)
        .add_plugins(register_embedded_assets)
        // Add the LaunchpadPlugin generic over our GameState
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_metadata(AppMetadata {
                    name: "minimal_2d".into(),
                    title: "Minimal 2D Game".into(),
                    ..default()
                })
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .run();
}

fn setup_game(mut commands: Commands) {
    // Spawn 2D camera
    commands.spawn(Camera2d);

    // Simple placeholder for the game
    commands.spawn(Sprite {
        color: Color::srgb(0.0, 1.0, 0.0),
        custom_size: Some(Vec2::new(100.0, 100.0)),
        ..default()
    });

    info!("Minimal 2D Game Started!");
}
