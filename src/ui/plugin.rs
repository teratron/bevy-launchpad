use crate::core::states::LaunchpadStates;
use bevy::prelude::*;
use bevy::state::prelude::in_state;
use std::marker::PhantomData;

#[derive(Resource)]
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

        app.add_systems(
            Update,
            crate::ui::menu::main_menu::handle_menu_interactions.run_if(in_state(S::menu())),
        );
        app.add_systems(
            Update,
            crate::ui::splash::renderer::update_splash_renderer.run_if(in_state(S::splash())),
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
