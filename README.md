# 🚀 Bevy Launchpad

[![Crates.io](https://img.shields.io/crates/v/bevy_launchpad.svg)](https://crates.io/crates/bevy-launchpad)
[![Docs.rs](https://docs.rs/bevy-launchpad/badge.svg)](https://docs.rs/bevy-launchpad)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](LICENSE)
[![Bevy](https://img.shields.io/badge/Bevy-0.18-blue)](https://bevyengine.org)

**Production-ready launcher framework for Bevy** - includes splash screens, main menu, settings UI, localization, and smooth state transitions out of the box.

## ✨ Features

## 🚀 Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
bevy = { version = "0.18", default-features = false, features = ["bevy_winit", "x11"] } # Adjust features as needed
bevy-launchpad = { version = "0.1", features = ["ui", "2d"] }
```

## 📚 Examples

Run the examples to see Bevy Launchpad in action:

```bash
# Minimal 2D game (~50 lines)
cargo run --example minimal_2d

# Minimal 3D game (~60 lines)
cargo run --example minimal_3d
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

### Custom Menu Items

### Add Custom Settings Tab

## 📖 Documentation

- [Getting Started Guide](https://github.com/teratron/bevy-launchpad/blob/main/docs/getting-started.md)
- [Architecture Overview](https://github.com/teratron/bevy-launchpad/blob/main/docs/architecture.md)
- [API Documentation](https://docs.rs/bevy-launchpad)
- [Theming Guide](https://github.com/teratron/bevy-launchpad/blob/main/docs/theming.md)
- [Localization Guide](https://github.com/teratron/bevy-launchpad/blob/main/docs/localization.md)
- [State Management](https://github.com/teratron/bevy-launchpad/blob/main/docs/state-management.md)

## 🏗️ Architecture

## 🔧 Feature Flags

Optimize your build by enabling only what you need in `Cargo.toml`:

```toml
[dependencies]
bevy-launchpad = { 
    version = "0.1", 
    default-features = false,
    features = [
        "ui",       # Enable UI components (menus, widgets, theming)
        "locale",   # Enable localization support
        "2d",       # Enable 2D-specific features (sprites, cameras)
        "3d",       # Enable 3D-specific features (PBR, cameras)
    ]
}
```

### Cross-Platform Support

Bevy Launchpad now supports Windows, Linux, and macOS out of the box. The `x11` feature is optional for Linux users who prefer it over Wayland.

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

## 📊 Compatibility

| Bevy Launchpad | Bevy Version   |
|----------------|----------------|
| 0.1.x          | 0.18           |
| 0.2.x          | 0.19 (planned) |

## 🙏 Acknowledgments

Built with:

- [Rust](https://www.rust-lang.org/) - A fast, safe, and concurrent systems programming language
- [Bevy](https://bevyengine.org) - A refreshingly simple data-driven game engine
- [Fluent](https://projectfluent.org) - Localization system

Inspired by professional game launchers and the Bevy community.

## 📜 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

## 🌟 Show Your Support

If Bevy Launchpad helps your project, please ⭐ star the repository!

---

Made with ❤️ for the **Bevy community**
