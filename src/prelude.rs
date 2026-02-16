//! Convenient re-exports of commonly used types and traits.

// Core types
pub use crate::core::{
    boot::{AppMetadata, AppPaths, BootConfig, BootSequence, CliArgs},
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
pub use crate::utils::{
    Persistable, SingleInstance, SingleInstanceError, SingleInstanceLock,
    acquire_single_instance_lock, load_ron, save_ron,
};

// Main Plugin
pub use crate::LaunchpadPlugin;

// Re-export Bevy prelude
pub use bevy::prelude::*;
