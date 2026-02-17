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
                .with_metadata(AppMetadata {
                    name:  "mod_support_demo".into(),
                    title: "Mod Support Demo".into(),
                    ..default()
                })
                // SearchPaths: base game → DLC pack → user mods
                // First directory that contains the requested file wins.
                .with_assets_root(AssetsRootStrategy::SearchPaths(vec![
                    "assets/".into(),               // base game (always present)
                    "dlc/space_pack/assets/".into(), // DLC (optional)
                    "mods/".into(),                  // user mods (optional)
                ]))
                .with_splash(SplashConfig {
                    screens: vec![
                        // If mods/branding/studio.png exists → override
                        // Otherwise falls back to assets/branding/studio.png
                        SplashScreenConfig::studio("branding/studio.png"),
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
    info!("Mod support demo running");
}
