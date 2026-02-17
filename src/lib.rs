//! Bevy Launchpad: A modular framework for Bevy 0.18.

pub mod core;
pub mod locale;
pub mod prelude;
pub mod ui;
pub mod utils;

use crate::core::assets::AssetsRootStrategy;
use crate::core::boot::cli::CliArgs;
use crate::core::boot::metadata::AppMetadata;
use crate::core::boot::paths::AppPaths;
use crate::core::splash::sequence::SplashConfig;
use crate::core::states::LaunchpadStates;
use crate::core::states::app_state::AppState;
use crate::utils::single_instance::acquire_single_instance_lock;
use bevy::prelude::*;
use std::marker::PhantomData;

pub use bevy_launchpad_derive::LaunchpadStates;

/// Main plugin that coordinates the Bevy Launchpad framework.
pub struct LaunchpadPlugin<S: LaunchpadStates = AppState> {
    _marker: PhantomData<S>,
    pub metadata: AppMetadata,
    pub cli_args: CliArgs,
    pub allow_multiple_instances: bool,
    pub assets_root: AssetsRootStrategy,
    pub splash_config: SplashConfig,

    #[cfg(feature = "ui")]
    pub theme: crate::ui::theme::ThemeConfig,
    #[cfg(feature = "ui")]
    pub main_menu: crate::ui::menu::main_menu::MainMenuConfig,

    #[cfg(feature = "locale")]
    pub default_locale: String,
}

impl<S: LaunchpadStates> Default for LaunchpadPlugin<S> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
            metadata: AppMetadata::default(),
            cli_args: CliArgs::parse_args(),
            allow_multiple_instances: false,
            assets_root: AssetsRootStrategy::default(),
            splash_config: SplashConfig::default(),
            #[cfg(feature = "ui")]
            theme: crate::ui::theme::ThemeConfig::dark(),
            #[cfg(feature = "ui")]
            main_menu: crate::ui::menu::main_menu::MainMenuConfig::default(),
            #[cfg(feature = "locale")]
            default_locale: "en-US".into(),
        }
    }
}

impl<S: LaunchpadStates> Plugin for LaunchpadPlugin<S> {
    fn build(&self, app: &mut App) {
        // 0. Register Embedded Assets
        register_embedded_assets(app);

        // 1. Path Resolution
        let paths = AppPaths::new(&self.metadata.name, &self.assets_root);
        if let Err(e) = paths.ensure_dirs() {
            error!("Failed to initialize application directories: {}", e);
        }

        // 2. Single Instance Protection
        match acquire_single_instance_lock(&paths.instance_lock_file, self.allow_multiple_instances)
        {
            Ok(Some(guard)) => {
                app.insert_non_send_resource(guard);
            }
            Ok(None) => {}
            Err(e) => {
                error!("Could not acquire instance lock: {}", e);
                #[cfg(not(test))]
                std::process::exit(1);
            }
        }

        // 3. Register Core Resources
        app.insert_resource(self.metadata.clone());
        app.insert_resource(self.cli_args.clone());
        app.insert_resource(self.splash_config.clone());
        app.insert_resource(self.assets_root.clone());
        app.insert_resource(paths);

        // 4. Add Sub-Plugins
        app.add_plugins(crate::core::plugin::LaunchpadCorePlugin::<S>::default());

        #[cfg(feature = "ui")]
        {
            app.add_plugins(crate::ui::plugin::LaunchpadUiPlugin::<S>::default());
            app.insert_resource(self.theme.clone());
            app.insert_resource(self.main_menu.clone());
        }

        #[cfg(feature = "locale")]
        {
            app.add_plugins(crate::locale::LocalizationPlugin);
            let lang = self
                .default_locale
                .parse::<crate::locale::language::Language>()
                .unwrap_or_default();
            app.insert_resource(lang);
        }
    }
}

impl<S: LaunchpadStates> LaunchpadPlugin<S> {
    pub fn builder() -> LaunchpadPluginBuilder<S> {
        LaunchpadPluginBuilder::new()
    }
}

/// Fluent builder for `LaunchpadPlugin`.
pub struct LaunchpadPluginBuilder<S: LaunchpadStates> {
    plugin: LaunchpadPlugin<S>,
}

impl<S: LaunchpadStates> Default for LaunchpadPluginBuilder<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: LaunchpadStates> LaunchpadPluginBuilder<S> {
    /// Creates a new builder with default configuration.
    pub fn new() -> Self {
        Self {
            plugin: LaunchpadPlugin::default(),
        }
    }

    pub fn with_metadata(mut self, metadata: AppMetadata) -> Self {
        self.plugin.metadata = metadata;
        self
    }

    pub fn with_cli_args(mut self, cli: CliArgs) -> Self {
        self.plugin.cli_args = cli;
        self
    }

    pub fn allow_multiple_instances(mut self, allow: bool) -> Self {
        self.plugin.allow_multiple_instances = allow;
        self
    }

    pub fn with_assets_root(mut self, strategy: AssetsRootStrategy) -> Self {
        self.plugin.assets_root = strategy;
        self
    }

    pub fn with_splash(mut self, config: SplashConfig) -> Self {
        self.plugin.splash_config = config;
        self
    }

    #[cfg(feature = "ui")]
    pub fn with_theme(mut self, theme: crate::ui::theme::ThemeConfig) -> Self {
        self.plugin.theme = theme;
        self
    }

    #[cfg(feature = "ui")]
    pub fn with_fonts(mut self, fonts: crate::ui::theme::ThemeFonts) -> Self {
        self.plugin.theme.fonts = fonts;
        self
    }

    #[cfg(feature = "ui")]
    pub fn with_main_menu(mut self, menu: crate::ui::menu::main_menu::MainMenuConfig) -> Self {
        self.plugin.main_menu = menu;
        self
    }

    #[cfg(feature = "locale")]
    pub fn with_locale(mut self, default: impl Into<String>) -> Self {
        self.plugin.default_locale = default.into();
        self
    }

    pub fn build(self) -> LaunchpadPlugin<S> {
        self.plugin
    }
}

/// Registers the library's internal assets (fonts/branding) into the Bevy app.
/// This must be called if the `embedded_assets` feature is enabled.
pub fn register_embedded_assets(app: &mut App) {
    bevy::asset::embedded_asset!(app, "assets/fonts/NotoSans-Regular.ttf");
    bevy::asset::embedded_asset!(app, "assets/fonts/NotoSans-Bold.ttf");
    bevy::asset::embedded_asset!(app, "assets/branding/default_splash.png");
}
