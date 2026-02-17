use bevy::prelude::*;
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
        // Everything out of the box:
        //   boot → embedded splash ("Powered by Bevy Launchpad") → menu → Playing
        //   theme:  dark
        //   font:   Bevy built-in FiraMono (zero cost)
        //   locale: en-US
        .add_plugins(LaunchpadPlugin::<GameState>::default())
        .add_systems(OnEnter(GameState::Playing), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite {
        color: Color::srgb(0.0, 1.0, 0.0),
        custom_size: Some(Vec2::splat(100.0)),
        ..default()
    });
    info!("Minimal 2D started");
}
