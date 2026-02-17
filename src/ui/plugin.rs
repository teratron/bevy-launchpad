use crate::core::states::LaunchpadStates;
use bevy::prelude::*;
use bevy::state::prelude::in_state;
use std::marker::PhantomData;

pub struct LaunchpadUiPlugin<S: LaunchpadStates> {
    _marker: PhantomData<S>,
}

impl<S: LaunchpadStates> Default for LaunchpadUiPlugin<S> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<S: LaunchpadStates> Plugin for LaunchpadUiPlugin<S> {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::ui::theme::ThemeConfig>();
        app.init_resource::<crate::ui::menu::main_menu::MainMenuConfig>();
        app.init_resource::<crate::ui::menu::settings::SettingsConfig>();

        // app.add_event::<crate::ui::menu::main_menu::CustomMenuButtonPressed>();

        app.add_systems(
            Update,
            crate::ui::menu::main_menu::handle_menu_interactions::<S>.run_if(in_state(S::menu())),
        );
        app.add_systems(
            Update,
            crate::ui::splash::renderer::update_splash_renderer::<S>.run_if(in_state(S::splash())),
        );

        app.add_systems(
            OnEnter(S::splash()),
            crate::ui::splash::renderer::setup_splash_renderer,
        );
        app.add_systems(
            OnExit(S::splash()),
            crate::ui::splash::renderer::cleanup_splash_renderer,
        );

        // Bind UI setup/cleanup to state transitions
        app.add_systems(
            OnEnter(S::menu()),
            crate::ui::menu::main_menu::setup_main_menu,
        );
        app.add_systems(
            OnExit(S::menu()),
            crate::ui::menu::main_menu::cleanup_main_menu,
        );
    }
}
