# bevy-launchpad — Complete Library Specification

Bevy 0.18  ·  Rust Edition 2024

## 1. Overview

bevy_launchpad is a modular, production-ready launcher framework for Bevy 0.18 games. It eliminates boilerplate by providing a complete, configurable boot pipeline — state machine, splash screens, main menu, theming, localization, asset resolution, single-instance protection, and CLI developer tools — all through a single fluent builder API.

### 1.1 Design Goals

* Zero-config defaults — a minimal game compiles and runs with a single `.add_plugins(LaunchpadPlugin::<AppState>::default())` call.

* Opt-in complexity — every module (UI, locale, diagnostics, audio) is a Cargo feature; unused code is never compiled.

* User-extensible states — the `LaunchpadStates` derive macro maps any user enum to the framework state machine without naming restrictions.

* Framework stays out of the way — once Playing state is reached, the library is silent; no forced ECS patterns or mandatory components.

* Developer ergonomics — CLI flags (`--skip-splash`, `--state Playing`) and env-var overrides accelerate the inner dev loop without code changes.

### 1.2 Supported Platforms

| Platform | Tier | Notes |
| :---- | :---- | :---- |
| Linux (x11) | 1 — fully supported | Primary CI target |
| Windows 10/11 | 1 — fully supported |  |
| macOS 12+ | 1 — fully supported |  |
| WebAssembly | 2 — experimental | single-instance and file-lock disabled |
| Android / iOS | 3 — not tested | may work with custom feature flags |

### 1.3 Crate Structure

| Crate | Path | Purpose |
| :---- | :---- | :---- |
| bevy_launchpad | . | Main library (this spec) |
| bevy_launchpad_derive | crates/bevy_launchpad_derive | Proc-macro: `#[derive(LaunchpadStates)]` |

## 2. Cargo Configuration

### 2.1 Features

| Feature | Enables | Pulls in (Bevy) |
| :---- | :---- | :---- |
| default | ui, locale, 2d, 3d | All dimension + UI plugins |
| 2d | 2D rendering | bevy_sprite, bevy_gizmos |
| 3d | 3D rendering | bevy_pbr, bevy_gizmos |
| ui | Theme, menus, splash UI, widgets | bevy_ui, bevy_text (already in default) |
| locale | Fluent localization, Language resource | (none extra) |
| diagnostics | Dev overlay (FPS, system info) | bevy_dev_tools |
| audio | Audio playback | bevy_audio, vorbis |
| embedded_assets | Embed NotoSans + default splash PNG | (none extra) |
| full | default + diagnostics + audio + embedded_assets | All of the above |
| minimal | Core only — no UI, no locale | Minimal Bevy subset |

**NOTE:** The `embedded_assets` feature embeds `NotoSans-Regular.ttf`, `NotoSans-Bold.ttf`, and `default_splash.png` into the binary via `bevy::asset::embedded_asset!`. Without this feature the fonts are still registered (`register_embedded_assets` is always called) but the files must exist on the filesystem. This is a known issue — see Section 7.1 for the fix.

### 2.2 Workspace Layout

```text
bevy-launchpad/
├── Cargo.toml          ← workspace root + main crate
├── crates/
│   └── bevy_launchpad_derive/
│       ├── Cargo.toml
│       └── src/lib.rs
├── examples/
│   ├── minimal_2d/
│   ├── minimal_3d/
│   ├── full_2d/
│   ├── full_3d/
│   ├── custom_theme_dark/
│   ├── custom_theme_light/
│   ├── custom_theme/
│   ├── custom_menu/
│   ├── custom_settings/
│   ├── custom_assets_root/
│   ├── color_splash/
│   ├── no_splash/
│   ├── embedded_only/
│   ├── pause_menu/
│   ├── dev_mode/
│   ├── locale_switching/
│   ├── mod_support/
│   ├── multi_instance/
│   └── no_ui/
└── src/
    ├── lib.rs
    ├── prelude.rs
    ├── core/           ← no UI dependency
    ├── ui/             ← cfg(feature = "ui")
    ├── locale/         ← cfg(feature = "locale")
    └── utils/
```

**NOTE:** The example `custom_theme` (`examples/custom_theme/`) exists on disk but is not registered as `[[example]]` in `Cargo.toml`. Either register it or remove the directory. This spec registers it under the name `custom_theme` with `required-features = ["ui"]`.

