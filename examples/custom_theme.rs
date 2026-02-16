use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Booting,
    Menu,
    Playing,
}

fn main() {
    // Define a custom theme
    let my_theme = ThemeConfig {
        colors: ThemeColors {
            primary: Color::srgb(0.8, 0.2, 0.4), // Pink/Red primary
            secondary: Color::srgb(0.6, 0.1, 0.3),
            background: Color::srgb(0.1, 0.05, 0.1), // Dark purple bg
            text: Color::srgb(0.9, 0.8, 0.8),
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
