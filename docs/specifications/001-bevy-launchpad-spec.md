# bevy-launchpad — Complete Library Specification

> **Target stack:** Rust 1.93 · Bevy 0.18  
> **Purpose:** A production-ready, modular launcher framework for Bevy games.  
> Developers plug it in and get boot sequence, splash screens, main menu, settings,
> localization, and state management — without writing boilerplate.

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Repository Layout](#2-repository-layout)
3. [Cargo.toml & Feature Flags](#3-cargotoml--feature-flags)
4. [Module Architecture (src/)](#4-module-architecture-src)
5. [Asset Strategy](#5-asset-strategy)
6. [State Management](#6-state-management)
7. [Boot Sequence](#7-boot-sequence)
8. [Splash Screen System](#8-splash-screen-system)
9. [UI Module](#9-ui-module)
10. [Settings & Persistence](#10-settings--persistence)
11. [Localization](#11-localization)
12. [Utilities](#12-utilities)
13. [Public API & Builder Pattern](#13-public-api--builder-pattern)
14. [Examples](#14-examples)
15. [Environment Variable Backdoors](#15-environment-variable-backdoors)
16. [Decision Log](#16-decision-log)

---

## 1. Project Overview

### What it is

`bevy_launchpad` is a **single-crate Bevy plugin** that provides the entire
"launcher shell" of a game:

- Booting (paths, single-instance lock, CLI args)
- Splash screen sequence (fade in/out, skip triggers, legal screens)
- Main menu, pause menu, settings menu
- Settings persistence (RON format)
- Localization (Fluent)
- Theme system (colors, fonts, spacing)
- Generic state machine integration

### What it is NOT

- It does not provide gameplay systems (physics, AI, networking).
- It does not force a specific game state enum — the developer owns that.
- It does not embed large media (music, high-res textures).

### Design Principles

| Principle | Meaning |
|---|---|
| Convention over Configuration | Works with zero config; explicit config overrides defaults |
| Graceful Degradation | Missing assets → embedded fallback, never a hard crash |
| Feature-flag modularity | Disable UI, locale, audio independently |
| Generic State Integration | Works with any `States` enum via a trait |
| Environment Backdoors | Every key behavior can be overridden via env var (CI/testing) |

---

## 2. Repository Layout

```
bevy-launchpad/
├── Cargo.toml
├── build.rs                         # embeds assets into binary
├── README.md
├── CHANGELOG.md
├── ARCHITECTURE.md
├── LICENSE-MIT
├── LICENSE-APACHE
│
├── .github/
│   └── workflows/
│       ├── ci.yml
│       └── release.yml
│
├── src/                             # ALL library code lives here
│   ├── lib.rs                       # crate root, LaunchpadPlugin
│   ├── prelude.rs                   # re-exports for `use bevy_launchpad::prelude::*`
│   │
│   ├── core/                        # Framework logic — no rendering, no UI
│   │   ├── mod.rs
│   │   ├── plugin.rs                # LaunchpadCorePlugin<S>
│   │   ├── boot/
│   │   │   ├── mod.rs
│   │   │   ├── cli.rs               # CliArgs (clap)
│   │   │   ├── config.rs            # BootConfig
│   │   │   ├── metadata.rs          # AppMetadata resource
│   │   │   ├── paths.rs             # AppPaths (platform-specific)
│   │   │   └── sequence.rs          # BootSequence resource + system
│   │   ├── assets/
│   │   │   ├── mod.rs
│   │   │   └── resolver.rs          # AssetPath, AssetResolver, AssetsRootStrategy
│   │   ├── loading/
│   │   │   ├── mod.rs
│   │   │   ├── manifest.rs          # AssetManifest
│   │   │   ├── progress.rs          # LoadingState
│   │   │   └── tracker.rs           # AssetTracker
│   │   ├── splash/
│   │   │   ├── mod.rs
│   │   │   ├── sequence.rs          # SplashConfig, SplashScreenConfig, SplashSource
│   │   │   └── timer.rs             # SplashTimer
│   │   └── states/
│   │       ├── mod.rs
│   │       ├── machine.rs           # StateMachine<S>
│   │       ├── mapping.rs           # LaunchpadStates trait  ← KEY TRAIT
│   │       └── transitions.rs       # TransitionConfig, TransitionStateEvent<S>
│   │
│   ├── ui/                          # Visual components — depends on core
│   │   ├── mod.rs
│   │   ├── plugin.rs                # LaunchpadUiPlugin<S>
│   │   ├── common.rs                # UiChildSpawner type alias, shared constants
│   │   ├── theme/
│   │   │   ├── mod.rs
│   │   │   ├── colors.rs            # ThemeColors
│   │   │   ├── fonts.rs             # ThemeFonts
│   │   │   ├── spacing.rs           # ThemeSpacing
│   │   │   └── presets.rs           # ThemeConfig (dark/light/custom)
│   │   ├── widgets/
│   │   │   ├── mod.rs
│   │   │   ├── button.rs
│   │   │   ├── slider.rs
│   │   │   ├── checkbox.rs
│   │   │   ├── dropdown.rs
│   │   │   └── input.rs
│   │   ├── menu/
│   │   │   ├── mod.rs
│   │   │   ├── main_menu.rs
│   │   │   ├── pause_menu.rs
│   │   │   └── settings/
│   │   │       ├── mod.rs           # SettingsConfig
│   │   │       ├── panel.rs
│   │   │       ├── graphics.rs
│   │   │       ├── audio.rs
│   │   │       ├── controls.rs
│   │   │       └── general.rs
│   │   ├── modal/
│   │   │   ├── mod.rs
│   │   │   ├── dialog.rs
│   │   │   ├── confirm.rs
│   │   │   └── alert.rs
│   │   ├── transitions/
│   │   │   ├── mod.rs
│   │   │   ├── fade.rs
│   │   │   ├── slide.rs
│   │   │   └── zoom.rs
│   │   └── splash/
│   │       ├── mod.rs
│   │       ├── renderer.rs          # SplashScreen component, setup/update systems
│   │       └── animations.rs
│   │
│   ├── locale/                      # Localization (Fluent)
│   │   ├── mod.rs
│   │   ├── plugin.rs                # LocalePlugin
│   │   ├── fluent.rs
│   │   ├── language.rs              # Language resource
│   │   └── utils.rs
│   │
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── persistence.rs           # Settings save/load (RON)
│   │   ├── platform.rs              # get_data_dir()
│   │   ├── single_instance.rs       # SingleInstanceLock
│   │   ├── singleton.rs
│   │   └── validation.rs
│   │
│   └── assets/                      # EMBEDDED assets (compiled into the binary)
│       ├── fonts/
│       │   ├── NotoSans-Regular.ttf # Full Unicode + Cyrillic (~330 KB)
│       │   └── NotoSans-Bold.ttf
│       ├── branding/
│       │   └── default_splash.png   # "Powered by Bevy Launchpad" screen
│       └── audio/
│           └── ui/
│               ├── click.ogg
│               └── hover.ogg
│
├── examples/
│   ├── minimal_2d/
│   │   ├── Cargo.toml
│   │   ├── src/main.rs
│   │   └── assets/
│   ├── minimal_3d/
│   │   ├── Cargo.toml
│   │   ├── src/main.rs
│   │   └── assets/
│   ├── full_2d/
│   │   ├── Cargo.toml
│   │   ├── src/main.rs
│   │   └── assets/
│   │       ├── branding/
│   │       │   ├── studio_logo.png
│   │       │   └── game_logo.png
│   │       └── fonts/
│   │           └── Roboto-Regular.ttf
│   ├── full_3d/
│   │   ├── Cargo.toml
│   │   ├── src/main.rs
│   │   └── assets/
│   └── custom_theme/
│       ├── Cargo.toml
│       └── src/main.rs
│
├── tests/
│   ├── boot_sequence.rs
│   ├── settings_persistence.rs
│   └── state_transitions.rs
│
└── docs/
    ├── getting-started.md
    ├── architecture.md
    ├── theming.md
    ├── localization.md
    └── state-management.md
```

---

## 3. Cargo.toml & Feature Flags

```toml
[package]
name = "bevy_launchpad"
version = "0.1.1"
edition = "2024"
rust-version = "1.93"
authors = ["Oleg Alexandrov <alexandrovoleg.ru@gmail.com>"]
description = "A production-ready launcher framework for Bevy games."
license = "MIT OR Apache-2.0"
repository = "https://github.com/teratron/bevy-launchpad"
homepage = "https://teratron.github.io/bevy-launchpad"
documentation = "https://docs.rs/bevy_launchpad"
keywords = ["bevy", "game", "launcher", "framework", "ui"]
categories = ["game-development", "gui"]
readme = "README.md"

[workspace]
members = [".", "examples/*"]

[dependencies]
bevy = { version = "0.18", default-features = false, features = [
    "bevy_winit",
    "bevy_gilrs",
    "bevy_ui",
    "bevy_text",
    "bevy_asset",
    "bevy_render",
    "bevy_core_pipeline",
    "x11",
    "png",
] }
serde     = { version = "1.0",  features = ["derive"] }
ron       = "0.12"
thiserror = "2.0"
clap      = { version = "4.5",  features = ["derive"] }

[features]
# ── default: everything a typical indie game needs ──────────────────────────
default = ["ui", "locale", "2d", "3d"]

# ── dimension features ──────────────────────────────────────────────────────
2d = ["bevy/bevy_sprite", "bevy/bevy_gizmos"]
3d = ["bevy/bevy_pbr",    "bevy/bevy_gizmos"]

# ── optional modules ────────────────────────────────────────────────────────
ui     = []   # theme, widgets, menus, modals, transitions
locale = []   # Fluent localization

# ── optional extras ─────────────────────────────────────────────────────────
diagnostics     = ["bevy/bevy_dev_tools"]
audio           = ["bevy/bevy_audio", "bevy/vorbis"]
embedded_assets = []

# ── convenience bundles ─────────────────────────────────────────────────────
full    = ["default", "diagnostics", "audio", "embedded_assets"]
minimal = []   # core only — no UI, no locale

[[example]]
name = "minimal_2d"
path = "examples/minimal_2d/src/main.rs"
required-features = ["2d"]

[[example]]
name = "minimal_3d"
path = "examples/minimal_3d/src/main.rs"
required-features = ["3d"]

[[example]]
name = "full_2d"
path = "examples/full_2d/src/main.rs"
required-features = ["ui", "locale", "2d"]

[[example]]
name = "full_3d"
path = "examples/full_3d/src/main.rs"
required-features = ["ui", "locale", "3d"]

[[example]]
name = "custom_theme"
path = "examples/custom_theme/src/main.rs"
required-features = ["ui"]
```

---

## 4. Module Architecture (src/)

### Dependency graph

```plaintext
utils  ◄──────────────────────────────┐
  ▲                                   │
core  (boot, states, loading, splash) │
  ▲                                   │
ui    (theme, widgets, menu, modal)   │
  ▲                                   │
locale                                │
  ▲                                   │
lib.rs  (LaunchpadPlugin) ────────────┘
```

**Hard rules:**

- `core` MUST NOT import from `ui` or `locale`.
- `ui` MAY import from `core` (reads state, SplashConfig, AppPaths).
- `locale` MAY import from `core` (reads AppPaths for locale directory).
- `utils` has zero internal crate imports.

### `src/ui/common.rs` — shared UI helpers

Every UI sub-module imports from here instead of duplicating.

```rust
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::relationship::RelatedSpawnerCommands;

/// Standard child-spawner type used across all UI sub-modules.
pub type UiChildSpawner<'w> = RelatedSpawnerCommands<'w, ChildOf>;

pub const BUTTON_W:      f32 = 200.0;
pub const BUTTON_H:      f32 =  50.0;
pub const MENU_GAP:      f32 =  20.0;
pub const PANEL_PADDING: f32 =  24.0;
```

---

## 5. Asset Strategy

### Ownership matrix

| Asset | Owner | Location | How loaded |
|---|---|---|---|
| `NotoSans-Regular.ttf` | Library | `src/assets/fonts/` | `embedded_asset!` |
| `NotoSans-Bold.ttf` | Library | `src/assets/fonts/` | `embedded_asset!` |
| `default_splash.png` | Library | `src/assets/branding/` | `embedded_asset!` |
| `click.ogg` / `hover.ogg` | Library | `src/assets/audio/ui/` | `embedded_asset!` (requires `audio` feature) |
| Game splash images | Developer | `assets/branding/` | `AssetServer` |
| Game fonts | Developer | `assets/fonts/` | `AssetServer` |
| Game audio | Developer | `assets/audio/` | `AssetServer` |
| Sprites / Models | Developer | `assets/sprites/` or `assets/models/` | `AssetServer` |
| Locale files | Developer | `assets/locales/` | `AssetServer` |

### Convention for developer's assets/ folder

```plaintext
<game_root>/assets/          ← Bevy's default AssetServer root
├── branding/
│   ├── legal.png            # legal / age-rating screen
│   ├── studio_logo.png      # publisher / studio logo
│   └── game_logo.png        # game title card
├── fonts/
│   ├── MyFont-Regular.ttf
│   └── MyFont-Bold.ttf
├── locales/
│   ├── en-US/main.ftl
│   └── ru-RU/main.ftl
├── audio/
│   ├── music/
│   └── sfx/
├── sprites/                 # 2D games
└── models/                  # 3D games
```

### `src/core/assets/resolver.rs`

```rust
/// Resolution priority chain:
///
/// 1. Env var override   (BEVY_LAUNCHPAD_ASSETS_ROOT / per-asset overrides)
/// 2. Developer explicit path (set via builder)
/// 3. Bevy default: AssetServer resolves ./assets/ automatically
/// 4. Embedded library fallback (embedded_asset!)
/// 5. Panic with a human-readable message listing all tried paths
#[derive(Debug, Clone)]
pub enum AssetPath {
    /// Relative path for Bevy's AssetServer ("branding/logo.png")
    Project(String),
    /// Compiled into the binary ("embedded://bevy_launchpad/fonts/NotoSans-Regular.ttf")
    Embedded(&'static str),
    /// Absolute filesystem path (mods, editor, env-var override)
    Absolute(PathBuf),
}

impl AssetPath {
    pub fn as_load_path(&self) -> String {
        match self {
            Self::Project(p)  => p.clone(),
            Self::Embedded(p) => p.to_string(),
            Self::Absolute(p) => p.to_string_lossy().into_owned(),
        }
    }
}

/// How to locate the developer's assets/ root.
#[derive(Debug, Clone)]
pub enum AssetsRootStrategy {
    /// Let Bevy handle it — works for both `cargo run` and installed binaries.
    BevyDefault,
    /// Developer-supplied explicit path (relative or absolute).
    Explicit(PathBuf),
    /// Ordered list — first existing directory wins.
    /// Useful for DLC packs, mod support, or split asset locations.
    SearchPaths(Vec<PathBuf>),
}

impl Default for AssetsRootStrategy {
    fn default() -> Self {
        if let Ok(p) = std::env::var("BEVY_LAUNCHPAD_ASSETS_ROOT") {
            return Self::Explicit(PathBuf::from(p));
        }
        Self::BevyDefault
    }
}

pub struct AssetResolver;

impl AssetResolver {
    /// Resolve a font path.
    pub fn font(user_path: Option<&str>, embedded_fallback: &'static str) -> AssetPath {
        if let Ok(p) = std::env::var("BEVY_LAUNCHPAD_FONT_OVERRIDE") {
            return AssetPath::Absolute(PathBuf::from(p));
        }
        match user_path {
            Some(p) => AssetPath::Project(p.into()),
            None    => AssetPath::Embedded(embedded_fallback),
        }
    }

    /// Resolve a splash screen image. Returns None when skipping.
    pub fn splash_image(source: &SplashSource) -> Option<AssetPath> {
        if std::env::var("BEVY_LAUNCHPAD_SKIP_SPLASH").is_ok() {
            return None;
        }
        match source {
            SplashSource::File(p)   => Some(AssetPath::Project(p.clone())),
            SplashSource::Embedded  => Some(AssetPath::Embedded(
                "embedded://bevy_launchpad/branding/default_splash.png",
            )),
            SplashSource::ColorOnly => None,
        }
    }
}
```

### Embedding assets in `src/lib.rs`

```rust
use bevy::asset::embedded_asset;

pub(crate) fn register_embedded_assets(app: &mut App) {
    embedded_asset!(app, "src/assets/fonts/NotoSans-Regular.ttf");
    embedded_asset!(app, "src/assets/fonts/NotoSans-Bold.ttf");
    embedded_asset!(app, "src/assets/branding/default_splash.png");

    #[cfg(feature = "audio")]
    {
        embedded_asset!(app, "src/assets/audio/ui/click.ogg");
        embedded_asset!(app, "src/assets/audio/ui/hover.ogg");
    }
}
```

---

## 6. State Management

### `src/core/states/mapping.rs` — LaunchpadStates trait

This is the single integration point between the library and the developer's game.
The developer implements it once and the library drives the full boot flow.

```rust
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;

/// Implement this on your game's `States` enum.
/// bevy_launchpad uses these methods to know which variant
/// represents each phase of the launcher flow.
pub trait LaunchpadStates: States + FreelyMutableState + Default {
    fn booting()  -> Self;  // framework initialises here
    fn loading()  -> Self;  // assets are loading
    fn splash()   -> Self;  // splash screens showing
    fn menu()     -> Self;  // main menu active
    fn playing()  -> Self;  // gameplay running
    fn paused()   -> Self;  // game paused
}
```

**Developer implementation (required once per game):**

```rust
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

### Automatic transition flow

```plaintext
Booting ──(boot done)──► Loading ──(assets ready)──► Splash ──(screens done)──► Menu
                                                          │
                                        (skip_all=true or no screens)
                                                          ▼
                                                        Menu
```

Systems registered in `LaunchpadCorePlugin::build`:

```rust
app.add_systems(Update, (
    boot_to_loading::<S>.run_if(in_state(S::booting())),
    loading_to_splash::<S>.run_if(in_state(S::loading())),
    splash_to_menu::<S>.run_if(in_state(S::splash())),
).chain());
```

### `src/core/states/transitions.rs`

```rust
#[derive(Resource, Debug, Clone, Default)]
pub struct TransitionConfig {
    /// Fade duration in seconds (0.0 = instant cut).
    pub fade_duration: f32,
}

/// Send this message to trigger a state change from anywhere in the game.
#[derive(bevy::ecs::message::Message, Debug, Clone, PartialEq)]
pub struct TransitionStateEvent<S: States> {
    pub next: S,
}
```

---

## 7. Boot Sequence

### `src/core/boot/metadata.rs`

```rust
#[derive(Resource, Debug, Clone)]
pub struct AppMetadata {
    /// Used for data directory name (no spaces, lowercase).
    pub name:        String,
    /// Displayed in window title bar.
    pub title:       String,
    pub version:     String,
    pub description: String,
}

impl Default for AppMetadata {
    fn default() -> Self {
        Self {
            name:        "bevy_game".into(),
            title:       "Bevy Game".into(),
            version:     env!("CARGO_PKG_VERSION").into(),
            description: "A game built with Bevy Launchpad".into(),
        }
    }
}
```

### `src/core/boot/paths.rs`

```rust
#[derive(Resource, Debug, Clone)]
pub struct AppPaths {
    pub data_dir:           PathBuf,  // ~/.local/share/<name>  on Linux
    pub assets_dir:         PathBuf,  // resolved assets root
    pub settings_file:      PathBuf,  // data_dir/settings.ron
    pub log_file:           PathBuf,  // data_dir/session.log
    pub instance_lock_file: PathBuf,  // data_dir/instance.lock
}
```

### `src/core/boot/cli.rs`

```rust
#[derive(Parser, Resource, Debug, Clone, Default)]
pub struct CliArgs {
    /// Skip all splash screens.
    #[arg(long)]
    pub skip_splash: bool,

    /// Start in a specific state ("Menu", "Playing").
    #[arg(long)]
    pub state: Option<String>,

    /// Verbose logging.
    #[arg(short, long)]
    pub debug: bool,

    /// Custom log filter ("debug,game=info").
    #[arg(long)]
    pub log_filter: Option<String>,
}
```

### Boot steps executed inside `LaunchpadPlugin::build`

1. Resolve `AppPaths` from `AppMetadata.name`.
2. Call `paths.ensure_dirs()` — create data dir if absent.
3. Acquire single-instance lock (skipped when `allow_multiple_instances = true`).
4. Insert resources: `AppMetadata`, `AppPaths`, `CliArgs`.
5. Add `LaunchpadCorePlugin<S>`.
6. *(feature = "ui")* Add `LaunchpadUiPlugin<S>`.
7. *(feature = "locale")* Add `LocalePlugin`.
8. Call `register_embedded_assets(app)`.

---

## 8. Splash Screen System

### Design goals (AAA-inspired)

- `min_duration` — cannot skip before this expires (legal / rating screens).
- `max_duration` — auto-advances when reached; `None` = waits for skip input.
- Per-screen `SkipTrigger` — any input / Escape only / not skippable.
- Per-screen `fade_in` and `fade_out` timing.
- Per-screen `background` color shown while image loads.
- When `screens` is empty and `show_default_branding = true` (the default),
  the library shows its embedded "Powered by Bevy Launchpad" screen.

### `src/core/splash/sequence.rs`

```rust
#[derive(Resource, Debug, Clone)]
pub struct SplashConfig {
    pub screens:               Vec<SplashScreenConfig>,
    /// Show embedded branding when `screens` is empty. Default: true.
    pub show_default_branding: bool,
    /// Skip the entire splash sequence. Default: false.
    pub skip_all:              bool,
}

impl Default for SplashConfig {
    fn default() -> Self {
        Self {
            screens:               vec![],
            show_default_branding: true,
            skip_all:              false,
        }
    }
}

impl SplashConfig {
    /// Returns the screens that will actually be displayed, applying all rules.
    pub fn effective_screens(&self) -> Vec<SplashScreenConfig> {
        if self.skip_all
            || std::env::var("BEVY_LAUNCHPAD_SKIP_SPLASH").is_ok()
        {
            return vec![];
        }
        if !self.screens.is_empty() {
            return self.screens.clone();
        }
        if self.show_default_branding {
            vec![SplashScreenConfig::default_branding()]
        } else {
            vec![]
        }
    }
}

// ── Single splash screen ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SplashScreenConfig {
    pub source:       SplashSource,
    pub min_duration: f32,          // seconds — cannot be skipped before this
    pub max_duration: Option<f32>,  // seconds — auto-advance; None = wait for input
    pub skip:         SkipTrigger,
    pub fade_in:      f32,
    pub fade_out:     f32,
    pub background:   Color,
}

#[derive(Debug, Clone)]
pub enum SplashSource {
    /// Path relative to developer's assets/ folder.
    File(String),
    /// Library's embedded "Powered by Bevy Launchpad" image.
    Embedded,
    /// No image — just a solid background color.
    ColorOnly,
}

#[derive(Debug, Clone, Default)]
pub enum SkipTrigger {
    #[default]
    AnyInput,    // any keyboard key or mouse button
    EscapeOnly,  // only the Escape key
    None,        // cannot be skipped (use for legal screens)
}

// ── Factory constructors ─────────────────────────────────────────────────────

impl SplashScreenConfig {
    /// Library's default branding screen.
    pub fn default_branding() -> Self {
        Self {
            source:       SplashSource::Embedded,
            min_duration: 1.5,
            max_duration: Some(2.5),
            skip:         SkipTrigger::AnyInput,
            fade_in:      0.3,
            fade_out:     0.3,
            background:   Color::BLACK,
        }
    }

    /// Legal / age-rating screen — cannot be skipped.
    pub fn legal(path: &str) -> Self {
        Self {
            source:       SplashSource::File(path.into()),
            min_duration: 5.0,
            max_duration: Some(5.0),
            skip:         SkipTrigger::None,
            fade_in:      0.5,
            fade_out:     0.5,
            background:   Color::WHITE,
        }
    }

    /// Publisher / studio logo screen.
    pub fn studio(path: &str) -> Self {
        Self {
            source:       SplashSource::File(path.into()),
            min_duration: 2.0,
            max_duration: Some(3.0),
            skip:         SkipTrigger::AnyInput,
            fade_in:      0.4,
            fade_out:     0.4,
            background:   Color::BLACK,
        }
    }

    /// Engine / "powered by" logo screen.
    pub fn engine(path: &str) -> Self {
        Self {
            source:       SplashSource::File(path.into()),
            min_duration: 1.5,
            max_duration: Some(2.5),
            skip:         SkipTrigger::AnyInput,
            fade_in:      0.3,
            fade_out:     0.3,
            background:   Color::BLACK,
        }
    }

    // ── Builder methods ───────────────────────────────────────────────────────

    pub fn with_duration(mut self, min: f32, max: f32) -> Self {
        self.min_duration = min;
        self.max_duration = Some(max);
        self
    }

    pub fn with_fade(mut self, fade_in: f32, fade_out: f32) -> Self {
        self.fade_in  = fade_in;
        self.fade_out = fade_out;
        self
    }

    pub fn skippable(mut self, trigger: SkipTrigger) -> Self {
        self.skip = trigger;
        self
    }

    pub fn on_background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }
}
```

---

## 9. UI Module

### Theme system

```rust
// src/ui/theme/fonts.rs
#[derive(Resource, Debug, Clone)]
pub struct ThemeFonts {
    /// None  → Bevy built-in FiraMono (zero binary cost, always available).
    /// Some  → path loaded via AssetServer or embedded:// URI.
    pub regular: Option<String>,
    pub bold:    Option<String>,
    pub mono:    Option<String>,  // None → Bevy built-in FiraMono
}

impl ThemeFonts {
    /// Bevy's built-in font everywhere — zero bytes added to binary.
    pub fn bevy_default() -> Self {
        Self { regular: None, bold: None, mono: None }
    }

    /// Embedded Noto Sans — full Unicode including Cyrillic and CJK.
    pub fn noto_sans() -> Self {
        Self {
            regular: Some("embedded://bevy_launchpad/fonts/NotoSans-Regular.ttf".into()),
            bold:    Some("embedded://bevy_launchpad/fonts/NotoSans-Bold.ttf".into()),
            mono:    None,
        }
    }

    /// Developer's own fonts from their assets/ folder.
    pub fn from_project(regular: impl Into<String>, bold: impl Into<String>) -> Self {
        Self {
            regular: Some(regular.into()),
            bold:    Some(bold.into()),
            mono:    None,
        }
    }
}

impl Default for ThemeFonts {
    fn default() -> Self { Self::bevy_default() }
}
```

```rust
// src/ui/theme/presets.rs
#[derive(Resource, Debug, Clone, Default)]
pub struct ThemeConfig {
    pub colors:  ThemeColors,
    pub fonts:   ThemeFonts,
    pub spacing: ThemeSpacing,
}

impl ThemeConfig {
    pub fn dark() -> Self { Self::default() }

    pub fn light() -> Self {
        Self {
            colors: ThemeColors {
                primary:    Color::srgb(0.1, 0.3, 0.7),
                secondary:  Color::srgb(0.3, 0.5, 0.8),
                background: Color::WHITE,
                text:       Color::BLACK,
            },
            ..default()
        }
    }

    /// Dark theme with Noto Sans (use when Cyrillic / full Unicode needed).
    pub fn dark_with_noto() -> Self {
        Self { fonts: ThemeFonts::noto_sans(), ..Self::dark() }
    }
}
```

### Main Menu config

```rust
// src/ui/menu/main_menu.rs
#[derive(Resource, Debug, Clone)]
pub struct MainMenuConfig {
    pub title:   String,
    pub buttons: Vec<MenuButton>,
}

#[derive(Debug, Clone)]
pub enum MenuButton {
    Play,
    Settings,
    Exit,
    Custom { label: String, target_state_name: String },
}

impl Default for MainMenuConfig {
    fn default() -> Self {
        Self {
            title:   String::new(),
            buttons: vec![MenuButton::Play, MenuButton::Settings, MenuButton::Exit],
        }
    }
}
```

---

## 10. Settings & Persistence

```rust
// src/utils/persistence.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsSettings {
    pub fullscreen:  bool,
    pub vsync:       bool,
    pub resolution:  (u32, u32),
    pub quality:     QualityPreset,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum QualityPreset { Low, Medium, #[default] High, Ultra }

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            fullscreen:  false,
            vsync:       true,
            resolution:  (1920, 1080),
            quality:     QualityPreset::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    pub master_volume: f32,   // 0.0 – 1.0
    pub music_volume:  f32,
    pub sfx_volume:    f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self { master_volume: 1.0, music_volume: 0.8, sfx_volume: 1.0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameSettings {
    pub graphics: GraphicsSettings,
    pub audio:    AudioSettings,
    pub locale:   String,   // e.g. "en-US"
    pub theme:    String,   // "dark" | "light"
}

pub struct Persistence;

impl Persistence {
    pub fn load(path: &std::path::Path) -> Result<GameSettings, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        Ok(ron::from_str(&content)?)
    }

    pub fn save(
        settings: &GameSettings,
        path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let content = ron::ser::to_string_pretty(settings, Default::default())?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
```

---

## 11. Localization

```rust
// src/locale/language.rs
#[derive(Resource, Debug, Clone, Default)]
pub struct Language(pub String);   // "en-US", "ru-RU", "ja-JP"
```

- Developer places `.ftl` files in `assets/locales/<lang>/main.ftl`.
- `LocalePlugin` reads the `Language` resource on startup and loads the bundle.
- Runtime language switching: change the `Language` resource value;
  the plugin detects the change and reloads the Fluent bundle.

---

## 12. Utilities

### Single-instance lock behavior

1. On startup: attempt to create `AppPaths.instance_lock_file` exclusively.
2. Write current PID into the file.
3. If file already exists: read PID, check whether that process is alive.
   - **Dead PID** → stale lock; delete and retry.
   - **Live PID** → return `Err(SingleInstanceError::AlreadyRunning)` → `LaunchpadPlugin::build` panics with a clear message.
4. `Drop` implementation removes the lock file on clean exit.
5. Setting `allow_multiple_instances = true` bypasses the entire mechanism.

---

## 13. Public API & Builder Pattern

### `src/lib.rs` — LaunchpadPlugin fields

```rust
pub struct LaunchpadPlugin<S: LaunchpadStates> {
    _marker:                      PhantomData<S>,
    pub metadata:                 AppMetadata,
    pub cli_args:                 CliArgs,
    pub allow_multiple_instances: bool,
    pub assets_root:              AssetsRootStrategy,
    pub splash_config:            SplashConfig,

    #[cfg(feature = "ui")]
    pub theme:     ThemeConfig,
    #[cfg(feature = "ui")]
    pub main_menu: MainMenuConfig,

    #[cfg(feature = "locale")]
    pub default_locale: String,
}
```

### `LaunchpadPluginBuilder<S>` — complete method reference

| Method | Feature gate | Default value |
|---|---|---|
| `.with_metadata(AppMetadata)` | always | `AppMetadata::default()` |
| `.with_cli_args(CliArgs)` | always | parsed from env / argv |
| `.allow_multiple_instances(bool)` | always | `false` |
| `.with_assets_root(AssetsRootStrategy)` | always | `BevyDefault` |
| `.with_splash(SplashConfig)` | always | `SplashConfig::default()` |
| `.with_theme(ThemeConfig)` | `ui` | `ThemeConfig::dark()` |
| `.with_fonts(ThemeFonts)` | `ui` | `ThemeFonts::bevy_default()` |
| `.with_main_menu(MainMenuConfig)` | `ui` | Play / Settings / Exit |
| `.with_locale(default: &str)` | `locale` | `"en-US"` |

---

## 14. Examples

All examples share the same `LaunchpadStates` impl below — it is omitted in
subsequent examples for brevity.

```rust
// Shared impl for all examples
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

### 14.1 — Absolute minimum 2D

```rust
// examples/minimal_2d/src/main.rs
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState { #[default] Booting, Loading, Splash, Menu, Playing, Paused }
impl LaunchpadStates for GameState { /* ... */ }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Zero configuration:
        // boot → embedded splash ("Powered by Bevy Launchpad") → menu → Playing
        // theme: dark  |  font: Bevy built-in  |  locale: en-US
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
}
```

---

### 14.2 — Absolute minimum 3D

```rust
// examples/minimal_3d/src/main.rs
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState { #[default] Booting, Loading, Splash, Menu, Playing, Paused }
impl LaunchpadStates for GameState { /* ... */ }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LaunchpadPlugin::<GameState>::default())
        .add_systems(OnEnter(GameState::Playing), setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes:    ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.8, 0.2))),
    ));
    commands.spawn((
        PointLight { shadows_enabled: true, ..default() },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
```

---

### 14.3 — Full-featured 2D (typical indie game)

```rust
// examples/full_2d/src/main.rs
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState { #[default] Booting, Loading, Splash, Menu, Playing, Paused }
impl LaunchpadStates for GameState { /* ... */ }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_metadata(AppMetadata {
                    name:    "full_2d_game".into(),
                    title:   "My 2D Adventure".into(),
                    version: "0.1.0".into(),
                    ..default()
                })
                .with_splash(SplashConfig {
                    screens: vec![
                        SplashScreenConfig::studio("branding/studio_logo.png"),
                        SplashScreenConfig::engine("branding/game_logo.png")
                            .with_duration(2.0, 3.0)
                            .with_fade(0.5, 0.5),
                    ],
                    show_default_branding: false,
                    ..default()
                })
                // Noto Sans for Cyrillic / full Unicode support
                .with_fonts(ThemeFonts::noto_sans())
                .with_theme(ThemeConfig::dark())
                .with_main_menu(MainMenuConfig {
                    title: "My 2D Adventure".into(),
                    buttons: vec![
                        MenuButton::Play,
                        MenuButton::Custom {
                            label: "Credits".into(),
                            target_state_name: "Credits".into(),
                        },
                        MenuButton::Settings,
                        MenuButton::Exit,
                    ],
                })
                .with_locale("en-US")
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), setup_level)
        .add_systems(Update, game_loop.run_if(in_state(GameState::Playing)))
        .run();
}

fn setup_level(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite {
        color:       Color::srgb(0.0, 0.5, 0.8),
        custom_size: Some(Vec2::new(50.0, 50.0)),
        ..default()
    });
}

fn game_loop() { /* gameplay */ }
```

---

### 14.4 — Full-featured 3D (AAA splash sequence)

```rust
// examples/full_3d/src/main.rs
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState { #[default] Booting, Loading, Splash, Menu, Playing, Paused }
impl LaunchpadStates for GameState { /* ... */ }

#[derive(Component)]
struct RotatingCube;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_metadata(AppMetadata {
                    name:  "galaxy_quest_3d".into(),
                    title: "Galaxy Quest 3D".into(),
                    ..default()
                })
                // AAA four-screen splash sequence
                .with_splash(SplashConfig {
                    screens: vec![
                        SplashScreenConfig::legal("branding/legal.png"),     // cannot skip
                        SplashScreenConfig::studio("branding/publisher.png"),
                        SplashScreenConfig::studio("branding/developer.png"),
                        SplashScreenConfig::engine("branding/engine.png")
                            .with_fade(0.5, 0.5),
                    ],
                    show_default_branding: false,
                    ..default()
                })
                // Game-specific font from developer's assets/
                .with_fonts(ThemeFonts::from_project(
                    "fonts/Orbitron-Regular.ttf",
                    "fonts/Orbitron-Bold.ttf",
                ))
                .with_theme(ThemeConfig {
                    colors: ThemeColors {
                        primary:    Color::srgb(0.0, 0.8, 1.0),
                        secondary:  Color::srgb(0.0, 0.5, 0.8),
                        background: Color::srgb(0.03, 0.03, 0.06),
                        text:       Color::WHITE,
                    },
                    ..default()
                })
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), setup_3d)
        .add_systems(Update, rotate.run_if(in_state(GameState::Playing)))
        .run();
}

fn setup_3d(
    mut commands: Commands,
    mut meshes:    ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.8, 0.2))),
        Transform::from_xyz(0.0, 1.0, 0.0),
        RotatingCube,
    ));
    commands.spawn((
        PointLight { intensity: 1500.0, shadows_enabled: true, ..default() },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn rotate(mut q: Query<&mut Transform, With<RotatingCube>>, time: Res<Time>) {
    for mut t in &mut q { t.rotate_y(time.delta_secs()); }
}
```

---

### 14.5 — Custom theme, no splash (2D)

```rust
// examples/custom_theme/src/main.rs
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState { #[default] Booting, Loading, Splash, Menu, Playing, Paused }
impl LaunchpadStates for GameState { /* ... */ }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_theme(ThemeConfig {
                    colors: ThemeColors {
                        primary:    Color::srgb(0.8, 0.2, 0.4),
                        secondary:  Color::srgb(0.6, 0.1, 0.3),
                        background: Color::srgb(0.1, 0.05, 0.1),
                        text:       Color::srgb(0.9, 0.8, 0.8),
                    },
                    fonts:   ThemeFonts::noto_sans(),
                    spacing: ThemeSpacing::default(),
                })
                .with_splash(SplashConfig {
                    show_default_branding: false,
                    skip_all: true,   // go straight to menu
                    ..default()
                })
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing), |mut commands: Commands| {
            commands.spawn(Camera2d);
        })
        .run();
}
```

---

### 14.6 — Minimal core only (no UI, headless / server)

```toml
# Cargo.toml
[dependencies]
bevy_launchpad = { version = "0.1", default-features = false }
```

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum ServerState { #[default] Booting, Loading, Splash, Menu, Playing, Paused }
impl LaunchpadStates for ServerState { /* ... */ }

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(
            LaunchpadPlugin::<ServerState>::builder()
                .with_splash(SplashConfig { skip_all: true, ..default() })
                .allow_multiple_instances(true)
                .build(),
        )
        .add_systems(OnEnter(ServerState::Playing), server_tick)
        .run();
}

fn server_tick() { info!("Server running"); }
```

---

## 15. Environment Variable Backdoors

All variables are evaluated at runtime (not compile time).
They are the highest-priority override for every configurable behavior.

| Variable | Effect |
|---|---|
| `BEVY_LAUNCHPAD_ASSETS_ROOT=/path` | Override the developer's assets root directory |
| `BEVY_LAUNCHPAD_SKIP_SPLASH=1` | Skip all splash screens unconditionally |
| `BEVY_LAUNCHPAD_FONT_OVERRIDE=/abs/path/font.ttf` | Replace every font slot with one file |
| `BEVY_LAUNCHPAD_THEME=light` | Force a theme preset at startup |
| `BEVY_LAUNCHPAD_LOCALE=ru-RU` | Force a locale at startup |

**Primary use cases:**

- CI/CD pipelines: `BEVY_LAUNCHPAD_SKIP_SPLASH=1 cargo run --example full_2d`
- Automated screenshot / integration tests
- Accessibility tools that need to override fonts
- Fast developer iteration

---

## 16. Decision Log

| # | Decision | Rationale |
|---|---|---|
| 1 | Single crate, not workspace | Lower barrier for users; workspace only when >15k LOC |
| 2 | `LaunchpadStates` trait | Developer owns their state enum; library maps to it generically |
| 3 | Noto Sans embedded + Bevy FiraMono fallback | Noto covers Cyrillic/CJK; FiraMono is zero-cost always available |
| 4 | Show default branding when no screens given | Library discoverability; trivially disabled with one flag |
| 5 | `min_duration` per splash screen | Legal and publisher requirements in shipped titles |
| 6 | RON for settings persistence | Human-readable; standard in the Bevy ecosystem |
| 7 | `AssetsRootStrategy::BevyDefault` as default | Bevy handles dev vs release path resolution correctly |
| 8 | `AssetsRootStrategy::SearchPaths` variant | Future-proof for DLC, mod support, split asset locations |
| 9 | Env var backdoors for all key behaviors | CI testing, automation, fast iteration without recompilation |
| 10 | `UiChildSpawner` alias in `ui/common.rs` | Eliminate copy-paste of the same type across 8+ UI files |
| 11 | `core` must not import `ui` | Enables `minimal` feature with absolutely zero UI overhead |
| 12 | `SplashSource::ColorOnly` variant | Useful for fade-in-from-black without any image |
| 13 | `SkipTrigger::None` for legal screens | Publishers legally require some screens to be un-skippable |

---

*End of specification* — bevy_launchpad v0.1
