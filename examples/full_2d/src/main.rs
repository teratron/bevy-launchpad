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
                    name:        "full_2d_game".into(),
                    title:       "My 2D Adventure".into(),
                    version:     "0.1.0".into(),
                    description: "A 2D platformer built with bevy_launchpad".into(),
                })
                .with_splash(SplashConfig {
                    screens: vec![
                        // Studio logo — skippable after 2 s
                        SplashScreenConfig::studio("branding/studio_logo.png"),
                        // Game logo — custom timing and fade
                        SplashScreenConfig::engine("branding/game_logo.png")
                            .with_duration(2.0, 3.5)
                            .with_fade(0.6, 0.6)
                            .on_background(Color::srgb(0.05, 0.05, 0.1)),
                    ],
                    show_default_branding: false,
                    ..default()
                })
                // Noto Sans supports Cyrillic (needed for ru-RU)
                .with_fonts(ThemeFonts::noto_sans())
                .with_theme(ThemeConfig::dark())
                .with_main_menu(MainMenuConfig {
                    title: "My 2D Adventure".into(),
                    buttons: vec![
                        MenuButton::Play,
                        MenuButton::Custom {
                            label:             "Credits".into(),
                            target_state_name: "Credits".into(),
                        },
                        MenuButton::Settings,
                        MenuButton::Exit,
                    ],
                })
                .with_locale("en-US")   // default locale; player can switch in settings
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), setup_level)
        .add_systems(Update, (
            move_player,
            check_pause,
        ).run_if(in_state(GameState::Playing)))
        .run();
}

#[derive(Component)]
struct Player;

fn setup_level(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            color:       Color::srgb(0.2, 0.6, 1.0),
            custom_size: Some(Vec2::new(48.0, 48.0)),
            ..default()
        },
        Player,
    ));
    info!("Full 2D level loaded");
}

fn move_player(
    mut q: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let speed = 200.0;
    for mut t in &mut q {
        let mut dir = Vec2::ZERO;
        if keys.pressed(KeyCode::ArrowRight) { dir.x += 1.0; }
        if keys.pressed(KeyCode::ArrowLeft)  { dir.x -= 1.0; }
        if keys.pressed(KeyCode::ArrowUp)    { dir.y += 1.0; }
        if keys.pressed(KeyCode::ArrowDown)  { dir.y -= 1.0; }
        t.translation += dir.extend(0.0) * speed * time.delta_secs();
    }
}

fn check_pause(
    keys:       Res<ButtonInput<KeyCode>>,
    state:      Res<State<GameState>>,
    mut next:   ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if *state.get() == GameState::Playing {
            next.set(GameState::Paused);
        }
    }
}