## 3. State Machine

### 3.1 Built-in State Flow

The framework defines a six-stage lifecycle that every game progresses through. Each stage maps to a required variant of the user's state enum.

| Stage | Variant (default) | Purpose | Auto-advance condition |
| :---- | :---- | :---- | :---- |
| Booting | AppState::Booting | Platform init, path resolution, single-instance lock, CLI parsing | `BootSequence::is_finished == true` |
| Loading | AppState::Loading | Asset preloading, manifest processing | `AssetTracker` fully loaded (see §3.3) |
| Splash | AppState::Splash | Splash screen sequence | All `SplashScreenConfig` screens complete |
| Menu | AppState::Menu | Main menu UI active | Player presses Play (or `--state` CLI flag) |
| Playing | AppState::Playing | Active gameplay — entirely user-controlled | User code only |
| Paused | AppState::Paused | Pause overlay active | User code only |

### 3.2 LaunchpadStates Trait

The trait is the bridge between user enums and the framework's internal logic:

```rust
pub trait LaunchpadStates: States + FreelyMutableState + Default {
    fn booting()  -> Self;
    fn loading()  -> Self;
    fn splash()   -> Self;
    fn menu()     -> Self;
    fn playing()  -> Self;
    fn paused()   -> Self;
}
```

The framework uses these methods everywhere instead of hard-coding `AppState` variants, so it works with any user-defined enum.

### 3.3 #[derive(LaunchpadStates)] Macro

The proc-macro crate (`bevy_launchpad_derive`) generates the `LaunchpadStates` impl automatically. It supports two mapping modes:

#### Convention-based (zero attributes)

If variant names match the convention exactly, no annotation is needed:

```rust
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[derive(LaunchpadStates)]
enum GameState {
    #[default]
    Booting, Loading, Splash, Menu, Playing, Paused,
    Credits,      // extra custom state — ignored by macro
    LevelSelect,  // extra custom state — ignored by macro
}
```

#### Attribute-based (explicit mapping)

Use `#[launchpad(<method>)]` when variant names differ from the convention:

```rust
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[derive(LaunchpadStates)]
enum MyState {
    #[default]
    #[launchpad(booting)] Init,
    #[launchpad(loading)] Load,
    #[launchpad(splash)]  Intro,
    #[launchpad(menu)]    Home,
    #[launchpad(playing)] InGame,
    #[launchpad(paused)]  Pause,
    Cutscene, // custom — not mapped
}
```

Convention and attribute modes can be freely mixed. The macro emits a compile-time error listing exactly which mappings are missing and how to fix them.

### 3.4 State Transitions

### Automatic transitions

* Booting → Loading: triggered when `BootSequence::is_finished` is true (after ~2 s simulated init).

* Loading → Splash: triggered when `AssetTracker::progress()` returns 1.0 (all registered assets loaded).

* Splash → Menu: triggered by `SplashRenderer` after all effective screens complete.

### Manual transitions

User code triggers transitions via the message system:

```rust
// Send a transition event (works from any system)
commands.send_message(TransitionStateEvent { next: GameState::Playing });

// Or use Bevy's NextState directly
next_state.set(GameState::Paused);
```

**FIXED:** `auto_transition_loading` previously called `next_state.set(S::splash())` unconditionally every frame, causing repeated state sets. It is now guarded by `AssetTracker::progress() >= 1.0` and a one-shot flag to fire exactly once.

## 4. Plugin & Builder API

### 4.1 Entry Points

There are two equivalent ways to add the plugin:

```rust
// A — zero-config default
app.add_plugins(LaunchpadPlugin::<AppState>::default());

// B — fluent builder (recommended for all non-trivial games)
app.add_plugins(
    LaunchpadPlugin::<AppState>::builder()
        .with_metadata(AppMetadata { name: "mygame".into(), title: "My Game".into(), ..default() })
        .with_splash(SplashConfig { skip_all: true, ..default() })
        .with_theme(ThemeConfig::dark())
        .with_locale("en-US")
        .build(),
);
```

### 4.2 Builder Methods

