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
                // Option A: explicit folder name
                .with_assets_root(AssetsRootStrategy::Explicit(
                    "game_data/".into()  // instead of default "assets/"
                ))
                // Option B: env-var override (highest priority, no rebuild needed)
                // BEVY_LAUNCHPAD_ASSETS_ROOT=/opt/mygame/data cargo run
                //
                // Option C: SearchPaths (try multiple, first found wins)
                // .with_assets_root(AssetsRootStrategy::SearchPaths(vec![
                //     "/opt/mygame/data".into(),
                //     "game_data/".into(),
                //     "assets/".into(),
                // ]))
                .with_splash(SplashConfig {
                    screens: vec![
                        // Path is relative to the custom root ("game_data/")
                        SplashScreenConfig::studio("branding/logo.png"),
                    ],
                    show_default_branding: false,
                    ..default()
                })
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
