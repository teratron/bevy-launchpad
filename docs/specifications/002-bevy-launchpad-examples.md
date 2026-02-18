# bevy-launchpad — Complete Examples Reference

> **Stack:** Rust 1.93 · Bevy 0.18 · bevy_launchpad 0.1  
> All examples share the same `LaunchpadStates` impl shown in §0.
> Each section lists the required `assets/` files — mark ✋ means you must add the file manually.

---

## §0. Shared boilerplate (used in every example)

```rust
// Shared GameState enum — paste into each example's main.rs
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
```

---

## Example index

| # | Name | Folder | Key concept |
|---|---|---|---|
| 01 | Minimal 2D | `minimal_2d/` | Zero config, embedded branding |
| 02 | Minimal 3D | `minimal_3d/` | Zero config, custom metadata |
| 03 | Full 2D — Indie | `full_2d/` | Studio splash, Noto Sans, custom menu |
| 04 | Full 3D — AAA | `full_3d/` | 4-screen splash, legal, custom font |
| 05 | Custom Theme Dark | `custom_theme_dark/` | Custom colors, skip splash |
| 06 | Light Theme | `custom_theme_light/` | Light preset, Bevy font |
| 07 | Color-only Splash | `color_splash/` | No images — gradient background |
| 08 | No Splash | `no_splash/` | Straight to menu |
| 09 | Locale Switching | `locale_switching/` | Runtime language change |
| 10 | Custom Settings | `custom_settings/` | Developer-defined settings tab |
| 11 | Pause Menu | `pause_menu/` | In-game pause with resume |
| 12 | Custom Menu Buttons | `custom_menu/` | Credits, Level Select buttons |
| 13 | Mod Support | `mod_support/` | SearchPaths for DLC / mods |
| 14 | Embedded Assets Only | `embedded_only/` | Ship without developer assets/ |
| 15 | CLI / Dev Mode | `dev_mode/` | `--skip-splash --state Playing` |
| 16 | Multiple Instances | `multi_instance/` | Allow parallel launches |
| 17 | Minimal No-UI | `no_ui/` | Core only, no menus (server/tool) |
| 18 | Custom Assets Root | `custom_assets_root/` | Non-standard asset folder name |

---

## 01 — Minimal 2D

**Concept:** absolute minimum. Zero config. Library handles everything.

### `examples/minimal_2d/Cargo.toml`

```toml
[package]
name    = "example_minimal_2d"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
bevy          = { version = "0.18" }
bevy-launchpad = { path = "../..", features = ["2d"] }
```

### `examples/minimal_2d/src/main.rs`

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

// §0 GameState + LaunchpadStates impl here

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Everything out of the box:
        //   boot → embedded splash ("Powered by Bevy Launchpad") → menu → Playing
        //   theme:  dark
        //   font:   Bevy built-in FiraMono (zero cost)
        //   locale: en-US
        .add_plugins(LaunchpadPlugin::<GameState>::default())
        .add_systems(OnEnter(GameState::Playing), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite {
        color:       Color::srgb(0.0, 1.0, 0.0),
        custom_size: Some(Vec2::splat(100.0)),
        ..default()
    });
    info!("Minimal 2D started");
}
```

### `examples/minimal_2d/assets/`

```
assets/     ← empty folder (Bevy requires it to exist)
```

> No manual assets needed — the embedded splash is used automatically.

---

## 02 — Minimal 3D

**Concept:** zero config 3D with custom window title via `AppMetadata`.

### `examples/minimal_3d/Cargo.toml`

```toml
[package]
name    = "example_minimal_3d"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
bevy          = { version = "0.18" }
bevy-launchpad = { path = "../..", features = ["3d"] }
```

### `examples/minimal_3d/src/main.rs`

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

// §0 here

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_metadata(AppMetadata {
                    name:    "minimal_3d".into(),   // data dir: ~/.local/share/minimal_3d/
                    title:   "Minimal 3D Demo".into(), // window title
                    version: "0.1.0".into(),
                    ..default()
                })
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), setup)
        .run();
}

fn setup(
    mut commands:  Commands,
    mut meshes:    ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    commands.spawn((
        PointLight { shadows_enabled: true, ..default() },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    info!("Minimal 3D started");
}
```