| Method | Type | Default | Feature |
| :---- | :---- | :---- | :---- |
| `.with_metadata(m)` | `AppMetadata` | name=bevy_game, title=Bevy Game | always |
| `.with_cli_args(a)` | `CliArgs` | `CliArgs::parse_args()` | always |
| `.allow_multiple_instances(b)` | `bool` | false | always |
| `.with_assets_root(s)` | `AssetsRootStrategy` | `BevyDefault` (+ env var) | always |
| `.with_splash(c)` | `SplashConfig` | show_default_branding=true | always |
| `.with_theme(t)` | `ThemeConfig` | `ThemeConfig::dark()` | ui |
| `.with_fonts(f)` | `ThemeFonts` | `ThemeFonts::bevy_default()` | ui |
| `.with_main_menu(m)` | `MainMenuConfig` | Play / Settings / Exit | ui |
| `.with_locale(s)` | `impl Into<String>` | "en-US" | locale |

### 4.3 Plugin Initialization Order

1. `register_embedded_assets()` — embeds fonts & branding into Bevy's `AssetServer`.

2. `AppPaths::new()` — resolves platform data dir and assets dir.

3. `acquire_single_instance_lock()` — writes PID lock file; exits if another instance is live.

4. Core resources inserted: `AppMetadata`, `CliArgs`, `SplashConfig`, `AssetsRootStrategy`, `AppPaths`.

5. `LaunchpadCorePlugin<S>` added — boot sequence, state transitions, asset tracking.

6. `LaunchpadUiPlugin<S>` added (feature = "ui") — theme, menus, splash renderer.

7. `LocalizationPlugin` added (feature = "locale") — `Language` resource, Fluent bundle loader.

### 4.4 CLI Arguments

| Flag | Type | Effect |
| :---- | :---- | :---- |
| `--skip-splash` | `bool` | Sets `SplashConfig::skip_all = true` at runtime (no recompile) |
| `--state <name>` | `Option<String>` | Jumps directly to named state after boot (case-insensitive) |
| `--debug / -d` | `bool` | Enables verbose logging |
| `--log-filter <filter>` | `Option<String>` | Custom `env_logger` filter string |

#### Environment variable overrides

| Variable | Effect |
| :---- | :---- |
| `BEVY_LAUNCHPAD_SKIP_SPLASH=1` | Skip all splash screens |
| `BEVY_LAUNCHPAD_ASSETS_ROOT=<path>` | Override assets root directory |
| `BEVY_LAUNCHPAD_FONT_OVERRIDE=<path>` | Override all font paths |
| `BEVY_LAUNCHPAD_THEME=light` | Force light theme at startup |
| `RUST_LOG=<filter>` | Standard Rust log filter |

## 5. Boot Module (src/core/boot/)

### 5.1 AppMetadata

```rust
#[derive(Resource, Debug, Clone)]
pub struct AppMetadata {
    pub name:        String,  // used for data dir: ~/.local/share/<name>/
    pub title:       String,  // window title
    pub version:     String,
    pub description: String,
}
```

### 5.2 AppPaths

```rust
#[derive(Resource, Debug, Clone)]
pub struct AppPaths {
    pub data_dir:           PathBuf,   // ~/.local/share/<app_name>/
    pub assets_dir:         PathBuf,   // resolved via AssetsRootStrategy
    pub settings_file:      PathBuf,   // data_dir/settings.ron
    pub log_file:           PathBuf,   // data_dir/session.log
    pub instance_lock_file: PathBuf,   // data_dir/instance.lock
}
```

`AppPaths::new(app_name, strategy)` resolves all paths at startup. `AppPaths::ensure_dirs()` creates the data directory if it does not exist.

### 5.3 BootSequence

A simple progress resource used to gate the Booting → Loading transition:

```rust
#[derive(Resource, Debug, Default)]
pub struct BootSequence {
    pub progress:    f32,   // 0.0 → 1.0
    pub is_finished: bool,
}
```

// `update_boot_progress` advances at 0.5/s (simulated).
// Replace with real platform checks in shipping builds.

### 5.4 Single-Instance Protection

When `allow_multiple_instances` is false (default), the framework creates a PID lock file at `AppPaths::instance_lock_file` on startup. The lock file contains:

```text
pid=<process_id>
```

If the file already exists, the framework reads the stored PID and checks whether that process is still alive. If the process is dead (stale lock), the file is removed and a fresh lock is acquired. If the process is alive, the application exits with an error.

