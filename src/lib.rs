//! Bevy Launchpad: A modular framework for Bevy 0.18.

pub mod core;
pub mod locale;
pub mod prelude;
pub mod ui;
pub mod utils;

use crate::core::boot::cli::CliArgs;
use crate::core::boot::metadata::AppMetadata;
use crate::core::boot::paths::AppPaths;
use crate::core::splash::sequence::SplashConfig;
use crate::core::states::LaunchpadStates;
use crate::utils::single_instance::acquire_single_instance_lock;
use bevy::prelude::*;
use std::marker::PhantomData;

/// Main plugin that coordinates the Bevy Launchpad framework.
pub struct LaunchpadPlugin<S: LaunchpadStates> {
    pub metadata: AppMetadata,
    pub cli: CliArgs,
    pub splash: SplashConfig,
    pub states: PhantomData<S>,
    #[cfg(feature = "ui")]
    pub theme: Option<crate::ui::theme::ThemeConfig>,
    pub allow_multiple_instances: bool,
}

impl<S: LaunchpadStates> Default for LaunchpadPlugin<S> {
    fn default() -> Self {
        Self {
            metadata: AppMetadata::default(),
            cli: CliArgs::parse_args(),
            splash: SplashConfig::default(),
            states: PhantomData,
            #[cfg(feature = "ui")]
            theme: None,
            allow_multiple_instances: false,
        }
    }
}

impl<S: LaunchpadStates> Plugin for LaunchpadPlugin<S> {
    fn build(&self, app: &mut App) {
        // 1. Path Resolution
        let paths = AppPaths::new(&self.metadata.name);
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
                // In production, we typically want to exit here.
                #[cfg(not(test))]
                std::process::exit(1);
            }
        }

        // 3. Register Core Resources
        app.insert_resource(self.metadata.clone());
        app.insert_resource(self.cli.clone());
        app.insert_resource(self.splash.clone());
        app.insert_resource(paths);

        // 4. Add Sub-Plugins
        app.add_plugins(crate::core::plugin::LaunchpadCorePlugin::<S>::default());

        #[cfg(feature = "ui")]
        {
            app.add_plugins(crate::ui::plugin::LaunchpadUiPlugin::<S>::default());
            if let Some(theme) = &self.theme {
                app.insert_resource(theme.clone());
            }
        }

        #[cfg(feature = "locale")]
        app.add_plugins(crate::locale::LocalizationPlugin);
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

    pub fn with_cli(mut self, cli: CliArgs) -> Self {
        self.plugin.cli = cli;
        self
    }

    pub fn with_splash(mut self, splash: SplashConfig) -> Self {
        self.plugin.splash = splash;
        self
    }

    #[cfg(feature = "ui")]
    pub fn with_theme(mut self, theme: crate::ui::theme::ThemeConfig) -> Self {
        self.plugin.theme = Some(theme);
        self
    }

    pub fn allow_multiple_instances(mut self, allow: bool) -> Self {
        self.plugin.allow_multiple_instances = allow;
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