### `examples/minimal_3d/assets/`

```
assets/     ← empty
```

---

## 03 — Full 2D (Indie game pattern)

**Concept:** two splash screens, Noto Sans font, custom menu with Credits button,
Russian + English locale.

### `examples/full_2d/Cargo.toml`

```toml
[package]
name    = "example_full_2d"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
bevy          = { version = "0.18" }
bevy-launchpad = { path = "../..", features = ["ui", "locale", "2d"] }
```

### `examples/full_2d/src/main.rs`

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

// §0 here

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
```

### `examples/full_2d/assets/`

```
assets/
├── branding/
│   ├── studio_logo.png   ✋ add manually  (recommended: 1920×1080, PNG)
│   └── game_logo.png     ✋ add manually  (recommended: 1920×1080, PNG)
├── locales/
│   ├── en-US/
│   │   └── main.ftl      ← content below
│   └── ru-RU/
│       └── main.ftl      ← content below
└── audio/                ← optional, for game sounds
```

**`assets/locales/en-US/main.ftl`** (create this file):

```ftl
menu-play     = Play
menu-credits  = Credits
menu-settings = Settings
menu-exit     = Exit

settings-title   = Settings
settings-graphics = Graphics
settings-audio   = Audio
settings-controls = Controls
```

**`assets/locales/ru-RU/main.ftl`** (create this file):

```ftl
menu-play     = Играть
menu-credits  = Авторы
menu-settings = Настройки
menu-exit     = Выйти

settings-title    = Настройки
settings-graphics = Графика
settings-audio    = Звук
settings-controls = Управление
```

---

## 04 — Full 3D (AAA pattern)

**Concept:** four-screen splash (legal → publisher → developer → engine),
custom font from assets/, sci-fi color theme, rotating environment.

### `examples/full_3d/Cargo.toml`

```toml
[package]
name    = "example_full_3d"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
bevy          = { version = "0.18" }
bevy-launchpad = { path = "../..", features = ["ui", "locale", "3d"] }
```

### `examples/full_3d/src/main.rs`

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

// §0 here

#[derive(Component)]
struct RotatingCube;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_metadata(AppMetadata {
                    name:    "galaxy_quest".into(),
                    title:   "Galaxy Quest 3D".into(),
                    version: "0.1.0".into(),
                    ..default()
                })
                // AAA four-screen splash
                .with_splash(SplashConfig {
                    screens: vec![
                        // 1. Legal / age-rating — 5 s, cannot skip
                        SplashScreenConfig::legal("branding/legal.png")
                            .on_background(Color::WHITE),

                        // 2. Publisher
                        SplashScreenConfig::studio("branding/publisher.png")
                            .with_duration(2.5, 4.0),

                        // 3. Developer
                        SplashScreenConfig::studio("branding/developer.png")
                            .with_duration(2.0, 3.5)
                            .with_fade(0.5, 0.5),

                        // 4. Engine — skippable, dark background
                        SplashScreenConfig::engine("branding/engine.png")
                            .with_duration(1.5, 3.0)
                            .with_fade(0.4, 0.4)
                            .on_background(Color::srgb(0.02, 0.02, 0.05)),
                    ],
                    show_default_branding: false,
                    ..default()
                })
                // Custom game font (sci-fi feel)
                .with_fonts(ThemeFonts::from_project(
                    "fonts/Orbitron-Regular.ttf",
                    "fonts/Orbitron-Bold.ttf",
                ))
                // Custom sci-fi color scheme
                .with_theme(ThemeConfig {
                    colors: ThemeColors {
                        primary:    Color::srgb(0.0,  0.78, 1.0),
                        secondary:  Color::srgb(0.0,  0.45, 0.78),
                        background: Color::srgb(0.02, 0.02, 0.06),
                        text:       Color::srgb(0.9,  0.95, 1.0),
                    },
                    fonts:   ThemeFonts::from_project(
                        "fonts/Orbitron-Regular.ttf",
                        "fonts/Orbitron-Bold.ttf",
                    ),
                    spacing: ThemeSpacing::default(),
                })
                .with_locale("en-US")
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), setup_3d)
        .add_systems(Update, (
            rotate_cube,
            check_pause,
        ).run_if(in_state(GameState::Playing)))
        .run();
}

fn setup_3d(
    mut commands:  Commands,
    mut meshes:    ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // Rotating cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color:        Color::srgb(0.0, 0.78, 1.0),
            emissive:          LinearRgba::new(0.0, 0.3, 0.5, 1.0),
            metallic:          0.8,
            perceptual_roughness: 0.2,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
        RotatingCube,
    ));
    // Lights
    commands.spawn((
        PointLight { intensity: 2000.0, shadows_enabled: true, ..default() },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    commands.spawn((
        PointLight {
            intensity: 800.0,
            color: Color::srgb(0.0, 0.5, 1.0),
            ..default()
        },
        Transform::from_xyz(-4.0, 2.0, -4.0),
    ));
    info!("Galaxy Quest level loaded");
}

fn rotate_cube(
    mut q:    Query<&mut Transform, With<RotatingCube>>,
    time: Res<Time>,
) {
    for mut t in &mut q {
        t.rotate_y(time.delta_secs() * 0.8);
        t.rotate_x(time.delta_secs() * 0.3);
    }
}

fn check_pause(
    keys:    Res<ButtonInput<KeyCode>>,
    state:   Res<State<GameState>>,
    mut nxt: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if *state.get() == GameState::Playing {
            nxt.set(GameState::Paused);
        }
    }
}
```