#### Platform implementation

* Linux: checks `/proc/<pid>` existence.

* macOS: uses `kill(pid, 0)` via `libc`. The `/proc` check is Linux-only.

* Windows: calls `OpenProcess` + `GetExitCodeProcess` via unsafe FFI.

**FIXED:** The original `#[cfg(not(windows))]` branch used `/proc/<pid>` which does not exist on macOS. The macOS branch now uses POSIX `kill(pid, 0)`: returns 0 if alive, `ESRCH` if the process does not exist.

### 5.5 CliArgs

Parsed once at plugin initialization via `CliArgs::parse_args()`. Stored as a Bevy Resource for downstream systems to read. In test mode (no CLI args), all fields default.

## 6. Asset System (src/core/assets/)

### 6.1 AssetsRootStrategy

```rust
#[derive(Resource, Debug, Clone)]
pub enum AssetsRootStrategy {
    BevyDefault,                   // let Bevy resolve ./assets/ automatically
    Explicit(PathBuf),             // developer-supplied path
    SearchPaths(Vec<PathBuf>),     // first existing directory wins
}
```

Resolution priority (highest to lowest):

1. `BEVY_LAUNCHPAD_ASSETS_ROOT` env var — checked in `AssetsRootStrategy::default()`.

2. Builder `.with_assets_root()` — explicit developer override.

3. `SearchPaths` — iterated in order; first existing directory wins.

4. `BevyDefault` — Bevy's `AssetServer` resolves `./assets/` relative to executable.

### 6.2 AssetPath

```rust
pub enum AssetPath {
    Project(String),         // relative to assets/ — e.g. "branding/logo.png"
    Embedded(&'static str),  // embedded:// URI — e.g. "embedded://bevy_launchpad/..."
    Absolute(PathBuf),       // absolute filesystem path (mods, env var, editor)
}
```

### 6.3 Embedded Assets

The following assets are always registered via `bevy::asset::embedded_asset!`:

* `embedded://bevy_launchpad/fonts/NotoSans-Regular.ttf`

* `embedded://bevy_launchpad/fonts/NotoSans-Bold.ttf`

* `embedded://bevy_launchpad/branding/default_splash.png`

**FIXED:** `register_embedded_assets()` was called unconditionally, embedding the font files into every binary regardless of the `embedded_assets` feature flag. The macro call is now guarded by `#[cfg(feature = "embedded_assets")]`. Without the feature the URIs still exist in the asset registry but resolve to filesystem paths instead of binary data.

### 6.4 Asset Loading & Tracking

```rust
#[derive(Resource, Debug, Default)]
pub struct AssetTracker {
    pub total_assets:  usize,
    pub loaded_assets: usize,
}
impl AssetTracker {
    pub fn progress(&self) -> f32 { /* 0.0 → 1.0 */ }
    pub fn is_ready(&self) -> bool { self.progress() >= 1.0 }
}
```

User code registers handles during Booting/Loading. The framework polls `is_ready()` each frame during Loading and advances to Splash when it returns true.

#### Registering assets in user code

```rust
fn load_assets(
    mut tracker: ResMut<AssetTracker>,
    asset_server: Res<AssetServer>,
    mut handles: ResMut<MyHandles>,
) {
    handles.texture = asset_server.load("sprites/player.png");
    tracker.total_assets += 1;
}

fn check_assets(
    mut tracker: ResMut<AssetTracker>,
    asset_server: Res<AssetServer>,
    handles: Res<MyHandles>,
) {
    if asset_server.is_loaded_with_dependencies(&handles.texture) {
        tracker.loaded_assets += 1;
    }
}
```

## 7. Splash Screen System

### 7.1 SplashConfig

```rust
#[derive(Resource, Debug, Clone, Default)]
pub struct SplashConfig {
    pub screens:               Vec<SplashScreenConfig>,
    pub show_default_branding: bool,   // default: true
    pub skip_all:              bool,   // default: false
}
```

`SplashConfig::effective_screens()` applies all rules and returns the screens that will actually render:

1. If `skip_all` is true or `BEVY_LAUNCHPAD_SKIP_SPLASH` is set → empty vec.

2. If `screens` is non-empty → screens as-is.

3. If `screens` is empty and `show_default_branding` → single default branding screen.

