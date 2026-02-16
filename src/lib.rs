//! Bevy Launchpad: A modular framework for Bevy 0.18.

pub mod core;
pub mod locale;
pub mod prelude;
pub mod ui;
pub mod utils;

use crate::core::states::LaunchpadStates;
use bevy::prelude::*;
use std::marker::PhantomData;

/// Main plugin that combines all features.
pub struct LaunchpadPlugin<S: LaunchpadStates> {
    _marker: PhantomData<S>,
    pub metadata: crate::prelude::AppMetadata,
    pub cli_args: crate::prelude::CliArgs,
    pub allow_multiple_instances: bool,
    #[cfg(feature = "ui")]
    pub theme: Option<crate::ui::theme::ThemeConfig>,
}

impl<S: LaunchpadStates> Default for LaunchpadPlugin<S> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
            metadata: crate::prelude::AppMetadata::default(),
            cli_args: crate::prelude::CliArgs::parse_args(),
            allow_multiple_instances: false,
            #[cfg(feature = "ui")]
            theme: None,
        }
    }
}

impl<S: LaunchpadStates> Plugin for LaunchpadPlugin<S> {
    fn build(&self, app: &mut App) {
        // 1. Setup paths based on metadata
        let paths = crate::prelude::AppPaths::new(&self.metadata.name);
        if let Err(e) = paths.ensure_dirs() {
            error!("Failed to create data directory: {}", e);
        }

        // 2. Protect against multiple instances
        let lock = match crate::utils::single_instance::acquire_single_instance_lock(
            &paths.instance_lock_file,
            self.allow_multiple_instances,
        ) {
            Ok(lock) => lock,
            Err(e) => {
                // If it's already running, we might want to panic or handle it gracefully.
                // For a framework, panicking with a clear message is often the safest startup path
                // unless the user provided an error handler.
                panic!("Single instance protection: {}", e);
            }
        };

        if let Some(guard) = lock {
            // Holds the lock alive for the app's lifetime.
            app.insert_non_send_resource(guard);
        }

        // 3. Register resources
        app.insert_resource(self.metadata.clone());
        app.insert_resource(paths);
        app.insert_resource(self.cli_args.clone());

        // 4. Core is mandatory
        app.add_plugins(crate::core::LaunchpadCorePlugin::<S>::default());

        #[cfg(feature = "ui")]
        {
            app.add_plugins(crate::ui::plugin::LaunchpadUiPlugin::<S>::default());
            if let Some(theme) = &self.theme {
                app.insert_resource(theme.clone());
            }
        }

        #[cfg(feature = "locale")]
        app.add_plugins(crate::locale::LocalePlugin);
    }
}

impl<S: LaunchpadStates> LaunchpadPlugin<S> {
    pub fn builder() -> LaunchpadPluginBuilder<S> {
        LaunchpadPluginBuilder {
            plugin: Self::default(),
        }
    }
}

pub struct LaunchpadPluginBuilder<S: LaunchpadStates> {
    plugin: LaunchpadPlugin<S>,
}

impl<S: LaunchpadStates> LaunchpadPluginBuilder<S> {
    pub fn with_metadata(mut self, metadata: crate::prelude::AppMetadata) -> Self {
        self.plugin.metadata = metadata;
        self
    }

    pub fn with_cli_args(mut self, args: crate::prelude::CliArgs) -> Self {
        self.plugin.cli_args = args;
        self
    }

    pub fn allow_multiple_instances(mut self, allow: bool) -> Self {
        self.plugin.allow_multiple_instances = allow;
        self
    }

    #[cfg(feature = "ui")]
    pub fn with_theme(mut self, theme: crate::ui::theme::ThemeConfig) -> Self {
        self.plugin.theme = Some(theme);
        self
    }

    pub fn build(self) -> LaunchpadPlugin<S> {
        self.plugin
    }
}
