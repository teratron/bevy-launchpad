use bevy::prelude::*;
use bevy_launchpad::prelude::*;  // only core types exported without "ui" feature

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
        .add_plugins(MinimalPlugins)  // no window, no renderer
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_metadata(AppMetadata {
                    name:  "game_server".into(),
                    title: "Game Server".into(),
                    ..default()
                })
                .with_splash(SplashConfig { skip_all: true, ..default() })
                .allow_multiple_instances(true)
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), server_start)
        .add_systems(Update, server_tick.run_if(in_state(GameState::Playing)))
        .run();
}

fn server_start() {
    info!("Server started — listening for connections");
}

fn server_tick(time: Res<Time>) {
    // Runs every frame — no renderer overhead
    if (time.elapsed_secs() as u32) % 5 == 0 {
        // info!("Server tick: {:.0}s", time.elapsed_secs());
    }
}