4. Otherwise → empty vec (splash state is skipped immediately).

### 7.2 SplashScreenConfig

| Field | Type | Description |
| :---- | :---- | :---- |
| `source` | `SplashSource` | Image origin: `File(path)`, `Embedded`, or `ColorOnly` |
| `min_duration` | `f32` | Seconds the screen is visible before it can be skipped |
| `max_duration` | `Option<f32>` | Auto-advance after N seconds; `None` = wait for input |
| `skip` | `SkipTrigger` | `AnyInput` \| `EscapeOnly` \| `None` (legal screens) |
| `fade_in` | `f32` | Fade-in duration in seconds |
| `fade_out` | `f32` | Fade-out duration in seconds |
| `background` | `Color` | Background fill color |

### 7.3 Factory Constructors

| Constructor | min/max | skip | fade | bg |
| :---- | :---- | :---- | :---- | :---- |
| `SplashScreenConfig::legal(path)` | 5.0 / 5.0 | `None` | 0.5 / 0.5 | `WHITE` |
| `SplashScreenConfig::studio(path)` | 2.0 / 3.0 | `AnyInput` | 0.4 / 0.4 | `BLACK` |
| `SplashScreenConfig::engine(path)` | 1.5 / 2.5 | `AnyInput` | 0.3 / 0.3 | `BLACK` |
| `SplashScreenConfig::default_branding()` | 1.5 / 2.5 | `AnyInput` | 0.3 / 0.3 | `BLACK` |

### Builder methods on SplashScreenConfig

```rust
SplashScreenConfig::studio("branding/publisher.png")
    .with_duration(2.5, 4.0)       // min_duration, max_duration
    .with_fade(0.5, 0.5)           // fade_in, fade_out
    .skippable(SkipTrigger::None)  // override skip trigger
    .on_background(Color::WHITE)   // background color
```

### 7.4 Splash Renderer

The `SplashRenderer` system (`src/ui/splash/renderer.rs`) manages the per-frame lifecycle of each screen through three states:

| SplashState | Timer duration | Alpha |
| :---- | :---- | :---- |
| `FadeIn` | `screen_cfg.fade_in` | `timer.fraction()` (0→1) |
| `Visible` | `max_duration - fade_in - fade_out` | 1.0 |
| `FadeOut` | `screen_cfg.fade_out` | `1.0 - timer.fraction()` (1→0) |

**FIXED:** `SplashScreen` entities and `SplashTimer` were not cleaned up when leaving `Splash` state. `OnExit(S::splash())` now despawns all `SplashScreen` entities and removes the `SplashTimer` resource. Additionally, the state machine no longer stays permanently in `Splash` when `screens` is empty — it immediately sends a transition to `Menu`.

**FIXED:** The renderer transitions to `Menu` state by sending `TransitionStateEvent` after the last screen's `FadeOut` completes, instead of relying on manual state setting that was missing from the original code.

## 8. UI System (src/ui/ — feature = "ui")

### 8.1 Theme

#### ThemeConfig

```rust
#[derive(Resource, Debug, Clone, Default)]
pub struct ThemeConfig {
    pub colors:  ThemeColors,
    pub fonts:   ThemeFonts,
    pub spacing: ThemeSpacing,
}

impl ThemeConfig {
    pub fn dark() -> Self { /* default dark palette */ }
    pub fn light() -> Self { /* light palette */ }
    pub fn dark_with_noto() -> Self { /* dark + NotoSans embedded */ }
}
```

#### ThemeColors

| Field | Dark default | Light default |
| :---- | :---- | :---- |
| primary | `srgb(0.2, 0.4, 0.8)` | `srgb(0.1, 0.3, 0.7)` |
| secondary | `srgb(0.4, 0.6, 0.9)` | `srgb(0.3, 0.5, 0.8)` |
| background | `srgb(0.05, 0.05, 0.05)` | `WHITE` |
| text | `WHITE` | `BLACK` |

#### ThemeFonts

| Preset | regular | bold | Use when |
| :---- | :---- | :---- | :---- |
| `ThemeFonts::bevy_default()` | `None` (FiraMono) | `None` (FiraMono) | ASCII-only, zero binary cost |
| `ThemeFonts::noto_sans()` | `embedded://...NotoSans-Regular` | `embedded://...NotoSans-Bold` | Cyrillic, CJK, full Unicode |
| `ThemeFonts::from_project(r, b)` | `Project(r)` | `Project(b)` | Custom game font from `assets/` |

