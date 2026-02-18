use bevy::prelude::*;
use bevy_launchpad::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            custom_layer: |_| Some(Box::new(bevy_launchpad::logging::layer::AsyncFileLayer)),
            ..default()
        }))
        // Everything out of the box:
        //   boot → embedded splash ("Powered by Bevy Launchpad") → menu → Playing
        //   theme:  dark
        //   font:   Bevy built-in FiraMono (zero cost)
        //   locale: en-US
        .add_plugins(LaunchpadPlugin::<AppState>::default())
        .add_systems(OnEnter(AppState::Playing), setup)
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
