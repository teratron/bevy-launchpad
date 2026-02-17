use bevy::prelude::*;
use bevy_launchpad::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<AppState>::builder()
                .with_theme(ThemeConfig::light())
                // ThemeFonts not set → Bevy built-in FiraMono (zero binary cost)
                .with_splash(SplashConfig {
                    screens: vec![
                        // Color-only splash — white fade-in, no image needed
                        SplashScreenConfig {
                            source:       SplashSource::ColorOnly,
                            min_duration: 0.5,
                            max_duration: Some(1.0),
                            skip:         SkipTrigger::AnyInput,
                            fade_in:      0.3,
                            fade_out:     0.3,
                            background:   Color::srgb(0.95, 0.95, 0.95),
                        },
                    ],
                    show_default_branding: false,
                    ..default()
                })
                .build(),
        )
        .add_systems(OnEnter(AppState::Playing), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