#### ThemeSpacing

```rust
pub struct ThemeSpacing {
    pub padding:       f32,   // default: 16.0
    pub margin:        f32,   // default: 8.0
    pub corner_radius: f32,   // default: 4.0
}
```

### 8.2 Main Menu

```rust
#[derive(Resource, Debug, Clone)]
pub struct MainMenuConfig {
    pub title:   String,
    pub buttons: Vec<MenuButton>,
}

pub enum MenuButton {
    Play,
    Settings,
    Exit,
    Custom { label: String, target_state_name: String },
}
```

The main menu is spawned via `OnEnter(S::menu())` and cleaned up via `OnExit(S::menu())`. Custom buttons receive an `OnCustomMenuButtonPressed` event that user code can handle to drive state transitions.

**FIXED:** Custom `MenuButton` variants previously had no mechanism to actually trigger a state transition. A new event `CustomMenuButtonPressed { state_name: String }` is now emitted when a custom button is clicked. User code listens for this event and calls `next_state.set()` accordingly.

### 8.3 Pause Menu

Spawned by `OnEnter(S::paused())`, despawned by `OnExit(S::paused())`. Renders a semi-transparent overlay. Provides Resume and Quit to Menu buttons.

### 8.4 Settings Panel

Accessible via the Settings button in the main menu or pause menu. Contains tabbed sub-panels:

* Graphics — resolution dropdown, fullscreen checkbox, VSync checkbox.

* Audio — master, music, and SFX volume sliders.

* Controls — key binding list (placeholder for user extension).

Settings are persisted to `AppPaths::settings_file` (RON format) via the persistence utilities.

### 8.5 Widgets

| Widget | Component | Spawn function |
| :---- | :---- | :---- |
| Button | `Button` (re-export from bevy) | `spawn_button(parent, label, theme)` |
| Slider | `Slider { min, max, value }` | `spawn_slider(parent, min, max, val, theme)` |
| Checkbox | `Checkbox { checked }` | `spawn_checkbox(parent, label, checked, theme)` |
| Dropdown | `Dropdown { options, selected_index }` | `spawn_dropdown(parent, opts, idx, theme)` |

## 9. Localization (src/locale/ — feature = "locale")

### 9.1 Language Resource

```rust
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Language {
    #[default] En,
    Ru, De, Fr, Es, Ja,   // Ja added in this revision
}

impl Language {
    pub fn as_locale_str(&self) -> &'static str { /* "en-US", "ru-RU", "de-DE", ... */ }
}

impl FromStr for Language {
    // Parses "en", "en-US", "ru-RU", "ja", "ja-JP", etc.
}
```

**FIXED:** `Language::Ja` (Japanese) was missing from the enum despite the `locale_switching` example referencing it and providing `ja-JP` Fluent files. Added `Ja` variant with locale string "ja-JP".

### 9.2 Fluent Bundle Loader

When `LocalizationPlugin` is active, the framework watches for changes to the `Language` resource and reloads the Fluent bundle from:

`assets/locales/<locale-str>/main.ftl`

Additional `.ftl` files in the locale directory are loaded automatically and merged into the bundle.

### 9.3 Switching Language at Runtime

```rust
fn switch_locale(keys: Res<ButtonInput<KeyCode>>, mut lang: ResMut<Language>) {
    if keys.just_pressed(KeyCode::Digit1) { *lang = Language::En; }
    if keys.just_pressed(KeyCode::Digit2) { *lang = Language::Ru; }
    if keys.just_pressed(KeyCode::Digit3) { *lang = Language::Ja; }
}
```

The framework detects the resource change and hot-reloads the bundle on the next frame.

### 9.4 Locale Directory Layout

```text
assets/
└── locales/
    ├── en-US/
    │   └── main.ftl
    ├── ru-RU/
    │   └── main.ftl
    └── ja-JP/
        └── main.ftl
```

## 10. Utilities (src/utils/)

### 10.1 Persistence

