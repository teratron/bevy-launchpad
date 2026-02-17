use bevy::prelude::*;
use bevy_launchpad::prelude::*;

fn make_color_screen(color: Color, duration: f32) -> SplashScreenConfig {
    SplashScreenConfig {
        source: SplashSource::ColorOnly,
        min_duration: duration * 0.5,
        max_duration: Some(duration),
        skip: SkipTrigger::AnyInput,
        fade_in: duration * 0.3,
        fade_out: duration * 0.3,
        background: color,
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<AppState>::builder()
                .with_splash(SplashConfig {
                    screens: vec![
                        make_color_screen(Color::BLACK, 1.5),                 // darkness
                        make_color_screen(Color::srgb(0.0, 0.05, 0.15), 2.0), // deep blue
                        make_color_screen(Color::srgb(0.0, 0.35, 0.65), 1.5), // ocean blue
                    ],
                    show_default_branding: false,
                    ..default()
                })
                .with_theme(ThemeConfig {
                    colors: ThemeColors {
                        primary: Color::srgb(0.0, 0.6, 1.0),
                        secondary: Color::srgb(0.0, 0.4, 0.8),
                        background: Color::srgb(0.0, 0.05, 0.15),
                        text: Color::WHITE,
                    },
                    ..default()
                })
                .build(),
        )
        .add_systems(OnEnter(AppState::Playing), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Color splash example running");
}
