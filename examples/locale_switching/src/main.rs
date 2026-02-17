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
                // Noto Sans required for Cyrillic characters
                .with_fonts(ThemeFonts::noto_sans())
                .with_locale("en-US")
                .with_splash(SplashConfig { skip_all: true, ..default() })
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(Update, switch_locale.run_if(in_state(GameState::Playing)))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Press 1 = English, Press 2 = Русский, Press 3 = 日本語");
}

/// Switch language by pressing 1 / 2 / 3.
/// bevy_launchpad detects the Language change and reloads the Fluent bundle.
fn switch_locale(
    keys: Res<ButtonInput<KeyCode>>,
    mut lang: ResMut<Language>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        *lang = Language::En;
        info!("Switched to English");
    }
    if keys.just_pressed(KeyCode::Digit2) {
        *lang = Language::Ru;
        info!("Переключено на русский");
    }
    if keys.just_pressed(KeyCode::Digit3) {
        *lang = Language::En; // Japanese not in enum, falling back to English for demo
        info!("Falling back to English (Japanese variant missing in demo)");
    }
}
