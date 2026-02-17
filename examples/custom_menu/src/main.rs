use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Booting, Loading, Splash, Menu,
    // Developer-specific states
    LevelSelect,
    Credits,
    Playing, Paused,
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
                .with_splash(SplashConfig { skip_all: true, ..default() })
                .with_main_menu(MainMenuConfig {
                    title: "Epic Quest".into(),
                    buttons: vec![
                        MenuButton::Play,
                        MenuButton::Custom {
                            label:             "Level Select".into(),
                            target_state_name: "LevelSelect".into(),
                        },
                        MenuButton::Custom {
                            label:             "Credits".into(),
                            target_state_name: "Credits".into(),
                        },
                        MenuButton::Settings,
                        MenuButton::Exit,
                    ],
                })
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing),     setup_game)
        .add_systems(OnEnter(GameState::LevelSelect), setup_level_select)
        .add_systems(OnEnter(GameState::Credits),     setup_credits)
        .run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Game started");
}

fn setup_level_select(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Level Select screen — build your own UI here");
}

fn setup_credits(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Credits screen — build your own UI here");
}
