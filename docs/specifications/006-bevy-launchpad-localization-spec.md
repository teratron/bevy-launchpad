# Localization System Specification

**Project:** `bevy_launchpad` — reusable launcher framework for Bevy games  
**Module:** `src/locale` (Internal Module)  
**Version:** 0.2.0  
**Status:** Revised from Planetarium reference implementation  

---

## 1. Overview

The localization system provides multi-language support for Bevy games using the
[Project Fluent](https://projectfluent.org/) format. It manages language selection,
bundle loading, string resolution with fallback, and reactive UI updates when the
language changes at runtime.

This specification describes the **optimized design** implemented as an internal module
of `bevy-launchpad`, guarded by the `locale` feature flag.

---

## 2. Directory Structure

```
bevy-launchpad/
├── Cargo.toml          ← imports fluent-bundle, unic-langid (optional)
├── src/
│   ├── lib.rs          ← exports `pub mod locale;`
│   └── locale/
│       ├── mod.rs      ← public API exports
│       ├── language.rs ← Language enum and FromStr/Display
│       ├── bundle.rs   ← FluentBundleType, bundle construction helpers
│       ├── resource.rs ← Localization resource
│       ├── cache.rs    ← LocalizedStrings cache resource
│       ├── component.rs← LocalizedText component + auto-update system
│       ├── utils.rs    ← resolve_locale_dir, scan_ftl_files
│       ├── event.rs    ← LanguageChanged event
│       └── plugin.rs   ← LocalizationPlugin
└── assets/
    └── locales/
        ├── en-US/
        │   └── text/
        │       └── menu.ftl
        └── ru-RU/
            └── text/
                └── menu.ftl
```

---

## 3. Language Enum

Language is represented as a **typed enum**, not a raw `String`.

```rust
/// Supported UI languages.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Reflect)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Language {
    #[default]
    EnUs,
    RuRu,
    DeDe,
    FrFr,
    EsEs,
    JaJp,
}
```

---

## 4. Key Components

### 4.1 Localization Resource

Owned by the app, holds the active `FluentBundle` and a fallback bundle (en-US).

```rust
#[derive(Resource)]
pub struct Localization {
    pub language: Language,
    // ... bundles
}
```

### 4.2 LocalizedText Component

Marker component for entities that need auto-translation.

```rust
#[derive(Component)]
pub struct LocalizedText {
    pub key: String,
}
```

### 4.3 Plugin

The `LocalizationPlugin` initializes the system, loads bundles, and registers event listeners.

```rust
pub struct LocalizationPlugin;
```

---

## 5. Usage

Enable the `locale` feature in `Cargo.toml`:

```toml
[dependencies]
bevy-launchpad = { version = "...", features = ["locale"] }
```

### 5.1 In Game Code

```rust
fn setup(mut commands: Commands, loc: Res<Localization>) {
    commands.spawn((
        Text::new(loc.t("menu-play")),
        LocalizedText::new("menu-play"),
    ));
}
```

---

## 6. Migration from Planetarium

| Old (Planetarium) | New (bevy_launchpad) |
|---|---|
| `crates/localization` | `src/locale` (internal module) |
| `UserSettings.language: String` | `Language` Enum (in `src/locale/language.rs`) |
| `load_ftl_into_bundle("menu.ftl")` | `scan_ftl_files` (auto-discovery) |
