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
    // Define a custom theme
    let my_theme = ThemeConfig {
        colors: ThemeColors {
            primary: Color::srgb(0.8, 0.2, 0.4), // Pink/Red primary
            secondary: Color::srgb(0.6, 0.1, 0.3),
            background: Color::srgb(0.1, 0.05, 0.1), // Dark purple bg
            text: Color::srgb(0.9, 0.8, 0.8),
            accent: Color::srgb(1.0, 0.6, 0.2),
        },
        // ... use default fonts/spacing
    };

    App::new()
        .add_plugins(DefaultPlugins)
        // Use builder to inject theme (or resource insertion)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_theme(my_theme)
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Custom Theme Game Started!");
}
