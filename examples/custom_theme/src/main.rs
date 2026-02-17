use bevy::prelude::*;
use bevy_launchpad::core::boot::metadata::AppMetadata;
use bevy_launchpad::prelude::*;
use bevy_launchpad::ui::theme::fonts::ThemeFonts;

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
        },
        fonts: ThemeFonts::noto_sans(),
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(register_embedded_assets)
        // Use builder to inject theme
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_metadata(AppMetadata {
                    name: "custom_theme_example".into(),
                    title: "Custom Theme Example".into(),
                    ..default()
                })
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
