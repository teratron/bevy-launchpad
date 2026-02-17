# bevy-launchpad — State Integration Specification

> **Stack:** Rust 1.93 · Bevy 0.18 · bevy_launchpad 0.1  
> **Scope:** This document specifies the complete design of the `LaunchpadStates`
> trait, the `#[derive(LaunchpadStates)]` proc-macro, the built-in `AppState`
> enum, and how all three levels of the API fit together.

---

## Table of Contents

1. [Problem Statement](#1-problem-statement)
2. [Solution Overview](#2-solution-overview)
3. [Repository Layout Changes](#3-repository-layout-changes)
4. [Level 1 — Built-in AppState](#4-level-1--built-in-appstate)
5. [Level 2 — derive + naming convention](#5-level-2--derive--naming-convention)
6. [Level 3 — derive + explicit attributes](#6-level-3--derive--explicit-attributes)
7. [LaunchpadStates Trait](#7-launchpadstates-trait)
8. [bevy_launchpad_derive crate](#8-bevy_launchpad_derive-crate)
9. [Cargo.toml changes](#9-cargotoml-changes)
10. [prelude.rs changes](#10-preluders-changes)
11. [Error messages](#11-error-messages)
12. [Full usage examples](#12-full-usage-examples)
13. [Decision log](#13-decision-log)

---

## 1. Problem Statement

The original API forces the developer to write ~20 lines of identical boilerplate
every time they create a game:

```rust
// CURRENT — repetitive, noise, easy to get wrong
impl LaunchpadStates for GameState {
    fn booting()  -> Self { Self::Booting  }
    fn loading()  -> Self { Self::Loading  }
    fn splash()   -> Self { Self::Splash   }
    fn menu()     -> Self { Self::Menu     }
    fn playing()  -> Self { Self::Playing  }
    fn paused()   -> Self { Self::Paused   }
}
```

**Goals:**

- Level 1: zero lines for simple games (use a built-in enum).
- Level 2: one line (`#[derive(LaunchpadStates)]`) when variant names follow
  the library convention.
- Level 3: one line + per-variant attributes when names differ from convention.
- Clear, actionable compile-time errors when the mapping is incomplete.
- All three levels must coexist — a developer can start at Level 1 and migrate
  to Level 2 without changing any other code.

---

## 2. Solution Overview

| Level | What developer writes | Use case |
|---|---|---|
| 1 | Nothing — use `AppState` directly | prototypes, game jams, testing |
| 2 | `#[derive(LaunchpadStates)]` + conventional variant names | most indie games |
| 3 | `#[derive(LaunchpadStates)]` + `#[launchpad(...)]` attributes | legacy naming, personal style |

All three levels produce the same runtime behavior. They differ only in the
amount of code the developer writes.

---

## 3. Repository Layout Changes

```
bevy-launchpad/
├── Cargo.toml                           ← add bevy_launchpad_derive dependency
├── src/
│   ├── lib.rs                           ← re-export AppState and the macro
│   ├── prelude.rs                       ← add AppState + LaunchpadStates re-export
│   └── core/
│       └── states/
│           ├── mod.rs
│           ├── app_state.rs             ← NEW: built-in AppState enum
│           ├── mapping.rs               ← existing LaunchpadStates trait (unchanged)
│           ├── machine.rs
│           └── transitions.rs
│
└── bevy_launchpad_derive/               ← NEW: proc-macro crate
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

---

## 4. Level 1 — Built-in AppState

The library ships a ready-to-use state enum. The developer imports it and passes
it to `LaunchpadPlugin`. No trait impl required.

### `src/core/states/app_state.rs`

```rust
use bevy::prelude::*;
use crate::core::states::mapping::LaunchpadStates;
use bevy::state::state::FreelyMutableState;

/// Built-in application state enum provided by bevy_launchpad.
///
/// Use this when you do not need custom states beyond the standard
/// boot → loading → splash → menu → playing → paused flow.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_launchpad::prelude::*;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(LaunchpadPlugin::default())   // uses AppState internally
///         .add_systems(OnEnter(AppState::Playing), setup)
///         .run();
/// }
/// ```
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    /// Framework initialises here (paths, lock, CLI args).
    #[default]
    Booting,
    /// Asset manifests are being loaded.
    Loading,
    /// Splash screen sequence is playing.
    Splash,
    /// Main menu is active.
    Menu,
    /// Gameplay is running.
    Playing,
    /// Gameplay is paused.
    Paused,
}

impl LaunchpadStates for AppState {
    fn booting()  -> Self { Self::Booting  }
    fn loading()  -> Self { Self::Loading  }
    fn splash()   -> Self { Self::Splash   }
    fn menu()     -> Self { Self::Menu     }
    fn playing()  -> Self { Self::Playing  }
    fn paused()   -> Self { Self::Paused   }
}
```

### `LaunchpadPlugin` — default type parameter

```rust
// src/lib.rs

/// LaunchpadPlugin defaults to AppState when no type parameter is given.
/// Both forms below are equivalent for simple games:
///
///   LaunchpadPlugin::default()
///   LaunchpadPlugin::<AppState>::default()
pub struct LaunchpadPlugin<S: LaunchpadStates = AppState> { ... }

impl LaunchpadPlugin<AppState> {
    /// Convenience constructor — no type annotation needed.
    pub fn default() -> Self { ... }
}
```

### Developer code (Level 1)

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LaunchpadPlugin::default())        // AppState implied
        .add_systems(OnEnter(AppState::Playing), setup) // use AppState variants directly
        .run();
}
```

---

## 5. Level 2 — derive + naming convention

The developer declares their own enum. The `#[derive(LaunchpadStates)]` macro
inspects variant names and generates the `impl LaunchpadStates` automatically.

### Naming convention

| Variant name (exact, case-sensitive) | Generated method |
|---|---|
| `Booting` | `fn booting()` |
| `Loading` | `fn loading()` |
| `Splash`  | `fn splash()`  |
| `Menu`    | `fn menu()`    |
| `Playing` | `fn playing()` |
| `Paused`  | `fn paused()`  |

Any variant whose name is **not** in this table is silently ignored — it becomes
a developer-owned state that the library never touches.

### Developer code (Level 2)

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[derive(LaunchpadStates)]   // single line replaces the 20-line manual impl
enum GameState {
    #[default]
    Booting,
    Loading,
    Splash,
    Menu,
    Playing,
    Paused,
    // Custom states — the macro ignores these completely
    Credits,
    LevelSelect,
    GameOver,
    Cutscene,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LaunchpadPlugin::<GameState>::default())
        .add_systems(OnEnter(GameState::Playing),     setup_game)
        .add_systems(OnEnter(GameState::Credits),     setup_credits)
        .add_systems(OnEnter(GameState::LevelSelect), setup_level_select)
        .run();
}
```

---

## 6. Level 3 — derive + explicit attributes

When the developer's variant names do not match the convention, they use
per-variant `#[launchpad(<method>)]` attributes to declare the mapping.

### Attribute syntax

```rust
#[launchpad(booting)]   // maps this variant to fn booting()
#[launchpad(loading)]   // maps this variant to fn loading()
#[launchpad(splash)]    // maps this variant to fn splash()
#[launchpad(menu)]      // maps this variant to fn menu()
#[launchpad(playing)]   // maps this variant to fn playing()
#[launchpad(paused)]    // maps this variant to fn paused()
```

Each attribute maps **exactly one variant** to **exactly one method**.
Duplicate mappings (two variants mapped to the same method) are a compile error.

### Resolution priority

For each required method, the macro resolves in this order:

1. Explicit `#[launchpad(<method>)]` attribute on a variant.
2. Variant name matches the naming convention exactly.
3. → compile error with a helpful message.

### Developer code (Level 3)

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[derive(LaunchpadStates)]
enum MyState {
    #[default]
    #[launchpad(booting)]   // "Init" does not match convention → explicit attr
    Init,

    #[launchpad(loading)]
    AssetsLoading,

    #[launchpad(splash)]
    Intro,

    #[launchpad(menu)]
    MainMenu,

    #[launchpad(playing)]
    InGame,

    #[launchpad(paused)]
    Pause,

    // Custom states — no attribute, ignored by the macro
    Cutscene,
    BossDefeated,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LaunchpadPlugin::<MyState>::default())
        .add_systems(OnEnter(MyState::InGame),   setup_game)
        .add_systems(OnEnter(MyState::Cutscene), play_cutscene)
        .run();
}
```

### Mixed usage (some conventional, some attributed)

Attributes and convention can be mixed freely within the same enum:

```rust
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[derive(LaunchpadStates)]
enum HybridState {
    #[default]
    Booting,              // conventional name → fn booting() — no attr needed
    Loading,              // conventional name → fn loading()
    Splash,               // conventional name → fn splash()

    #[launchpad(menu)]    // non-conventional name → explicit attr
    HomeScreen,

    Playing,              // conventional → fn playing()

    #[launchpad(paused)]  // non-conventional → explicit attr
    TimeStop,

    Inventory,            // custom, ignored
    Credits,              // custom, ignored
}
```

---

## 7. LaunchpadStates Trait

This is the existing trait in `src/core/states/mapping.rs`. **No changes required.**
Documented here for completeness.

```rust
// src/core/states/mapping.rs  (unchanged)
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;

/// Implement this trait on your game's `States` enum to integrate it with
/// bevy_launchpad's automatic boot → loading → splash → menu flow.
///
/// You should never implement this manually — use one of:
///   - `AppState`  (built-in, zero lines)
///   - `#[derive(LaunchpadStates)]`  (one line, automatic)
pub trait LaunchpadStates: States + FreelyMutableState + Default {
    /// Framework initialises here (paths, single-instance lock, CLI args).
    fn booting()  -> Self;
    /// Asset manifests are being loaded.
    fn loading()  -> Self;
    /// Splash screen sequence is active.
    fn splash()   -> Self;
    /// Main menu is active.
    fn menu()     -> Self;
    /// Gameplay is running.
    fn playing()  -> Self;
    /// Gameplay is paused.
    fn paused()   -> Self;
}
```

---

## 8. bevy_launchpad_derive crate

### Directory structure

```
bevy_launchpad_derive/
├── Cargo.toml
└── src/
    └── lib.rs
```

### `bevy_launchpad_derive/Cargo.toml`

```toml
[package]
name        = "bevy_launchpad_derive"
version     = "0.1.0"
edition     = "2024"
rust-version = "1.93"
description = "Procedural macros for bevy_launchpad"
license     = "MIT OR Apache-2.0"
repository  = "https://github.com/teratron/bevy-launchpad"
publish     = true   # published alongside the main crate

[lib]
proc-macro = true

[dependencies]
syn         = { version = "2.0", features = ["full"] }
quote       = "1.0"
proc-macro2 = "1.0"
```

### `bevy_launchpad_derive/src/lib.rs` — complete implementation

```rust
//! Procedural macros for bevy_launchpad.
//!
//! Do not use this crate directly — import via `bevy_launchpad::prelude::*`.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Error, Fields,
    Meta, Result,
};

// ── Naming convention ─────────────────────────────────────────────────────────

/// Maps conventional variant names to LaunchpadStates method names.
/// Key = variant name (exact, case-sensitive).
/// Value = method name.
const CONVENTION: &[(&str, &str)] = &[
    ("Booting", "booting"),
    ("Loading", "loading"),
    ("Splash",  "splash"),
    ("Menu",    "menu"),
    ("Playing", "playing"),
    ("Paused",  "paused"),
];

/// All methods that MUST be mapped for a valid impl.
const REQUIRED_METHODS: &[&str] =
    &["booting", "loading", "splash", "menu", "playing", "paused"];

// ── Entry point ───────────────────────────────────────────────────────────────

/// Derive macro that generates `impl LaunchpadStates for YourEnum`.
///
/// # Variant name convention
///
/// If your variant names match the convention exactly, no attributes are needed:
///
/// ```rust
/// #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
/// #[derive(LaunchpadStates)]
/// enum GameState {
///     #[default]
///     Booting, Loading, Splash, Menu, Playing, Paused,
///     Credits,      // custom — ignored by this macro
///     LevelSelect,  // custom — ignored by this macro
/// }
/// ```
///
/// # Explicit attribute mapping
///
/// When your variant names differ from the convention, annotate them:
///
/// ```rust
/// #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
/// #[derive(LaunchpadStates)]
/// enum MyState {
///     #[default]
///     #[launchpad(booting)] Init,
///     #[launchpad(loading)] Load,
///     #[launchpad(splash)]  Intro,
///     #[launchpad(menu)]    Home,
///     #[launchpad(playing)] InGame,
///     #[launchpad(paused)]  Pause,
///     Cutscene,   // custom
/// }
/// ```
///
/// Convention-based and attribute-based mappings can be mixed freely.
#[proc_macro_derive(LaunchpadStates, attributes(launchpad))]
pub fn derive_launchpad_states(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match impl_launchpad_states(&input) {
        Ok(ts)  => ts.into(),
        Err(e)  => e.to_compile_error().into(),
    }
}

// ── Core logic ────────────────────────────────────────────────────────────────

fn impl_launchpad_states(input: &DeriveInput) -> Result<TokenStream2> {
    let enum_name = &input.ident;

    // Only enums are supported
    let data_enum = match &input.data {
        Data::Enum(e) => e,
        _ => return Err(Error::new_spanned(
            enum_name,
            "#[derive(LaunchpadStates)] can only be applied to enums",
        )),
    };

    // Collect mapping: method_name → variant TokenStream
    // Use IndexMap-style ordered Vec so error messages are deterministic.
    let mut resolved: Vec<(&str, TokenStream2)> = Vec::new();

    // Track which methods have been explicitly attributed (for duplicate detection)
    let mut attributed: std::collections::HashSet<String> =
        std::collections::HashSet::new();

    for variant in &data_enum.variants {
        // Only unit variants are valid (no tuple/struct variants)
        if !matches!(&variant.fields, Fields::Unit) {
            return Err(Error::new_spanned(
                &variant.ident,
                "#[derive(LaunchpadStates)] only supports unit enum variants",
            ));
        }

        let variant_ident = &variant.ident;
        let variant_name  = variant_ident.to_string();

        // --- Pass 1: explicit #[launchpad(<method>)] attribute ---
        for attr in &variant.attrs {
            if !attr.path().is_ident("launchpad") {
                continue;
            }
            let method_ident: syn::Ident = attr.parse_args().map_err(|_| {
                Error::new_spanned(
                    attr,
                    "expected #[launchpad(<method_name>)], e.g. #[launchpad(booting)]",
                )
            })?;
            let method_name = method_ident.to_string();

            // Validate the method name is one of the required set
            if !REQUIRED_METHODS.contains(&method_name.as_str()) {
                return Err(Error::new_spanned(
                    &method_ident,
                    format!(
                        "`{}` is not a valid LaunchpadStates method. \
                         Valid values: {}",
                        method_name,
                        REQUIRED_METHODS.join(", ")
                    ),
                ));
            }

            // Detect duplicate attribute mapping
            if attributed.contains(&method_name) {
                return Err(Error::new_spanned(
                    variant_ident,
                    format!(
                        "duplicate #[launchpad({})] — another variant already \
                         maps to this method",
                        method_name
                    ),
                ));
            }

            attributed.insert(method_name.clone());

            // Remove any convention-based entry for this method (attr wins)
            resolved.retain(|(m, _)| *m != method_name.as_str());

            // Find the static str key
            let method_key = REQUIRED_METHODS
                .iter()
                .find(|&&m| m == method_name.as_str())
                .copied()
                .unwrap();

            resolved.push((method_key, quote! { #enum_name::#variant_ident }));
        }

        // --- Pass 2: naming convention (only if not already attributed) ---
        for &(conv_name, method_name) in CONVENTION {
            if variant_name == conv_name
                && !attributed.contains(method_name)
                && !resolved.iter().any(|(m, _)| *m == method_name)
            {
                resolved.push((method_name, quote! { #enum_name::#variant_ident }));
            }
        }
    }

    // --- Verify all required methods are present ---
    let mut missing: Vec<String> = Vec::new();
    for &method in REQUIRED_METHODS {
        if !resolved.iter().any(|(m, _)| *m == method) {
            let conventional_name = convention_variant_name(method);
            missing.push(format!(
                "  • `{}` — add a variant named `{}` or annotate one with #[launchpad({})]",
                method, conventional_name, method
            ));
        }
    }

    if !missing.is_empty() {
        return Err(Error::new_spanned(
            enum_name,
            format!(
                "#[derive(LaunchpadStates)] is missing mappings for:\n{}\n\n\
                 See bevy_launchpad documentation for examples.",
                missing.join("\n")
            ),
        ));
    }

    // --- Generate impl ---
    let get = |method: &str| -> TokenStream2 {
        resolved.iter()
            .find(|(m, _)| *m == method)
            .map(|(_, ts)| ts.clone())
            .unwrap()
    };

    let m_booting = get("booting");
    let m_loading = get("loading");
    let m_splash  = get("splash");
    let m_menu    = get("menu");
    let m_playing = get("playing");
    let m_paused  = get("paused");

    Ok(quote! {
        impl ::bevy_launchpad::core::states::mapping::LaunchpadStates
            for #enum_name
        {
            #[inline(always)]
            fn booting()  -> Self { #m_booting }
            #[inline(always)]
            fn loading()  -> Self { #m_loading }
            #[inline(always)]
            fn splash()   -> Self { #m_splash  }
            #[inline(always)]
            fn menu()     -> Self { #m_menu    }
            #[inline(always)]
            fn playing()  -> Self { #m_playing }
            #[inline(always)]
            fn paused()   -> Self { #m_paused  }
        }
    })
}

/// Returns the conventional variant name for a given method name.
/// Used in error messages to tell the developer exactly what to name their variant.
fn convention_variant_name(method: &str) -> &'static str {
    match method {
        "booting" => "Booting",
        "loading" => "Loading",
        "splash"  => "Splash",
        "menu"    => "Menu",
        "playing" => "Playing",
        "paused"  => "Paused",
        _         => method,
    }
}
```

---

## 9. Cargo.toml changes

### `bevy-launchpad/Cargo.toml` additions

```toml
[dependencies]
# ... existing deps ...
bevy_launchpad_derive = { version = "0.1", path = "./bevy_launchpad_derive" }

[workspace]
members = [
    ".",
    "bevy_launchpad_derive",  # add to workspace
    "examples/*",
]
```

---

## 10. prelude.rs changes

```rust
// src/prelude.rs — add these two lines to existing re-exports:

// Built-in state enum (Level 1)
pub use crate::core::states::app_state::AppState;

// Derive macro (Level 2 & 3)
pub use bevy_launchpad_derive::LaunchpadStates;
```

### `src/core/states/mod.rs` addition

```rust
pub mod app_state;   // add this line
pub mod machine;
pub mod mapping;
pub mod transitions;

pub use app_state::AppState;           // add
pub use mapping::LaunchpadStates;
pub use machine::StateMachine;
pub use transitions::{TransitionConfig, TransitionStateEvent, handle_state_transitions};
```

---

## 11. Error messages

The macro produces targeted, actionable errors. Examples:

### Missing mapping

```
error: #[derive(LaunchpadStates)] is missing mappings for:
         • `menu`    — add a variant named `Menu`    or annotate one with #[launchpad(menu)]
         • `paused`  — add a variant named `Paused`  or annotate one with #[launchpad(paused)]

       See bevy_launchpad documentation for examples.
  --> src/main.rs:5:10
   |
5  | #[derive(LaunchpadStates)]
   |          ^^^^^^^^^^^^^^^
```

### Invalid attribute value

```
error: `gameplay` is not a valid LaunchpadStates method.
       Valid values: booting, loading, splash, menu, playing, paused
  --> src/main.rs:12:22
   |
12 |     #[launchpad(gameplay)]
   |                 ^^^^^^^^
```

### Duplicate attribute

```
error: duplicate #[launchpad(menu)] — another variant already maps to this method
  --> src/main.rs:15:5
   |
15 |     #[launchpad(menu)]
   |     ^^^^^^^^^^^^^^^^^^
```

### Non-unit variant

```
error: #[derive(LaunchpadStates)] only supports unit enum variants
  --> src/main.rs:11:5
   |
11 |     Playing(u32),
   |     ^^^^^^^^^^^^
```

### Applied to struct

```
error: #[derive(LaunchpadStates)] can only be applied to enums
  --> src/main.rs:4:10
   |
4  | #[derive(LaunchpadStates)]
   |          ^^^^^^^^^^^^^^^
```

---

## 12. Full usage examples

### Level 1 — zero boilerplate

```rust
// Cargo.toml: bevy_launchpad = { version = "0.1" }

use bevy::prelude::*;
use bevy_launchpad::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LaunchpadPlugin::default())         // AppState is implicit
        .add_systems(OnEnter(AppState::Playing), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
```

---

### Level 2 — conventional names, 2D game

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[derive(LaunchpadStates)]
enum GameState {
    #[default]
    Booting, Loading, Splash, Menu, Playing, Paused,
    // Custom states — fully owned by the developer
    Credits,
    LevelSelect,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<GameState>::builder()
                .with_splash(SplashConfig {
                    screens: vec![
                        SplashScreenConfig::studio("branding/logo.png"),
                    ],
                    show_default_branding: false,
                    ..default()
                })
                .build(),
        )
        .add_systems(OnEnter(GameState::Playing),     setup_game)
        .add_systems(OnEnter(GameState::Credits),     setup_credits)
        .add_systems(OnEnter(GameState::LevelSelect), setup_level_select)
        .add_systems(OnEnter(GameState::GameOver),    setup_game_over)
        .run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn setup_credits(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn setup_level_select(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn setup_game_over(mut commands: Commands) {
    commands.spawn(Camera2d);
}
```

---

### Level 2 — conventional names, 3D game

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[derive(LaunchpadStates)]
enum GameState {
    #[default]
    Booting, Loading, Splash, Menu, Playing, Paused,
    Cutscene,
    BossArena,
}

#[derive(Component)]
struct RotatingCube;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LaunchpadPlugin::<GameState>::default())
        .add_systems(OnEnter(GameState::Playing), setup_3d)
        .add_systems(
            Update,
            rotate.run_if(in_state(GameState::Playing)),
        )
        .run();
}

fn setup_3d(
    mut commands:  Commands,
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
        PointLight { shadows_enabled: true, ..default() },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn rotate(mut q: Query<&mut Transform, With<RotatingCube>>, time: Res<Time>) {
    for mut t in &mut q {
        t.rotate_y(time.delta_secs());
    }
}
```

---

### Level 3 — non-conventional names, 2D game

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[derive(LaunchpadStates)]
enum MyState {
    #[default]
    #[launchpad(booting)]  Init,
    #[launchpad(loading)]  AssetsLoading,
    #[launchpad(splash)]   Intro,
    #[launchpad(menu)]     MainMenu,
    #[launchpad(playing)]  InGame,
    #[launchpad(paused)]   TimeStop,
    // Custom
    Inventory,
    Dialogue,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<MyState>::builder()
                .with_theme(ThemeConfig::dark_with_noto())
                .build(),
        )
        .add_systems(OnEnter(MyState::InGame),   setup)
        .add_systems(OnEnter(MyState::Dialogue), open_dialogue)
        .add_systems(OnEnter(MyState::GameOver), show_game_over)
        .run();
}

fn setup(mut commands: Commands)       { commands.spawn(Camera2d); }
fn open_dialogue()                     { info!("Dialogue opened"); }
fn show_game_over()                    { info!("Game over"); }
```

---

### Level 3 — non-conventional names, 3D game

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[derive(LaunchpadStates)]
enum AppFlow {
    #[default]
    #[launchpad(booting)]  Startup,
    #[launchpad(loading)]  ResourceLoad,
    #[launchpad(splash)]   CinematicIntro,
    #[launchpad(menu)]     HubWorld,
    #[launchpad(playing)]  Level,
    #[launchpad(paused)]   PauseScreen,
    // Game-specific
    Cutscene,
    VictoryScreen,
    MultiplayerLobby,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            LaunchpadPlugin::<AppFlow>::builder()
                .with_metadata(AppMetadata {
                    name:  "space_explorer".into(),
                    title: "Space Explorer".into(),
                    ..default()
                })
                .with_splash(SplashConfig {
                    screens: vec![
                        SplashScreenConfig::legal("branding/legal.png"),
                        SplashScreenConfig::studio("branding/studio.png"),
                    ],
                    show_default_branding: false,
                    ..default()
                })
                .build(),
        )
        .add_systems(OnEnter(AppFlow::Level),        setup_level)
        .add_systems(OnEnter(AppFlow::Cutscene),     start_cutscene)
        .add_systems(OnEnter(AppFlow::VictoryScreen), show_victory)
        .run();
}

fn setup_level(
    mut commands:  Commands,
    mut meshes:    ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color:           Color::srgb(0.1, 0.3, 0.9),
            emissive:             LinearRgba::new(0.0, 0.05, 0.3, 1.0),
            metallic:             0.9,
            perceptual_roughness: 0.1,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        PointLight { intensity: 3000.0, shadows_enabled: true, ..default() },
        Transform::from_xyz(8.0, 12.0, 8.0),
    ));
}

fn start_cutscene() { info!("Cutscene started"); }
fn show_victory()   { info!("Victory!"); }
```

---

### Mixed — conventional + attributed in one enum

```rust
use bevy::prelude::*;
use bevy_launchpad::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[derive(LaunchpadStates)]
enum HybridState {
    #[default]
    Booting,              // conventional → fn booting()
    Loading,              // conventional → fn loading()
    Splash,               // conventional → fn splash()

    #[launchpad(menu)]    // non-conventional → explicit
    HomeScreen,

    Playing,              // conventional → fn playing()

    #[launchpad(paused)]  // non-conventional → explicit
    Freeze,

    // Custom — ignored
    Shop,
    Inventory,
    Ending,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LaunchpadPlugin::<HybridState>::default())
        .add_systems(OnEnter(HybridState::Playing),   setup)
        .add_systems(OnEnter(HybridState::Shop),      open_shop)
        .add_systems(OnEnter(HybridState::Inventory), open_inventory)
        .run();
}

fn setup(mut commands: Commands) { commands.spawn(Camera2d); }
fn open_shop()                   { info!("Shop opened"); }
fn open_inventory()              { info!("Inventory opened"); }
```

---

## 13. Decision log

| # | Decision | Rationale |
|---|---|---|
| 1 | Proc-macro crate separate from main crate | Rust requires `proc-macro = true` crates to be separate; keeps the main crate clean |
| 2 | `AppState` as default type parameter on `LaunchpadPlugin` | Allows `LaunchpadPlugin::default()` with no angle brackets for Level 1 |
| 3 | Convention names are PascalCase matching variant names | Idiomatic Rust enum variant naming; zero surprise for Bevy developers |
| 4 | `#[launchpad(<method>)]` attribute takes method name, not variant role | More explicit, easier to validate, matches how `serde` attributes work |
| 5 | Attribute wins over convention when both match | Explicit always beats implicit; allows overriding a conventional name if needed |
| 6 | Duplicate attribute mapping is a hard compile error | Silent shadowing would be a confusing runtime bug |
| 7 | Custom variants (not in CONVENTION, no attribute) are silently ignored | Developer owns their state enum; library must not interfere |
| 8 | Only unit variants supported | Tuple/struct variants cannot be constructed without arguments; the trait returns `Self` |
| 9 | `#[inline(always)]` on generated methods | Each method is a trivial constructor — inlining is free and removes call overhead |
| 10 | Error messages list all missing methods at once | Compiler stops at first error by default; listing all missing methods saves round-trips |
| 11 | `convention_variant_name()` helper in error messages | Tells the developer exactly what to name their variant — no guessing |

---

*End of specification* — bevy_launchpad state integration v0.1
