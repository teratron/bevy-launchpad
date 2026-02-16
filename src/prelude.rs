//! Convenient re-exports of commonly used types and traits.

// Core types
pub use crate::core::{
    boot::{BootConfig, BootPaths, BootSequence},
    loading::{AssetManifest, AssetTracker, LoadingState},
    plugin::LaunchpadCorePlugin,
    splash::{SplashConfig, SplashScreenConfig, SplashTimer},
    states::{StateMachine, TransitionConfig, TransitionStateEvent},
};

// UI types
#[cfg(feature = "ui")]
pub use crate::ui::{
    menu::main_menu::{MainMenuConfig, MainMenuRoot},
    plugin::LaunchpadUiPlugin,
    splash::renderer::SplashScreen,
    theme::{ThemeColors, ThemeConfig, ThemeFonts, ThemeSpacing},
    widgets::{button::Button, slider::Slider},
};

// Locale types
#[cfg(feature = "locale")]
pub use crate::locale::{Language, LocalePlugin};

// Utils
pub use crate::utils::{Persistence, SingleInstance};

// Main Plugin
pub use crate::LaunchpadPlugin;

// Re-export Bevy prelude
pub use bevy::prelude::*;
