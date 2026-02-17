use bevy::prelude::*;
use bevy_launchpad::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<AppState>::builder()
                .with_splash(SplashConfig {
                    screens: vec![],              // no custom screens
                    show_default_branding: false, // no library branding
                    skip_all: false,              // skip_all not needed when both above are off
                })
                .build(),
        )
        .add_systems(OnEnter(AppState::Playing), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