### `examples/full_3d/assets/`

```
assets/
├── branding/
│   ├── legal.png       ✋  White background, age-rating text (e.g. 1920×1080)
│   ├── publisher.png   ✋  Publisher logo on black (e.g. 1920×1080)
│   ├── developer.png   ✋  Developer logo on black (e.g. 1920×1080)
│   └── engine.png      ✋  "Made with Bevy" or custom engine logo
└── fonts/
    ├── Orbitron-Regular.ttf  ✋  https://fonts.google.com/specimen/Orbitron
    └── Orbitron-Bold.ttf     ✋  same download, Bold weight
```

---

## 05 — Custom Theme Dark

**Concept:** fully custom dark color scheme, skip splash (fast iteration mode).

### `examples/custom_theme_dark/src/main.rs`

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

// §0 here

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                // Hot-pink cyberpunk palette
                .with_theme(ThemeConfig {
                    colors: ThemeColors {
                        primary:    Color::srgb(0.90, 0.10, 0.45),
                        secondary:  Color::srgb(0.60, 0.05, 0.28),
                        background: Color::srgb(0.05, 0.02, 0.08),
                        text:       Color::srgb(0.95, 0.88, 0.95),
                    },
                    // Noto Sans so Cyrillic menus look good
                    fonts:   ThemeFonts::noto_sans(),
                    spacing: ThemeSpacing {
                        padding:       20.0,
                        margin:        12.0,
                        corner_radius:  8.0,
                    },
                })
                // Skip splash for fast dev iteration
                .with_splash(SplashConfig {
                    skip_all: true,
                    ..default()
                })
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Custom dark theme running");
}
```

### `examples/custom_theme_dark/assets/`

```
assets/     ← empty
```

---

## 06 — Light Theme

**Concept:** built-in light preset with Bevy default font (zero extra assets).

### `examples/custom_theme_light/src/main.rs`

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

// §0 here

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_theme(ThemeConfig::light())
                // ThemeFonts not set → Bevy built-in FiraMono (zero binary cost)
                .with_splash(SplashConfig {
                    screens: vec![
                        // Color-only splash — white fade-in, no image needed
                        SplashScreenConfig {
                            source:       SplashSource::ColorOnly,
                            min_duration: 0.5,
                            max_duration: Some(1.0),
                            skip:         SkipTrigger::AnyInput,
                            fade_in:      0.3,
                            fade_out:     0.3,
                            background:   Color::srgb(0.95, 0.95, 0.95),
                        },
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
```

### `examples/custom_theme_light/assets/`

```
assets/     ← empty
```

---

## 07 — Color-only Splash (no images)

**Concept:** three atmospheric color screens — no image files required.
Useful for prototyping or minimalist aesthetic.

### `examples/color_splash/src/main.rs`

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

// §0 here

