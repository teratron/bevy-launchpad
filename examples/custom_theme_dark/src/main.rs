use bevy::prelude::*;
use bevy_launchpad::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<AppState>::builder()
                // Hot-pink cyberpunk palette
                .with_theme(ThemeConfig {
                    colors: ThemeColors {
                        primary: Color::srgb(0.90, 0.10, 0.45),
                        secondary: Color::srgb(0.60, 0.05, 0.28),
                        background: Color::srgb(0.05, 0.02, 0.08),
                        text: Color::srgb(0.95, 0.88, 0.95),
                    },
                    // Noto Sans so Cyrillic menus look good
                    fonts: ThemeFonts::noto_sans(),
                    spacing: ThemeSpacing {
                        padding: 20.0,
                        margin: 12.0,
                        corner_radius: 8.0,
                    },
                })
                // Skip splash for fast dev iteration
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
    info!("Custom dark theme running");
}
