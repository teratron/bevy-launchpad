use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Booting, Loading, Splash, Menu, Playing, Paused,
}

impl LaunchpadStates for GameState {
    fn booting()  -> Self { Self::Booting  }
    fn loading()  -> Self { Self::Loading  }
    fn splash()   -> Self { Self::Splash   }
    fn menu()     -> Self { Self::Menu     }
    fn playing()  -> Self { Self::Playing  }
    fn paused()   -> Self { Self::Paused   }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                // Library embedded splash is used (no File source)
                .with_splash(SplashConfig {
                    screens: vec![
                        SplashScreenConfig {
                            source:       SplashSource::Embedded, // embedded PNG
                            min_duration: 1.5,
                            max_duration: Some(2.5),
                            skip:         SkipTrigger::AnyInput,
                            fade_in:      0.3,
                            fade_out:     0.3,
                            background:   Color::BLACK,
                        },
                    ],
                    show_default_branding: false,
                    ..default()
                })
                // Noto Sans is embedded via embedded_assets feature
                .with_fonts(ThemeFonts::noto_sans())
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Running with zero external assets");
}