fn make_color_screen(color: Color, duration: f32) -> SplashScreenConfig {
    SplashScreenConfig {
        source:       SplashSource::ColorOnly,
        min_duration: duration * 0.5,
        max_duration: Some(duration),
        skip:         SkipTrigger::AnyInput,
        fade_in:      duration * 0.3,
        fade_out:     duration * 0.3,
        background:   color,
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_splash(SplashConfig {
                    screens: vec![
                        make_color_screen(Color::BLACK,                    1.5), // darkness
                        make_color_screen(Color::srgb(0.0, 0.05, 0.15),   2.0), // deep blue
                        make_color_screen(Color::srgb(0.0, 0.35, 0.65),   1.5), // ocean blue
                    ],
                    show_default_branding: false,
                    ..default()
                })
                .with_theme(ThemeConfig {
                    colors: ThemeColors {
                        primary:    Color::srgb(0.0, 0.6, 1.0),
                        secondary:  Color::srgb(0.0, 0.4, 0.8),
                        background: Color::srgb(0.0, 0.05, 0.15),
                        text:       Color::WHITE,
                    },
                    ..default()
                })
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Color splash example running");
}
```

### `examples/color_splash/assets/`

```
assets/     ← empty (no images needed)
```

---

## 08 — No Splash (straight to menu)

**Concept:** disable both custom and default splash — game goes directly to menu.
Ideal for tools, editors, or games targeting fast startup time.

### `examples/no_splash/src/main.rs`

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

// §0 here

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_splash(SplashConfig {
                    screens:               vec![],   // no custom screens
                    show_default_branding: false,    // no library branding
                    skip_all:              false,    // skip_all not needed when both above are off
                })
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
```

### `examples/no_splash/assets/`

```
assets/     ← empty
```

---

## 09 — Locale Switching at Runtime

**Concept:** start in English, switch to Russian in the menu settings.
Shows how `Language` resource drives Fluent bundle reloading.

### `examples/locale_switching/src/main.rs`

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

// §0 here

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
        *lang = Language("en-US".into());
        info!("Switched to English");
    }
    if keys.just_pressed(KeyCode::Digit2) {
        *lang = Language("ru-RU".into());
        info!("Переключено на русский");
    }
    if keys.just_pressed(KeyCode::Digit3) {
        *lang = Language("ja-JP".into());
        info!("日本語に切り替えました");
    }
}
```

### `examples/locale_switching/assets/`

```
assets/
└── locales/
    ├── en-US/
    │   └── main.ftl   ← content below
    ├── ru-RU/
    │   └── main.ftl   ← content below
    └── ja-JP/
        └── main.ftl   ← content below
```

**`assets/locales/en-US/main.ftl`**:

```ftl
greeting        = Hello, adventurer!
menu-play       = Play
menu-settings   = Settings
menu-exit       = Exit
hint-language   = Press 1=EN  2=RU  3=JP
```

**`assets/locales/ru-RU/main.ftl`**:

```ftl
greeting        = Привет, искатель приключений!
menu-play       = Играть
menu-settings   = Настройки
menu-exit       = Выйти
hint-language   = Нажмите 1=EN  2=RU  3=JP
```

**`assets/locales/ja-JP/main.ftl`**:

```ftl
greeting        = こんにちは、冒険者よ！
menu-play       = プレイ
menu-settings   = 設定
menu-exit       = 終了
hint-language   = 1=EN  2=RU  3=JP を押してください
```

> Note: `ja-JP` requires a Japanese font. Either use Noto Sans (covers CJK)
> or provide `fonts/NotoSansJP-Regular.ttf` from developer's assets/.

---

## 10 — Custom Settings Tab

**Concept:** developer adds a "Gameplay" settings tab alongside the built-in
Graphics / Audio / Controls tabs.

### `examples/custom_settings/src/main.rs`

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;
use serde::{Deserialize, Serialize};

// §0 here

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
```

### `examples/custom_settings/assets/`

```
assets/     ← empty
```

---

## 11 — Pause Menu

**Concept:** full pause/resume cycle. Demonstrates how the library's pause state
integrates with gameplay systems.

### `examples/pause_menu/src/main.rs`

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