```rust
// Free functions
pub fn load_ron<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, Box<dyn Error>>;
pub fn save_ron<T: Serialize>(path: impl AsRef<Path>, data: &T) -> Result<(), Box<dyn Error>>;

// Convenience trait
pub trait Persistable: Serialize + DeserializeOwned + Sized {
    fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>>;
    fn save(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>>;
}
```

Example usage with a custom settings struct:

```rust
#[derive(Resource, Serialize, Deserialize)]
pub struct GameplaySettings { pub difficulty: Difficulty, pub camera_shake: bool }

impl Persistable for GameplaySettings {}

fn save_gameplay(settings: Res<GameplaySettings>, paths: Res<AppPaths>) {
    if let Err(e) = settings.save(paths.data_dir.join("gameplay.ron")) {
        error!("Failed to save settings: {e}");
    }
}
```

### 10.2 Platform Paths

| Platform | Data directory |
| :---- | :---- |
| Windows | `%APPDATA%\<app_name>` |
| macOS | `~/Library/Application Support/<app_name>` |
| Linux | `$XDG_DATA_HOME/<app_name>` (fallback: `~/.local/share/<app_name>`) |
| Other | `./data/<app_name>` |

### 10.3 Dead Code Removal

**FIXED:** The following placeholder files contain only `pub fn mod_placeholder() {}` and have no callers. They should either be implemented or removed before the 1.0 release:

* `src/locale/fluent.rs` — implement Fluent bundle integration or delete.

* `src/locale/utils.rs` — implement locale utilities or delete.

* `src/ui/transitions/slide.rs` — implement slide transition or delete.

* `src/ui/transitions/zoom.rs` — implement zoom transition or delete.

* `src/ui/menu/settings/controls.rs` — implement key bindings or delete.

* `src/ui/menu/settings/general.rs` — implement general settings or delete.

* `src/ui/modal/alert.rs / confirm.rs` — implement or delete.

* `src/utils/validation.rs` — implement or delete.

* `src/utils/singleton.rs` — `StateMachine<S>` and `SingleInstance` are defined but unused; remove or wire up.

## 11. Bug Fixes & Changes Summary

This section consolidates all corrections applied in this specification revision.

| ID | Severity | Component | Description | Resolution |
| :---- | :---- | :---- | :---- | :---- |
| BUG-01 | Critical | Loading transition | `auto_transition_loading` called `next_state.set()` every frame with no guard, causing repeated state sets | Guard with `AssetTracker::is_ready()` and a one-shot boolean flag |
| BUG-02 | High | Splash cleanup | `SplashScreen` entities and `SplashTimer` not removed on `OnExit(Splash)` | Register `OnExit(S::splash())` cleanup system in `LaunchpadUiPlugin` |
| BUG-03 | High | Splash → Menu | No system advanced state from `Splash` to `Menu` after screens finished | `SplashRenderer` sends `TransitionStateEvent { next: S::menu() }` after last `FadeOut` |
| BUG-04 | High | macOS single-instance | `/proc/<pid>` check used on macOS where `/proc` does not exist | Add `#[cfg(target_os = "macos")]` branch using `kill(pid, 0)` via `libc` |
| BUG-05 | Medium | Embedded assets | `register_embedded_assets()` always ran, inflating binaries without `embedded_assets` feature | `#[cfg(feature = "embedded_assets")]` guard around the macro calls |
| BUG-06 | Medium | Custom menu buttons | `MenuButton::Custom` had no transition mechanism | Emit `CustomMenuButtonPressed` event; user handles with `next_state.set()` |
| BUG-07 | Medium | Locale enum | `Language::Ja` missing despite `ja-JP` FTL files and `locale_switching` example | Add `Ja` variant with `as_locale_str() = "ja-JP"` and `FromStr` support |
| BUG-08 | Low | Cargo.toml | `examples/custom_theme/` not registered as `[[example]]` | Add `[[example]]` entry with `name = "custom_theme"`, `required-features = ["ui"]` |
| BUG-09 | Low | Dead code | Multiple placeholder source files with no implementation | Track in §10.3; remove before 1.0 or replace with stubs that `panic!("not yet implemented")` |

## 12. Examples Reference

