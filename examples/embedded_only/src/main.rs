use bevy::prelude::*;
use bevy_launchpad::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<AppState>::builder()
                // Library embedded splash is used (no File source)
                .with_splash(SplashConfig {
                    screens: vec![SplashScreenConfig {
                        source: SplashSource::Embedded, // embedded PNG
                        min_duration: 1.5,
                        max_duration: Some(2.5),
                        skip: SkipTrigger::AnyInput,
                        fade_in: 0.3,
                        fade_out: 0.3,
                        background: Color::BLACK,
                    }],
                    show_default_branding: false,
                    ..default()
                })
                // Noto Sans is embedded via embedded_assets feature
                .with_fonts(ThemeFonts::noto_sans())
                .build(),
        )
        .add_systems(OnEnter(AppState::Playing), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Running with zero external assets");
}
