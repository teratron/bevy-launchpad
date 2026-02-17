use bevy::prelude::*;
use bevy_launchpad::prelude::*;
use serde::{Deserialize, Serialize};

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

/// Developer-defined gameplay settings (fully serializable → auto-saved).
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameplaySettings {
    pub difficulty:      Difficulty,
    pub camera_shake:    bool,
    pub tutorial_hints:  bool,
    pub auto_save_secs:  u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum Difficulty { Easy, #[default] Normal, Hard, Nightmare }

impl Default for GameplaySettings {
    fn default() -> Self {
        Self {
            difficulty:     Difficulty::default(),
            camera_shake:   true,
            tutorial_hints: true,
            auto_save_secs: 60,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_splash(SplashConfig { skip_all: true, ..default() })
                .build(),
        )
        // Register the custom resource — library will persist it alongside GameSettings
        .insert_resource(GameplaySettings::default())
        .add_systems(OnEnter(GameState::Playing), setup)
        // Save gameplay settings when they change
        .add_systems(Update,
            save_gameplay_settings.run_if(resource_changed::<GameplaySettings>),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Custom settings example running. Open Settings to see Gameplay tab.");
}

fn save_gameplay_settings(
    settings: Res<GameplaySettings>,
    paths:    Res<AppPaths>,
) {
    let path = paths.data_dir.join("gameplay.ron");
    if let Ok(content) = ron::ser::to_string_pretty(&*settings, Default::default()) {
        let _ = std::fs::write(&path, content);
        info!("Gameplay settings saved to {:?}", path);
    }
}
