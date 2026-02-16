# 🚀 Bevy Launchpad

[![Crates.io](https://img.shields.io/crates/v/bevy_launchpad.svg)](https://crates.io/crates/bevy_launchpad)
[![Docs.rs](https://docs.rs/bevy_launchpad/badge.svg)](https://docs.rs/bevy_launchpad)
[![CI](https://github.com/teratron/bevy-launchpad/workflows/CI/badge.svg)](https://github.com/teratron/bevy-launchpad/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](LICENSE)
[![Bevy](https://img.shields.io/badge/Bevy-0.18-blue)](https://bevyengine.org)

**Production-ready launcher framework for Bevy** - includes splash screens, main menu, settings UI, localization, and smooth state transitions out of the box.

Stop building the same infrastructure for every game. Focus on gameplay, not boilerplate.

## ✨ Features

- 🎬 **Splash Screen System** - Branding and loading screens with fade transitions
- 📋 **Main Menu** - Professional menu with customizable buttons and layouts
- ⚙️ **Settings UI** - Complete settings system (graphics, audio, controls, general)
  - Graphics: Quality presets, resolution, fullscreen, VSync
  - Audio: Master/Music/SFX volume controls
  - Controls: Rebindable key mappings
  - General: Language selection, theme switching
- 🌍 **Localization** - Built-in Fluent integration with runtime language switching
- 🎨 **Theming** - Customizable color schemes and fonts (dark/light themes included)
- ⏸️ **Pause Menu** - In-game pause system with settings access
- 🔄 **State Management** - Robust state machine with smooth transitions
- 💾 **Settings Persistence** - Automatic save/load with validation
- 🎯 **Modal Dialogs** - Confirmation dialogs for critical actions
- 📦 **Asset Loading** - Progress tracking and manifest system
- 🛡️ **Single Instance Lock** - Prevent multiple game instances (optional)
- 🎮 **2D & 3D Ready** - Works with both 2D and 3D games
- 🔧 **Diagnostics** - Optional FPS and debug overlay

## 🚀 Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
bevy = "0.18"
bevy_launchpad = "0.1"
```

### Minimal Example (2D)

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Booting,
    Menu,
    Playing,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LaunchpadPlugin::<GameState>::default())
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .run();
}

fn setup_game(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d);
    
    // Your game setup here
    info!("Game started!");
}
```

That's it! You now have:

- ✅ Boot sequence with initialization
- ✅ Splash screen
- ✅ Main menu with Play/Settings/Exit
- ✅ Settings panel with graphics/audio controls
- ✅ Localization support
- ✅ State management

### With Custom Configuration

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_splash(SplashConfig {
                    logo: "branding/logo.png",
                    duration: 2.0,
                    skippable: true,
                })
                .with_localization(LocaleConfig {
                    default: "en-US",
                    supported: vec!["en-US", "ru-RU", "ja-JP"],
                })
                .with_theme(ThemeConfig::dark())
                .build()
        )
        .run();
}
```

## 📚 Examples

Run the examples to see Bevy Launchpad in action:

```bash
# Minimal 2D game (~50 lines)
cargo run --example minimal_2d

# Minimal 3D game (~60 lines)
cargo run --example minimal_3d

# Full-featured 2D game with all features
cargo run --example full_2d

# Full-featured 3D game
cargo run --example full_3d

# Custom theme example
cargo run --example custom_theme
```

## 🎮 Creating a New Game

### Option 1: Using cargo-generate

```bash
# Install cargo-generate
cargo install cargo-generate

# Create new 2D game
cargo generate --git https://github.com/teratron/bevy-launchpad \
                --name my-game \
                templates/game-2d

# Create new 3D game
cargo generate --git https://github.com/teratron/bevy-launchpad \
                --name my-game \
                templates/game-3d

cd my-game
cargo run
```

### Option 2: Manual Setup

1. Add dependency to `Cargo.toml`
2. Copy `src/main.rs` from examples
3. Add your assets to `assets/`
4. Implement your gameplay!

## 🎨 Customization

### Custom Theme

```rust
use bevy_launchpad::ui::theme::*;

let my_theme = ThemeColors {
    background: Color::srgb(0.1, 0.05, 0.15),
    surface: Color::srgb(0.15, 0.1, 0.2),
    accent: Color::srgb(1.0, 0.3, 0.5),
    text_primary: Color::srgb(0.95, 0.95, 0.95),
    // ... other colors
};

App::new()
    .add_plugins(LaunchpadPlugin::default())
    .insert_resource(my_theme)
    .run();
```

### Custom Menu Items

```rust
LaunchpadPlugin::<GameState>::builder()
    .with_main_menu(MainMenuConfig {
        buttons: vec![
            MenuButton::Play,
            MenuButton::Custom("Level Select", GameState::LevelSelect),
            MenuButton::Settings,
            MenuButton::Custom("Credits", GameState::Credits),
            MenuButton::Exit,
        ],
    })
    .build()
```

### Add Custom Settings Tab

```rust
use bevy_launchpad::ui::settings::*;

// Implement your custom settings
#[derive(Resource, Serialize, Deserialize, Clone)]
struct GameplaySettings {
    difficulty: Difficulty,
    auto_save: bool,
}

// Register in app
app.insert_resource(GameplaySettings::default())
   .add_systems(Update, save_gameplay_settings);
```

## 📖 Documentation

- [Getting Started Guide](https://github.com/teratron/bevy-launchpad/blob/main/docs/getting-started.md)
- [Architecture Overview](https://github.com/teratron/bevy-launchpad/blob/main/docs/architecture.md)
- [API Documentation](https://docs.rs/bevy_launchpad)
- [Theming Guide](https://github.com/teratron/bevy-launchpad/blob/main/docs/theming.md)
- [Localization Guide](https://github.com/teratron/bevy-launchpad/blob/main/docs/localization.md)
- [State Management](https://github.com/teratron/bevy-launchpad/blob/main/docs/state-management.md)

## 🏗️ Architecture

Bevy Launchpad is built as a modular workspace:

```plaintext
bevy-launchpad/
├── bevy_launchpad          # Main facade crate
├── bevy_launchpad_core     # Core systems (boot, states, loading)
├── bevy_launchpad_ui       # UI components (menu, settings, widgets)
├── bevy_launchpad_locale   # Localization system
└── bevy_launchpad_utils    # Utilities (singleton, validation)
```

Each crate can be used independently if you only need specific functionality:

```toml
# Use only core features
[dependencies]
bevy_launchpad_core = "0.1"

# Use only UI components
[dependencies]
bevy_launchpad_ui = "0.1"
```

## 🔧 Feature Flags

```toml
[dependencies]
bevy_launchpad = { version = "0.1", features = ["full"] }

# Or pick specific features:
bevy_launchpad = { 
    version = "0.1", 
    default-features = false,
    features = ["ui", "locale", "2d"]
}
```

Available features:

- `ui` - UI components (menu, settings, widgets)
- `locale` - Localization system
- `2d` - 2D-specific features
- `3d` - 3D-specific features
- `diagnostics` - FPS and debug overlay
- `full` - All features (default)

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/teratron/bevy-launchpad
cd bevy-launchpad

# Run tests
cargo test --workspace

# Run examples
cargo run --example minimal_2d

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --workspace -- -D warnings
```

### Project Goals

- 🎯 **Production-ready** - Used in real games, not just a prototype
- 🧩 **Modular** - Use what you need, ignore the rest
- 📚 **Well-documented** - Clear examples and API docs
- 🔒 **Stable** - Semantic versioning, minimal breaking changes
- ⚡ **Performant** - No unnecessary overhead
- 🌍 **Accessible** - I18n and a11y built-in

## 🗺️ Roadmap

### v0.1.0 (Current)

- [x] Core boot sequence
- [x] Main menu system
- [x] Settings UI (graphics, audio, controls)
- [x] Localization (Fluent)
- [x] Theme system
- [x] 2D/3D examples

### v0.2.0 (Planned)

- [ ] More settings tabs (accessibility, gameplay)
- [ ] Achievement/notification system
- [ ] Save/load system integration
- [ ] More theme presets
- [ ] Additional widgets (checkbox, radio, tabs)

### v0.3.0 (Future)

- [ ] Multiplayer lobby UI
- [ ] Cloud save integration hooks
- [ ] Steam/Epic integration helpers
- [ ] Mobile platform support

See [ROADMAP.md](ROADMAP.md) for detailed plans.

## 📊 Compatibility

| Bevy Launchpad | Bevy Version   |
|----------------|----------------|
| 0.1.x          | 0.18           |
| 0.2.x          | 0.19 (planned) |

## 🙏 Acknowledgments

Built with:

- [Bevy](https://bevyengine.org) - A refreshingly simple data-driven game engine
- [Fluent](https://projectfluent.org) - Localization system

Inspired by professional game launchers and the Bevy community.

## 📜 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## 🌟 Show Your Support

If Bevy Launchpad helps your project, please ⭐ star the repository!

---

Made with ❤️ for the **Bevy community**
