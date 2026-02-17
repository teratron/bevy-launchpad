//! Convenient re-exports of commonly used types and traits.

// Core types
pub use crate::core::{
    assets::AssetsRootStrategy,
    boot::{AppMetadata, AppPaths, BootConfig, BootSequence, CliArgs},
    loading::{AssetManifest, AssetTracker, LoadingState},
    plugin::LaunchpadCorePlugin,
    splash::{SplashConfig, SplashScreenConfig, SplashSource, SplashTimer, SkipTrigger},
    states::{LaunchpadStates, StateMachine, TransitionConfig, TransitionStateEvent, AppState},
};

pub use bevy_launchpad_derive::LaunchpadStates;

// UI types
#[cfg(feature = "ui")]
pub use crate::ui::{
    menu::main_menu::{MainMenuConfig, MainMenuRoot, MenuButton},
    plugin::LaunchpadUiPlugin,
    splash::renderer::SplashScreen,
    theme::{ThemeColors, ThemeConfig, ThemeFonts, ThemeSpacing},
    widgets::{button::Button, slider::Slider},
};

// Locale types
#[cfg(feature = "locale")]
pub use crate::locale::{Language, LocalizationPlugin};

// Utils
pub use crate::utils::{
    persistence::{Persistable, load_ron, save_ron},
    single_instance::{
        InstanceLockGuard, SingleInstanceError, SingleInstanceLock, acquire_single_instance_lock,
    },
};

// Main Plugin
pub use crate::{LaunchpadPlugin, register_embedded_assets};

// Re-export Bevy prelude
pub use bevy::prelude::*;