// §0 here

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_splash(SplashConfig { skip_all: true, ..default() })
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), spawn_ball)
        .add_systems(Update, (
            move_ball.run_if(in_state(GameState::Playing)),
            toggle_pause,
        ))
        .add_systems(OnEnter(GameState::Paused),  on_pause)
        .add_systems(OnExit(GameState::Paused),   on_resume)
        .run();
}

fn spawn_ball(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            color:       Color::srgb(1.0, 0.5, 0.0),
            custom_size: Some(Vec2::splat(30.0)),
            ..default()
        },
        Ball { velocity: Vec2::new(200.0, 150.0) },
    ));
    info!("Ball spawned. Press Escape to pause.");
}

fn move_ball(
    mut q:    Query<(&mut Transform, &mut Ball)>,
    windows:  Query<&Window>,
    time:     Res<Time>,
) {
    let Ok(window) = windows.get_single() else { return };
    let hw = window.width()  * 0.5;
    let hh = window.height() * 0.5;

    for (mut t, mut ball) in &mut q {
        t.translation += ball.velocity.extend(0.0) * time.delta_secs();
        if t.translation.x.abs() > hw { ball.velocity.x *= -1.0; }
        if t.translation.y.abs() > hh { ball.velocity.y *= -1.0; }
    }
}

fn toggle_pause(
    keys:    Res<ButtonInput<KeyCode>>,
    state:   Res<State<GameState>>,
    mut nxt: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::Playing => nxt.set(GameState::Paused),
            GameState::Paused  => nxt.set(GameState::Playing),
            _ => {}
        }
    }
}

fn on_pause()  { info!("Game paused  — library shows pause menu"); }
fn on_resume() { info!("Game resumed — pause menu dismissed"); }
```

### `examples/pause_menu/assets/`

```
assets/     ← empty
```

---

## 12 — Custom Menu Buttons

**Concept:** fully custom main menu: Play, Level Select, Credits, Settings, Exit.
Each `Custom` button maps to a developer-owned state name.

### `examples/custom_menu/src/main.rs`

```rust
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
```

### `examples/custom_menu/assets/`

```
assets/     ← empty
```

---

## 13 — Mod Support (SearchPaths)

**Concept:** game checks multiple asset folders in order — base game, then DLC,
then user mods. First existing file wins.

### `examples/mod_support/src/main.rs`

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

// §0 here

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
```

### `examples/mod_support/assets/`

```
assets/
└── branding/
    └── studio.png    ✋  Base game studio logo

dlc/space_pack/assets/
└── branding/
    └── studio.png    ✋  (optional) DLC override logo — place to test override

mods/
└── branding/
    └── studio.png    ✋  (optional) User mod override — highest priority
```

---

## 14 — Embedded Assets Only

**Concept:** ship a game with NO developer `assets/` folder — every asset
(fonts, splash, sounds) comes from the library's embedded binary.

```toml
# Cargo.toml
bevy-launchpad = {
    version = "0.1",
    features = ["ui", "locale", "2d", "embedded_assets"]
}
#                                         ^^^^^^^^^^^^^^^^^
#                                         forces all assets into the binary
```

### `examples/embedded_only/src/main.rs`

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

// §0 here

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
```

### `examples/embedded_only/assets/`

```
(no assets/ folder needed — everything is compiled into the binary)
```

---

## 15 — CLI / Dev Mode

**Concept:** shows how CLI flags and env variables speed up development iteration.

### `examples/dev_mode/src/main.rs`

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

// §0 here

fn main() {
    // Run any of these in terminal to test different modes:
    //
    //   cargo run --example dev_mode
    //     → normal flow: boot → splash → menu
    //
    //   cargo run --example dev_mode -- --skip-splash
    //     → skip splash, go straight to menu
    //
    //   cargo run --example dev_mode -- --skip-splash --state Playing
    //     → skip splash AND menu, start in Playing immediately
    //
    //   BEVY_LAUNCHPAD_SKIP_SPLASH=1 cargo run --example dev_mode
    //     → env-var override (no code change needed)
    //
    //   BEVY_LAUNCHPAD_THEME=light cargo run --example dev_mode
    //     → force light theme

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_metadata(AppMetadata {
                    name:  "dev_mode_demo".into(),
                    title: "Dev Mode Demo".into(),
                    ..default()
                })
                .with_splash(SplashConfig {
                    screens: vec![
                        SplashScreenConfig::studio("branding/studio.png"),
                    ],
                    show_default_branding: true,
                    ..default()
                    // Note: CliArgs.skip_splash = true → SplashConfig.skip_all
                    //       is set automatically by LaunchpadPlugin
                })
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Dev mode: reached Playing state");
}
```

