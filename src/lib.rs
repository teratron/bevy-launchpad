//! Bevy Launchpad: A modular framework for Bevy 0.18.

pub mod core;
pub mod locale;
pub mod prelude;
pub mod ui;
pub mod utils;

use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;
use std::marker::PhantomData;

/// Main plugin that combines all features.
pub struct LaunchpadPlugin<S: States + FreelyMutableState> {
    _marker: PhantomData<S>,
    #[cfg(feature = "ui")]
    pub theme: Option<crate::ui::theme::ThemeConfig>,
}

impl<S: States + FreelyMutableState> Default for LaunchpadPlugin<S> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
            #[cfg(feature = "ui")]
            theme: None,
        }
    }
}

impl<S: States + FreelyMutableState> Plugin for LaunchpadPlugin<S> {
    fn build(&self, app: &mut App) {
        // Core is mandatory
        app.add_plugins(crate::core::LaunchpadCorePlugin::<S>::default());

        #[cfg(feature = "ui")]
        {
            app.add_plugins(crate::ui::plugin::LaunchpadUiPlugin);
            if let Some(theme) = &self.theme {
                app.insert_resource(theme.clone());
            }
        }

        #[cfg(feature = "locale")]
        app.add_plugins(crate::locale::LocalePlugin);
    }
}

impl<S: States + FreelyMutableState> LaunchpadPlugin<S> {
    pub fn builder() -> LaunchpadPluginBuilder<S> {
        LaunchpadPluginBuilder {
            plugin: Self::default(),
        }
    }
}

pub struct LaunchpadPluginBuilder<S: States + FreelyMutableState> {
    plugin: LaunchpadPlugin<S>,
}

impl<S: States + FreelyMutableState> LaunchpadPluginBuilder<S> {
    #[cfg(feature = "ui")]
    pub fn with_theme(mut self, theme: crate::ui::theme::ThemeConfig) -> Self {
        self.plugin.theme = Some(theme);
        self
    }

    pub fn build(self) -> LaunchpadPlugin<S> {
        self.plugin
    }
}