| Example | Features | Demonstrates |
| :---- | :---- | :---- |
| `minimal_2d` | `2d` | Zero-config 2D game, `AppState::default()` |
| `minimal_3d` | `3d` | Zero-config 3D game with `AppMetadata` |
| `full_2d` | `ui`, `locale`, `2d` | Full pipeline: splash, menu, locale, 2D gameplay, pause |
| `full_3d` | `ui`, `locale`, `3d` | AAA four-screen splash, custom fonts, 3D rendering |
| `custom_theme_dark` | `ui` | Hot-pink cyberpunk `ThemeConfig`, NotoSans fonts |
| `custom_theme_light` | `ui` | `ThemeConfig::light()`, color-only splash |
| `custom_theme` | `ui` | Inline `ThemeConfig` with custom palette |
| `custom_menu` | `ui` | Extra `MenuButton::Custom` entries, `Credits` state |
| `custom_settings` | `ui` | Serializable `GameplaySettings` resource, save on change |
| `custom_assets_root` | `ui` | `AssetsRootStrategy::Explicit("game_data/")` |
| `color_splash` | `ui` | Multiple `ColorOnly` splash screens, no images needed |
| `no_splash` | `ui` | `screens: []`, `show_default_branding: false` |
| `embedded_only` | `ui`, `embedded_assets` | `SplashSource::Embedded`, NotoSans embedded |
| `pause_menu` | `ui` | Bouncing ball, Escape toggles `AppState::Paused` |
| `dev_mode` | `ui` | `--skip-splash`, `--state Playing` CLI flags |
| `locale_switching` | `ui`, `locale` | Runtime `Language` switch (EN / RU / JA) |
| `mod_support` | `ui` | `AssetsRootStrategy::SearchPaths` for DLC and mods |
| `multi_instance` | `ui` | `allow_multiple_instances(true)` |
| `no_ui` | (none) | Headless server, `MinimalPlugins`, no renderer |

## 13. Integration Guide

### 13.1 Minimal Setup

```rust
// Cargo.toml
[dependencies]
bevy-launchpad = "0.2"

// main.rs
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LaunchpadPlugin::<AppState>::default())
        .add_systems(OnEnter(AppState::Playing), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
```

### 13.2 Custom State Enum

```rust
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[derive(LaunchpadStates)]
enum GameState {
    #[default]
    Booting, Loading, Splash, Menu, Playing, Paused,
    // Custom states — add as many as you need
    Credits,
    LevelSelect,
    Cutscene,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_metadata(AppMetadata {
                    name: "mygame".into(),
                    title: "My Awesome Game".into(),
                    version: "0.1.0".into(),
                    ..default()
                })
                .with_splash(SplashConfig {
                    screens: vec![
                        SplashScreenConfig::studio("branding/studio.png"),
                        SplashScreenConfig::engine("branding/engine.png"),
                    ],
                    ..default()
                })
                .with_theme(ThemeConfig::dark_with_noto())
                .with_main_menu(MainMenuConfig {
                    title: "My Game".into(),
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
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .run();
}
```

### 13.3 Adding Custom Assets to Loading

```rust
fn start_loading(
    mut tracker:      ResMut<AssetTracker>,
    asset_server:     Res<AssetServer>,
    mut my_handles:   ResMut<MyAssetHandles>,
) {
    my_handles.player = asset_server.load("sprites/player.png");
    my_handles.music  = asset_server.load("audio/theme.ogg");
    tracker.total_assets += 2;
}

fn check_loaded(
    mut tracker:  ResMut<AssetTracker>,
    asset_server: Res<AssetServer>,
    handles:      Res<MyAssetHandles>,
) {
    if asset_server.is_loaded_with_dependencies(&handles.player) { tracker.loaded_assets += 1; }
    if asset_server.is_loaded_with_dependencies(&handles.music)  { tracker.loaded_assets += 1; }
}

// Register both systems in Loading state
app.add_systems(OnEnter(GameState::Loading), start_loading);
app.add_systems(Update, check_loaded.run_if(in_state(GameState::Loading)));
```

### 13.4 Handling Custom Menu Buttons

```rust
fn handle_custom_buttons(
    mut events:    EventReader<CustomMenuButtonPressed>,
    mut next:      ResMut<NextState<GameState>>,
) {
    for ev in events.read() {
        match ev.state_name.as_str() {
            "Credits"     => next.set(GameState::Credits),
            "LevelSelect" => next.set(GameState::LevelSelect),
            _             => warn!("Unknown custom state: {}", ev.state_name),
        }
    }
}
```