### `examples/dev_mode/assets/`

```
assets/
└── branding/
    └── studio.png   ✋  Any PNG to show when not using --skip-splash
```

---

## 16 — Multiple Instances Allowed

**Concept:** game explicitly allows running several instances simultaneously
(useful for local multiplayer testing, dedicated server tools).

### `examples/multi_instance/src/main.rs`

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

// §0 here

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                // Disable single-instance protection
                .allow_multiple_instances(true)
                .with_splash(SplashConfig { skip_all: true, ..default() })
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("Multi-instance: you can run several copies of this game");
}
```

### `examples/multi_instance/assets/`

```
assets/     ← empty
```

---

## 17 — Minimal No-UI (server / tool / headless)

**Concept:** core-only. No menus, no splash, no UI systems loaded.
Ideal for game servers, level editors, or CLI tools built on Bevy.

```toml
# Cargo.toml
bevy-launchpad = { version = "0.1", default-features = false }
# No "ui", "locale", "2d", "3d" — only core + utils
```

### `examples/no_ui/src/main.rs`

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;  // only core types exported without "ui" feature

// §0 here

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
```

### `examples/no_ui/assets/`

```
assets/     ← empty (or omit entirely for a server binary)
```

---

## 18 — Custom Assets Root

**Concept:** developer stores their assets in a non-standard folder name,
or distributes assets in a separate directory from the executable.

### `examples/custom_assets_root/src/main.rs`

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

// §0 here

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
```

### `examples/custom_assets_root/game_data/`

```
game_data/               ← non-standard root name
└── branding/
    └── logo.png   ✋  Studio or game logo
```

---

## Master Asset Manifest

Summary of all files that need to be added manually (✋).

| Example | File | Notes |
|---|---|---|
| `full_2d` | `assets/branding/studio_logo.png` | 1920×1080, PNG, dark bg |
| `full_2d` | `assets/branding/game_logo.png` | 1920×1080, PNG, dark bg |
| `full_2d` | `assets/locales/en-US/main.ftl` | see §03 content |
| `full_2d` | `assets/locales/ru-RU/main.ftl` | see §03 content |
| `full_3d` | `assets/branding/legal.png` | 1920×1080, white bg, age-rating text |
| `full_3d` | `assets/branding/publisher.png` | 1920×1080, dark bg |
| `full_3d` | `assets/branding/developer.png` | 1920×1080, dark bg |
| `full_3d` | `assets/branding/engine.png` | 1920×1080, dark bg |
| `full_3d` | `assets/fonts/Orbitron-Regular.ttf` | [fonts.google.com](https://fonts.google.com/specimen/Orbitron) |
| `full_3d` | `assets/fonts/Orbitron-Bold.ttf` | same download, Bold |
| `locale_switching` | `assets/locales/en-US/main.ftl` | see §09 content |
| `locale_switching` | `assets/locales/ru-RU/main.ftl` | see §09 content |
| `locale_switching` | `assets/locales/ja-JP/main.ftl` | see §09 content |
| `mod_support` | `assets/branding/studio.png` | base game logo |
| `mod_support` | `dlc/space_pack/assets/branding/studio.png` | optional DLC override |
| `mod_support` | `mods/branding/studio.png` | optional mod override |
| `dev_mode` | `assets/branding/studio.png` | any PNG |
| `custom_assets_root` | `game_data/branding/logo.png` | any PNG |

### Recommended image spec for splash screens

| Property | Value |
|---|---|
| Resolution | 1920 × 1080 px (16:9) |
| Format | PNG (lossless, supports transparency) |
| Background (dark) | `#000000` or near-black |
| Background (legal) | `#FFFFFF` or near-white |
| Logo area | centered, max 60% of frame width |
| File size | < 2 MB each |

---

*End of examples reference* — bevy_launchpad v0.1
