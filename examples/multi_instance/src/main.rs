use bevy::prelude::*;
use bevy_launchpad::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<AppState>::builder()
                // Disable single-instance protection
                .allow_multiple_instances(true)
                .with_splash(SplashConfig {
                    skip_all: true,
                    ..default()
                })
                .build(),
        )
        .add_systems(OnEnter(AppState::Playing), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Multi-instance: you can run several copies of this game");
}
